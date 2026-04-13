"""
OTAcle Python ZMQ 命令辅助模块

用于构建和发送命令到 OTAcle Rust 后端。

Usage:
    # 方式1: 上下文管理器（推荐）
    with OTAcleCommand() as cmd:
        cmd.execute([0, 2])                      # 执行 index 0 和 2
        cmd.set_params({"target_x": 100, "target_y": 200})
        cmd.send()

    # 方式2: 显式 close()
    cmd = OTAcleCommand()
    cmd.execute([0])
    cmd.send()
    cmd.close()

    # 便捷函数
    from otacle import send_command
    send_command([0, 2], {"target_x": 100})
"""

import atexit
import json
import time
import zmq
from typing import Optional


ZMQ_ADDR = "tcp://127.0.0.1:5555"
DEFAULT_ACTION_COUNT = 10  # 默认动作数组长度


class OTAcleCommand:
    """构建 OTAcle ZMQ 命令消息"""

    def __init__(self, action_count: int = DEFAULT_ACTION_COUNT, address: Optional[str] = None):
        """
        初始化命令构建器

        Args:
            action_count: 动作数组长度，默认为 10
            address: ZMQ 地址，默认为 tcp://127.0.0.1:5555
        """
        self._address = address or ZMQ_ADDR
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
    address: str = ZMQ_ADDR,
) -> None:
    """
    便捷函数：发送简单命令

    Args:
        execute_indices: 要执行的动作索引列表
        params: 动态参数字典，键名对应 variables 中的 param_name
        address: ZMQ 地址
    """
    cmd = OTAcleCommand(address=address)
    cmd.execute(execute_indices)
    if params:
        cmd.set_params(params)
    cmd.send()
    cmd.close()
