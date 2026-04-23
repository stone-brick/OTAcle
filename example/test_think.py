"""
OTAcle Think 模块工具方法测试脚本

测试 otacle.py 中 OTAcleThinkSender 的各项功能：
- DecisionLog 数据类
- 发送决策日志
- 统计数据

Usage:
    python test_think.py
"""

import sys
import os

# 添加 python 目录到路径以便导入 otacle
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "python"))

from otacle import (
    OTAcleThinkSender,
    DecisionLog,
    send_decision_log,
    ZMQ_THINK_ADDR,
)


def test_decision_log():
    """测试 DecisionLog 数据类"""
    print("\n=== 测试 DecisionLog ===")

    # 测试创建
    log = DecisionLog(step=100, custom={"reward": 1.0, "epsilon": 0.1})
    assert log.step == 100
    assert log.custom["reward"] == 1.0
    assert log.custom["epsilon"] == 0.1
    print(f"  创建: OK (step={log.step}, custom={log.custom})")

    # 测试 to_dict
    d = log.to_dict()
    assert d["step"] == 100
    assert d["custom"]["reward"] == 1.0
    print(f"  to_dict: OK ({d})")

    # 测试 to_json
    j = log.to_json()
    assert '"step": 100' in j
    assert '"reward": 1.0' in j
    print(f"  to_json: OK ({j})")

    # 测试 from_dict
    log2 = DecisionLog.from_dict({"step": 200, "custom": {"loss": 0.5}})
    assert log2.step == 200
    assert log2.custom["loss"] == 0.5
    print(f"  from_dict: OK (step={log2.step})")


def test_sender_basic():
    """测试 OTAcleThinkSender 基本 API"""
    print("\n=== 测试 OTAcleThinkSender 基本 API ===")

    sender = OTAcleThinkSender()

    # 测试属性初始状态
    assert sender.sent_count == 0
    print(f"  初始状态: sent_count=0")

    # 测试 repr
    repr_str = repr(sender)
    assert "OTAcleThinkSender" in repr_str
    print(f"  repr: {repr_str}")

    sender.stop()
    print("  基本 API 测试完成")


def test_sender_context_manager():
    """测试 OTAcleThinkSender 上下文管理器"""
    print("\n=== 测试 OTAcleThinkSender 上下文管理器 ===")

    with OTAcleThinkSender() as sender:
        assert sender._sock is not None
        print(f"  进入上下文: socket 已创建")
        print(f"  repr: {repr(sender)}")

    print("  退出上下文: 已自动 stop()")
    print("  上下文管理器测试完成")


def test_send_decision_log():
    """测试发送决策日志"""
    print("\n=== 测试发送决策日志 ===")
    print(f"  目标地址: {ZMQ_THINK_ADDR}")
    print("  提示: 需要 Rust 后端 Think 模块运行中")

    try:
        with OTAcleThinkSender() as sender:
            # 发送几条测试数据
            sender.send_step(1, reward=1.0, epsilon=0.9, q_values={"a": 0.5, "b": 0.3})
            print(f"  [1] 发送成功: reward=1.0, epsilon=0.9")

            sender.send_step(2, reward=0.8, epsilon=0.8)
            print(f"  [2] 发送成功: reward=0.8, epsilon=0.8")

            sender.send_step(3, reward=0.6, epsilon=0.7, loss=0.2)
            print(f"  [3] 发送成功: reward=0.6, epsilon=0.7, loss=0.2")

            print(f"  总发送: {sender.sent_count} 条")

    except Exception as e:
        print(f"  发送失败: {e}")
        print("  (这是正常的，如果 Rust 后端未运行)")


def test_convenience_function():
    """测试便捷函数"""
    print("\n=== 测试便捷函数 ===")

    try:
        send_decision_log(100, {"reward": 2.0, "epsilon": 0.5})
        print("  send_decision_log(100, ...) 发送成功")
    except Exception as e:
        print(f"  send_decision_log 失败: {e}")
        print("  (这是正常的，如果 Rust 后端未运行)")


def test_complex_custom_data():
    """测试复杂的自定义数据"""
    print("\n=== 测试复杂自定义数据 ===")

    try:
        with OTAcleThinkSender() as sender:
            # 测试嵌套字典
            log = DecisionLog(
                step=1000,
                custom={
                    "q_values": {"action_0": 0.1, "action_1": 0.9, "action_2": 0.0},
                    "reward": 1.5,
                    "episode": 5,
                    "metadata": {"source": "test", "version": 1},
                }
            )
            sender.send(log)
            print(f"  发送复杂数据成功: step={log.step}")
            print(f"  JSON: {log.to_json()}")

    except Exception as e:
        print(f"  发送失败: {e}")


def main():
    print("=" * 60)
    print("OTAcle Think 模块工具方法测试")
    print("=" * 60)

    # 数据类测试（不依赖网络）
    test_decision_log()
    test_sender_basic()
    test_sender_context_manager()

    # 网络测试（需要 Rust 后端运行）
    print("\n" + "=" * 60)
    print("网络测试（需要 Rust 后端 Think 模块运行）")
    print("=" * 60)

    test_send_decision_log()
    test_convenience_function()
    test_complex_custom_data()

    print("\n" + "=" * 60)
    print("所有测试完成")
    print("=" * 60)


if __name__ == "__main__":
    main()
