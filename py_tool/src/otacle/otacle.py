"""
OTAcle Python ZMQ 命令辅助模块

用于与 OTAcle Rust 后端通信：
- OTAcleCommand: 发送动作命令（ZMQ PUSH to Act）
- OTAcleObserver: 接收图像帧数据（ZMQ SUB from Observe）
- OTAcleThinkSender: 发送决策日志（ZMQ PUSH to Think）

Usage:
    # 发送动作命令
    with OTAcleCommand() as cmd:
        cmd.execute([0, 2])                      # 执行 index 0 和 2
        cmd.set_params({"target_x": 100, "target_y": 200})
        cmd.send()

    # 接收图像帧
    with OTAcleObserver() as observer:
        for frame in observer:
            print(f"帧 {frame.frame_id}: {frame.width}x{frame.height}")
            # frame.data 是裁切块列表

    # 发送决策日志
    with OTAcleThinkSender() as sender:
        sender.send_step(1, reward=1.0, epsilon=0.1)
        sender.send_step(2, reward=0.5, epsilon=0.05)
"""

import atexit
import base64
import json
import time
import zmq
from dataclasses import dataclass
from typing import Optional, Callable, Iterator


# =============================================================================
# 常量
# =============================================================================

ZMQ_ACT_ADDR = "tcp://127.0.0.1:5555"  # Act 模块 PUSH 地址
ZMQ_OBSERVE_ADDR = "tcp://127.0.0.1:5556"  # Observe 模块 PUB 地址
ZMQ_THINK_ADDR = "tcp://127.0.0.1:5557"  # Think 模块 PULL 地址
DEFAULT_ACTION_COUNT = 10  # 默认动作数组长度


# =============================================================================
# 配置管理
# =============================================================================

import os
from pathlib import Path

DEFAULT_OTACLE_DIR = ".otacle"
DEFAULT_COMM_CONFIG = "comm.json"


def find_project_root(start_path: Optional[str] = None) -> Optional[Path]:
    """
    向上搜索项目根目录（包含 .otacle 目录）

    Args:
        start_path: 起始搜索路径，默认为当前工作目录

    Returns:
        项目根目录 Path 或 None
    """
    cwd = Path(start_path) if start_path else Path.cwd()
    current = cwd.resolve()

    # 向上搜索最多 10 层
    for _ in range(10):
        if (current / DEFAULT_OTACLE_DIR).is_dir():
            return current
        parent = current.parent
        if parent == current:
            break
        current = parent
    return None


def load_comm_config(project_root: Optional[str] = None) -> dict:
    """
    从 .otacle/comm.json 加载通信配置

    Args:
        project_root: 项目根目录，默认为自动搜索

    Returns:
        通信配置字典，包含:
        - act_pull_address: Act 模块 PULL 地址
        - observe_pub_address: Observe 模块 PUB 地址
        - think_pull_address: Think 模块 PULL 地址
        若文件不存在或读取失败，返回空字典
    """
    root = Path(project_root) if project_root else find_project_root()
    if not root:
        return {}

    config_path = root / DEFAULT_OTACLE_DIR / DEFAULT_COMM_CONFIG
    if not config_path.exists():
        return {}

    try:
        with open(config_path, "r", encoding="utf-8") as f:
            return json.load(f)
    except (json.JSONDecodeError, IOError):
        return {}


def get_default_addresses() -> tuple[str, str, str]:
    """
    获取默认通信地址，优先从配置文件读取

    Returns:
        (act_addr, observe_addr, think_addr) 元组
    """
    config = load_comm_config()
    return (
        config.get("act_pull_address", ZMQ_ACT_ADDR),
        config.get("observe_pub_address", ZMQ_OBSERVE_ADDR),
        config.get("think_pull_address", ZMQ_THINK_ADDR),
    )


# =============================================================================
# 数据类
# =============================================================================

@dataclass
class CropBlock:
    """裁切块"""
    x: int
    y: int
    w: int
    h: int
    image: str  # Base64 编码的图像

    @classmethod
    def from_dict(cls, data: dict) -> "CropBlock":
        return cls(
            x=data.get("x", 0),
            y=data.get("y", 0),
            w=data.get("w", 0),
            h=data.get("h", 0),
            image=data.get("image", ""),
        )

    def decode_image(self) -> bytes:
        """解码 Base64 图像数据"""
        return base64.b64decode(self.image)


class FrameType:
    """帧类型"""
    NORMAL = "Normal"
    STOP = "Stop"


@dataclass
class FrameMessage:
    """图像帧消息"""
    width: int
    height: int
    timestamp: int
    frame_id: int
    data: list[CropBlock]
    frame_type: str = "Normal"  # 帧类型：Normal 或 Stop

    @classmethod
    def from_dict(cls, data: dict) -> "FrameMessage":
        return cls(
            width=data.get("width", 0),
            height=data.get("height", 0),
            timestamp=data.get("timestamp", 0),
            frame_id=data.get("frame_id", 0),
            data=[CropBlock.from_dict(b) for b in data.get("data", [])],
            frame_type=data.get("frame_type", FrameType.NORMAL),
        )


@dataclass
class DecisionLog:
    """决策日志（用于 Think 模块）"""
    step: int
    custom: dict

    def to_dict(self) -> dict:
        """转换为字典"""
        return {
            "step": self.step,
            "custom": self.custom,
        }

    def to_json(self) -> str:
        """转换为 JSON 字符串"""
        return json.dumps(self.to_dict(), ensure_ascii=False)

    @classmethod
    def from_dict(cls, data: dict) -> "DecisionLog":
        return cls(
            step=data.get("step", 0),
            custom=data.get("custom", {}),
        )


# =============================================================================
# Act 模块 - 发送动作命令
# =============================================================================

class OTAcleCommand:
    """构建 OTAcle ZMQ 命令消息"""

    def __init__(self, action_count: int = DEFAULT_ACTION_COUNT, address: Optional[str] = None):
        """
        初始化命令构建器

        Args:
            action_count: 动作数组长度，默认为 10
            address: ZMQ 地址，默认为从 .otacle/comm.json 读取或硬编码默认值
        """
        if address is None:
            address = get_default_addresses()[0]
        self._address = address
        self._execute: list[bool] = [False] * action_count
        self._params: dict = {}
        self._ctx: Optional[zmq.Context] = None
        self._sock: Optional[zmq.Socket] = None
        atexit.register(self.close)

    def _ensure_connected(self) -> None:
        """确保 ZMQ 连接已建立（复用已有连接）"""
        if self._sock is not None:
            return
        self._ctx = zmq.Context()
        self._sock = self._ctx.socket(zmq.PUSH)
        self._sock.connect(self._address)

    def close(self) -> None:
        """关闭 ZMQ 连接"""
        if self._sock:
            self._sock.close()
            self._sock = None
        if self._ctx:
            self._ctx.term()
            self._ctx = None

    def set_action_count(self, count: int) -> "OTAcleCommand":
        """设置动作数组长度"""
        self._execute = [False] * count
        return self

    def set_action(self, index: int, enabled: bool = True) -> "OTAcleCommand":
        """
        设置指定索引的动作是否执行

        Args:
            index: 动作索引
            enabled: 是否执行，默认为 True
        """
        if 0 <= index < len(self._execute):
            self._execute[index] = enabled
        return self

    def execute(self, indices: list[int]) -> "OTAcleCommand":
        """
        按索引指定要执行的动作（追加式）

        Args:
            indices: 动作索引列表，如 [0, 2, 4]
        """
        for idx in indices:
            if 0 <= idx < len(self._execute):
                self._execute[idx] = True
        return self

    def execute_all(self) -> "OTAcleCommand":
        """执行所有动作"""
        self._execute = [True] * len(self._execute)
        return self

    def clear_execute(self) -> "OTAcleCommand":
        """清除所有执行标记"""
        self._execute = [False] * len(self._execute)
        return self

    def set_params(self, params: dict) -> "OTAcleCommand":
        """
        设置动态参数

        Args:
            params: 参数字典，键名对应动作配置中 variables 里的 param_name。
                   例如 variables 为 [{"param_name": "target_x", "field_name": "x"}] 时，
                   应传入 {"target_x": 100}。
        """
        self._params = params
        return self

    def add_param(self, key: str, value) -> "OTAcleCommand":
        """
        添加或更新单个参数

        Args:
            key: 参数名（对应 variables 中的 param_name）
            value: 参数值（支持 int, float, str）
        """
        self._params[key] = value
        return self

    def remove_param(self, key: str) -> "OTAcleCommand":
        """
        删除指定参数

        Args:
            key: 参数名（对应 variables 中的 param_name）
        """
        self._params.pop(key, None)
        return self

    def build(self) -> dict:
        """
        构建命令消息字典

        Returns:
            命令消息字典
        """
        return {
            "execute": self._execute,
            "params": self._params if self._params else {},
        }

    def to_json(self) -> str:
        """返回 JSON 格式的命令字符串"""
        return json.dumps(self.build(), ensure_ascii=False)

    def send(self) -> None:
        """发送命令到 ZMQ 地址（连接会被复用）"""
        self._ensure_connected()
        self._sock.send_string(self.to_json())

    def __repr__(self) -> str:
        return f"OTAcleCommand(execute={self._execute}, params={self._params})"

    def __del__(self) -> None:
        """析构时确保连接关闭"""
        self.close()

    def __enter__(self) -> "OTAcleCommand":
        return self

    def __exit__(self, *args) -> None:
        self.close()


def send_command(
    execute_indices: list[int],
    params: Optional[dict] = None,
    address: Optional[str] = None,
) -> None:
    """
    便捷函数：发送简单命令

    Args:
        execute_indices: 要执行的动作索引列表
        params: 动态参数字典，键名对应 variables 中的 param_name
        address: ZMQ 地址，默认为从配置文件读取
    """
    cmd = OTAcleCommand(address=address)
    cmd.execute(execute_indices)
    if params:
        cmd.set_params(params)
    cmd.send()
    cmd.close()


# =============================================================================
# Observe 模块 - 接收图像帧
# =============================================================================

class OTAcleObserver:
    """接收 OTAcle Observe 模块的图像帧（ZMQ SUB）"""

    def __init__(self, address: Optional[str] = None):
        """
        初始化图像帧接收器

        Args:
            address: ZMQ SUB 地址，默认为从 .otacle/comm.json 读取或硬编码默认值
        """
        if address is None:
            address = get_default_addresses()[1]
        self._address = address
        self._ctx: Optional[zmq.Context] = None
        self._sock: Optional[zmq.Socket] = None
        self._running = False
        self._frame_count = 0
        self._bytes_received = 0
        self._start_time: Optional[float] = None

    def start(self) -> "OTAcleObserver":
        """启动 ZMQ SUB 连接"""
        if self._running:
            return self

        self._ctx = zmq.Context()
        self._sock = self._ctx.socket(zmq.SUB)
        self._sock.connect(self._address)
        self._sock.setsockopt_string(zmq.SUBSCRIBE, "")  # 订阅所有消息
        self._running = True
        self._start_time = time.time()
        return self

    def stop(self) -> None:
        """停止 ZMQ SUB 连接"""
        self._running = False
        if self._sock:
            self._sock.close()
            self._sock = None
        if self._ctx:
            self._ctx.term()
            self._ctx = None

    def recv(self, timeout: int = 5000) -> Optional[FrameMessage]:
        """
        接收一帧图像数据

        Args:
            timeout: 接收超时时间（毫秒）

        Returns:
            FrameMessage 对象，或超时返回 None
        """
        if not self._running or not self._sock:
            raise RuntimeError("Observer not started. Call start() first.")

        if self._sock.poll(timeout=timeout):
            msg = self._sock.recv_string()
            self._bytes_received += len(msg.encode("utf-8"))
            self._frame_count += 1
            data = json.loads(msg)
            return FrameMessage.from_dict(data)
        return None

    def __iter__(self) -> Iterator[FrameMessage]:
        """迭代器接口，持续接收帧直到 stop()"""
        if not self._running:
            self.start()
        return self

    def __next__(self) -> FrameMessage:
        """迭代器下一个元素，阻塞等待"""
        frame = self.recv(timeout=5000)
        if frame is None:
            raise StopIteration
        # 检查是否为停止消息
        if frame.frame_type == FrameType.STOP:
            raise StopIteration
        return frame

    @property
    def frame_count(self) -> int:
        """总接收帧数"""
        return self._frame_count

    @property
    def bytes_received(self) -> int:
        """总接收字节数"""
        return self._bytes_received

    @property
    def elapsed_time(self) -> float:
        """运行时间（秒）"""
        if self._start_time is None:
            return 0.0
        return time.time() - self._start_time

    @property
    def fps(self) -> float:
        """当前帧率"""
        elapsed = self.elapsed_time
        if elapsed > 0:
            return self._frame_count / elapsed
        return 0.0

    def stats(self) -> dict:
        """返回统计信息字典"""
        return {
            "frame_count": self._frame_count,
            "bytes_received": self._bytes_received,
            "elapsed_time": self.elapsed_time,
            "fps": self.fps,
        }

    def __repr__(self) -> str:
        return f"OTAcleObserver(addr={self._address}, frames={self._frame_count}, fps={self.fps:.1f})"

    def __del__(self) -> None:
        """析构时确保连接关闭"""
        self.stop()

    def __enter__(self) -> "OTAcleObserver":
        return self.start()

    def __exit__(self, *args) -> None:
        self.stop()


def observe_frames(
    address: Optional[str] = None,
    callback: Optional[Callable[[FrameMessage], None]] = None,
) -> None:
    """
    便捷函数：持续接收图像帧直到中断

    Args:
        address: ZMQ SUB 地址
        callback: 可选的回调函数，接收每个 FrameMessage
    """
    with OTAcleObserver(address) as observer:
        print(f"[*] 监听 {observer._address}，等待帧数据...")
        print("按 Ctrl+C 退出\n")

        try:
            for frame in observer:
                elapsed = observer.elapsed_time
                fps = observer.fps

                print(
                    f"[{frame.frame_id:06d}] {frame.width}x{frame.height} | "
                    f"blocks={len(frame.data)} | fps={fps:.1f}"
                )

                if callback:
                    callback(frame)
        except KeyboardInterrupt:
            print("\n[*] 收到中断信号，退出...")

        # 打印最终统计
        stats = observer.stats()
        print(f"\n[*] 统计信息:")
        print(f"    总帧数: {stats['frame_count']}")
        print(f"    总字节: {stats['bytes_received']}")
        print(f"    运行时间: {stats['elapsed_time']:.1f}s")
        print(f"    平均帧率: {stats['fps']:.1f} fps")


# =============================================================================
# Think 模块 - 发送决策日志
# =============================================================================

class OTAcleThinkSender:
    """发送决策日志到 Think 模块（ZMQ PUSH）"""

    def __init__(self, address: Optional[str] = None):
        """
        初始化决策日志发送器

        Args:
            address: ZMQ PUSH 地址，默认为从 .otacle/comm.json 读取或硬编码默认值
        """
        if address is None:
            address = get_default_addresses()[2]
        self._address = address
        self._ctx: Optional[zmq.Context] = None
        self._sock: Optional[zmq.Socket] = None
        self._sent_count = 0

    def start(self) -> "OTAcleThinkSender":
        """启动 ZMQ PUSH 连接"""
        self._ctx = zmq.Context()
        self._sock = self._ctx.socket(zmq.PUSH)
        self._sock.connect(self._address)
        return self

    def stop(self) -> None:
        """停止 ZMQ 连接"""
        if self._sock:
            self._sock.close()
            self._sock = None
        if self._ctx:
            self._ctx.term()
            self._ctx = None

    def send(self, log: DecisionLog) -> None:
        """
        发送单个决策日志

        Args:
            log: DecisionLog 对象
        """
        if not self._sock:
            raise RuntimeError("Sender not started. Call start() first.")
        self._sock.send_string(log.to_json())
        self._sent_count += 1

    def send_step(self, step: int, **kwargs) -> None:
        """
        便捷方法：直接发送 step 和自定义字段

        Args:
            step: 训练步数
            **kwargs: 自定义字段，如 reward=1.0, epsilon=0.1
        """
        self.send(DecisionLog(step=step, custom=kwargs))

    @property
    def sent_count(self) -> int:
        """总发送条数"""
        return self._sent_count

    def __repr__(self) -> str:
        return f"OTAcleThinkSender(addr={self._address}, sent={self._sent_count})"

    def __del__(self) -> None:
        """析构时确保连接关闭"""
        self.stop()

    def __enter__(self) -> "OTAcleThinkSender":
        return self.start()

    def __exit__(self, *args) -> None:
        self.stop()


def send_decision_log(step: int, custom: Optional[dict] = None) -> None:
    """
    便捷函数：发送单个决策日志

    Args:
        step: 训练步数
        custom: 自定义字段字典
    """
    with OTAcleThinkSender() as sender:
        sender.send_step(step, **(custom or {}))
