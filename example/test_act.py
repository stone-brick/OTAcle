"""
OTAcle Act 模块工具方法测试脚本

测试 otacle.py 中 OTAcleCommand 的各项功能：
- 动作命令构建
- 参数设置
- 消息发送

Usage:
    python test_act.py
"""

import sys
import os

# 导入 otacle 包（通过 pip install -e py_tool/ 安装）
from otacle import OTAcleCommand, send_command, ZMQ_ACT_ADDR


def test_command_creation():
    """测试 OTAcleCommand 创建"""
    print("\n=== 测试 OTAcleCommand 创建 ===")

    cmd = OTAcleCommand()
    assert cmd._execute == [False] * 10
    assert cmd._params == {}
    assert cmd._address == ZMQ_ACT_ADDR
    print(f"  默认创建: OK (action_count=10, addr={cmd._address})")

    cmd2 = OTAcleCommand(action_count=5)
    assert len(cmd2._execute) == 5
    print(f"  自定义 action_count: OK (action_count=5)")

    cmd.close()


def test_execute_actions():
    """测试动作执行设置"""
    print("\n=== 测试动作执行设置 ===")

    cmd = OTAcleCommand()

    # 测试 set_action
    cmd.set_action(0)
    assert cmd._execute[0] == True
    print(f"  set_action(0): OK")

    cmd.set_action(2, enabled=False)
    assert cmd._execute[2] == False
    print(f"  set_action(2, enabled=False): OK")

    # 测试 execute (追加式)
    cmd.clear_execute()  # 先清空
    cmd.execute([1, 3, 5])
    assert cmd._execute == [False, True, False, True, False, True, False, False, False, False]
    print(f"  execute([1, 3, 5]): OK")

    # 测试 execute_all
    cmd.execute_all()
    assert all(cmd._execute)
    print(f"  execute_all(): OK")

    # 测试 clear_execute
    cmd.clear_execute()
    assert not any(cmd._execute)
    print(f"  clear_execute(): OK")

    cmd.close()


def test_params():
    """测试参数设置"""
    print("\n=== 测试参数设置 ===")

    cmd = OTAcleCommand()

    # 测试 set_params
    cmd.set_params({"target_x": 100, "target_y": 200})
    assert cmd._params == {"target_x": 100, "target_y": 200}
    print(f"  set_params: OK ({cmd._params})")

    # 测试 add_param
    cmd.add_param("speed", 1.5)
    assert cmd._params["speed"] == 1.5
    print(f"  add_param: OK ({cmd._params})")

    # 测试 remove_param
    cmd.remove_param("target_x")
    assert "target_x" not in cmd._params
    print(f"  remove_param: OK")

    cmd.close()


def test_build_message():
    """测试消息构建"""
    print("\n=== 测试消息构建 ===")

    cmd = OTAcleCommand()
    cmd.execute([0, 2])
    cmd.set_params({"x": 100})

    msg = cmd.build()
    assert msg["execute"] == [True, False, True, False, False, False, False, False, False, False]
    assert msg["params"] == {"x": 100}
    print(f"  build: OK")
    print(f"    execute: {msg['execute']}")
    print(f"    params: {msg['params']}")

    json_str = cmd.to_json()
    assert '"execute"' in json_str
    assert '"params"' in json_str
    print(f"  to_json: OK ({json_str})")

    cmd.close()


def test_repr():
    """测试 repr"""
    print("\n=== 测试 repr ===")

    cmd = OTAcleCommand()
    cmd.execute([1])
    cmd.set_params({"a": 1})

    r = repr(cmd)
    assert "OTAcleCommand" in r
    assert "execute=" in r
    print(f"  repr: {r}")

    cmd.close()


def test_context_manager():
    """测试上下文管理器"""
    print("\n=== 测试上下文管理器 ===")

    with OTAcleCommand() as cmd:
        cmd.execute([0])
        cmd.send()  # 触发 _ensure_connected
        assert cmd._sock is not None
        print(f"  上下文内: socket 已创建并发送消息")

    print("  退出上下文: 已自动 close()")


def test_send_command():
    """测试发送命令"""
    print("\n=== 测试发送命令 ===")
    print(f"  目标地址: {ZMQ_ACT_ADDR}")
    print("  提示: 需要 Rust 后端 Act 模块运行中")

    try:
        with OTAcleCommand() as cmd:
            cmd.execute([0])
            cmd.set_params({"target_x": 100, "target_y": 200})
            cmd.send()
            print(f"  [1] 发送成功: execute=[0], params={{target_x:100, target_y:200}}")

            cmd.clear_execute()
            cmd.execute([1, 2])
            cmd.send()
            print(f"  [2] 发送成功: execute=[1,2]")

    except Exception as e:
        print(f"  发送失败: {e}")
        print("  (这是正常的，如果 Rust 后端未运行)")


def test_convenience_function():
    """测试便捷函数"""
    print("\n=== 测试便捷函数 ===")

    try:
        send_command([0], {"target_x": 50})
        print("  send_command([0], {target_x: 50}) 发送成功")
    except Exception as e:
        print(f"  send_command 失败: {e}")
        print("  (这是正常的，如果 Rust 后端未运行)")


def test_empty_params():
    """测试空参数"""
    print("\n=== 测试空参数 ===")

    cmd = OTAcleCommand()
    cmd.execute([0])
    cmd.set_params({})  # 空参数字典

    msg = cmd.build()
    assert msg["params"] == {}
    print(f"  空 params: OK")

    cmd.send()
    print(f"  发送空 params: OK")

    cmd.close()


def main():
    print("=" * 60)
    print("OTAcle Act 模块工具方法测试")
    print("=" * 60)

    # 不依赖网络的测试
    test_command_creation()
    test_execute_actions()
    test_params()
    test_build_message()
    test_repr()
    test_context_manager()

    # 网络测试
    print("\n" + "=" * 60)
    print("网络测试（需要 Rust 后端 Act 模块运行）")
    print("=" * 60)

    test_send_command()
    test_convenience_function()
    test_empty_params()

    print("\n" + "=" * 60)
    print("所有测试完成")
    print("=" * 60)


if __name__ == "__main__":
    main()
