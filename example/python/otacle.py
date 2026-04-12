"""
OTAcle Python ZMQ 命令辅助模块

用于构建和发送命令到 OTAcle Rust 后端。

Usage:
    # 方式1: 按索引执行动作
    from otacle import OTAcleCommand
    cmd = OTAcleCommand()
    cmd.execute_by_indices([0, 2])           # 执行 index 0 和 2
    cmd.set_params({"x": 100, "y": 200})
    cmd.send()

    # 方式2: 按名称执行动作（需先加载 actions 配置）
    from otacle import OTAcleCommand
    cmd = OTAcleCommand()
    cmd.load_actions("path/to/actions.json")
    cmd.execute_by_names(["jump", "move_mouse"])
    cmd.set_params({"x": 100, "y": 200})
    cmd.send()
"""

import json
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
        self._actions_config: Optional[dict] = None

    def set_action_count(self, count: int) -> "OTAleCommand":
        """设置动作数组长度"""
        self._execute = [False] * count
        return self

    def execute_by_indices(self, indices: list[int]) -> "OTAleCommand":
        """
        按索引指定要执行的动作

        Args:
            indices: 动作索引列表，如 [0, 2, 4]
        """
        for idx in indices:
            if 0 <= idx < len(self._execute):
                self._execute[idx] = True
        return self

    def execute_all(self) -> "OTAleCommand":
        """执行所有动作"""
        self._execute = [True] * len(self._execute)
        return self

    def clear_execute(self) -> "OTAleCommand":
        """清除所有执行标记"""
        self._execute = [False] * len(self._execute)
        return self

    def load_actions(self, config_path: str) -> "OTAleCommand":
        """
        从 JSON 文件加载动作配置

        Args:
            config_path: actions.json 文件路径
        """
        with open(config_path, "r", encoding="utf-8") as f:
            self._actions_config = json.load(f)
        # 根据配置设置动作数组长度
        if "actions" in self._actions_config:
            self._execute = [False] * len(self._actions_config["actions"])
        return self

    def execute_by_names(self, names: list[str]) -> "OTAleCommand":
        """
        按名称指定要执行的动作（需先调用 load_actions）

        Args:
            names: 动作名称列表

        Raises:
            RuntimeError: 如果未先加载动作配置
        """
        if self._actions_config is None:
            raise RuntimeError(
                "必须先调用 load_actions() 加载动作配置才能使用 execute_by_names()"
            )

        name_to_index = {}
        for action in self._actions_config.get("actions", []):
            if "name" in action:
                name_to_index[action["name"]] = action["index"]

        for name in names:
            if name in name_to_index:
                idx = name_to_index[name]
                if 0 <= idx < len(self._execute):
                    self._execute[idx] = True
            else:
                raise ValueError(f"未找到动作名称: {name}")

        return self

    def set_params(self, params: dict) -> "OTAleCommand":
        """
        设置占位符参数

        Args:
            params: 参数字典，如 {"x": 100, "y": 200}
        """
        self._params = params
        return self

    def add_param(self, key: str, value) -> "OTAleCommand":
        """
        添加单个参数

        Args:
            key: 参数名
            value: 参数值（支持 int, float, str）
        """
        self._params[key] = value
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

    def send(self, address: Optional[str] = None) -> None:
        """
        发送命令到 ZMQ 地址

        Args:
            address: 可选的 ZMQ 地址，如果为 None 则使用初始化时的地址
        """
        addr = address or self._address
        ctx = zmq.Context()
        sock = ctx.socket(zmq.PUB)
        sock.connect(addr)
        try:
            sock.send_string(self.to_json())
        finally:
            ctx.term()

    def __repr__(self) -> str:
        return f"OTAleCommand(execute={self._execute}, params={self._params})"


def send_command(
    execute_indices: list[int],
    params: Optional[dict] = None,
    address: str = ZMQ_ADDR,
) -> None:
    """
    便捷函数：发送简单命令

    Args:
        execute_indices: 要执行的动作索引列表
        params: 占位符参数字典
        address: ZMQ 地址
    """
    cmd = OTAcleCommand()
    cmd.execute_by_indices(execute_indices)
    if params:
        cmd.set_params(params)
    cmd.send(address)


if __name__ == "__main__":
    # 简单的自测代码
    cmd = OTAcleCommand()
    cmd.execute_by_indices([0, 2, 4])
    cmd.set_params({"x": 100, "y": 200, "name": "test"})
    print("测试命令:")
    print(cmd.to_json())
    print(f"\n地址: {ZMQ_ADDR}")
