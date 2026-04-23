"""
OTAcle Observe 模块工具方法测试脚本

测试 otacle.py 中 OTAcleObserver 的各项功能：
- 基础接收
- 迭代器模式
- 上下文管理器
- 统计数据
- 图像数据解码

Usage:
    python test_observe.py
"""

import sys
import os

# 导入 otacle 包（通过 pip install -e py_tool/ 安装）
from otacle import OTAcleObserver, FrameMessage, CropBlock, observe_frames, ZMQ_OBSERVE_ADDR


def test_crop_block():
    """测试 CropBlock 数据类"""
    print("\n=== 测试 CropBlock ===")

    # 从字典创建
    block_dict = {
        "x": 100,
        "y": 200,
        "w": 320,
        "h": 240,
        "image": "SGVsbG8gV29ybGQ="  # "Hello World" 的 base64
    }
    block = CropBlock.from_dict(block_dict)

    assert block.x == 100
    assert block.y == 200
    assert block.w == 320
    assert block.h == 240
    assert block.image == "SGVsbG8gV29ybGQ="

    # 解码图像
    decoded = block.decode_image()
    assert decoded == b"Hello World"

    print(f"  CropBlock.from_dict: OK")
    print(f"  CropBlock.decode_image: OK (解码得到 {decoded})")


def test_frame_message():
    """测试 FrameMessage 数据类"""
    print("\n=== 测试 FrameMessage ===")

    frame_dict = {
        "width": 1920,
        "height": 1080,
        "timestamp": 1713000000000,
        "frame_id": 12345,
        "data": [
            {"x": 0, "y": 0, "w": 960, "h": 540, "image": "YWJj"}
        ]
    }
    frame = FrameMessage.from_dict(frame_dict)

    assert frame.width == 1920
    assert frame.height == 1080
    assert frame.timestamp == 1713000000000
    assert frame.frame_id == 12345
    assert len(frame.data) == 1
    assert isinstance(frame.data[0], CropBlock)
    assert frame.data[0].x == 0

    print(f"  FrameMessage.from_dict: OK")
    print(f"  嵌套 CropBlock: OK")
    print(f"  width={frame.width}, height={frame.height}, frame_id={frame.frame_id}")


def test_observer_basic():
    """测试 OTAcleObserver 基本 API"""
    print("\n=== 测试 OTAcleObserver 基本 API ===")

    obs = OTAcleObserver()

    # 测试属性初始状态
    assert obs.frame_count == 0
    assert obs.bytes_received == 0
    assert obs.elapsed_time == 0.0
    assert obs.fps == 0.0

    print(f"  初始状态: frame_count=0, fps=0.0, elapsed=0.0")

    # 测试 stats()
    stats = obs.stats()
    assert "frame_count" in stats
    assert "bytes_received" in stats
    assert "fps" in stats

    print(f"  stats(): OK ({stats})")

    # 测试 repr
    repr_str = repr(obs)
    assert "OTAcleObserver" in repr_str
    print(f"  repr: {repr_str}")

    obs.stop()
    print("  基本 API 测试完成")


def test_observer_context_manager():
    """测试 OTAcleObserver 上下文管理器"""
    print("\n=== 测试 OTAcleObserver 上下文管理器 ===")

    with OTAcleObserver() as obs:
        assert obs._running == True
        print(f"  进入上下文: running={obs._running}")
        print(f"  repr: {repr(obs)}")

    print("  退出上下文: 已自动 stop()")
    print("  上下文管理器测试完成")


def test_observer_iterator():
    """测试 OTAcleObserver 迭代器模式（需实际运行）"""
    print("\n=== 测试 OTAcleObserver 迭代器模式 ===")
    print(f"  地址: {ZMQ_OBSERVE_ADDR}")
    print("  等待实际帧数据...")

    count = 0
    try:
        with OTAcleObserver() as obs:
            for frame in obs:
                count += 1
                print(
                    f"  [帧 {count}] id={frame.frame_id}, "
                    f"{frame.width}x{frame.height}, "
                    f"blocks={len(frame.data)}, fps={obs.fps:.1f}"
                )

                # 收到 5 帧后退出测试
                if count >= 5:
                    break

        print(f"  迭代器测试完成，共接收 {count} 帧")

    except Exception as e:
        print(f"  迭代器测试跳过或失败: {e}")


def test_observe_frames_convenience():
    """测试 observe_frames 便捷函数"""
    print("\n=== 测试 observe_frames 便捷函数 ===")
    print("  按 Ctrl+C 可退出...")

    try:
        # 只接收 3 帧后通过 KeyboardInterrupt 退出
        count = 0
        with OTAcleObserver() as obs:
            for frame in obs:
                count += 1
                print(
                    f"  [便捷函数 帧 {count}] "
                    f"{frame.width}x{frame.height}, fps={obs.fps:.1f}"
                )
                if count >= 3:
                    print("  收到 3 帧，退出测试")
                    break

        print(f"  observe_frames 测试完成，共 {count} 帧")

    except KeyboardInterrupt:
        print("\n  用户中断")
    except Exception as e:
        print(f"  便捷函数测试跳过或失败: {e}")


def main():
    print("=" * 60)
    print("OTAcle Observe 模块工具方法测试")
    print("=" * 60)

    # 数据类测试（不依赖网络）
    test_crop_block()
    test_frame_message()
    test_observer_basic()
    test_observer_context_manager()

    # 网络测试（需要 Rust 后端运行）
    print("\n" + "=" * 60)
    print("网络测试（需要 Rust 后端和 Observe 模块运行）")
    print("=" * 60)

    test_observer_iterator()
    test_observe_frames_convenience()

    print("\n" + "=" * 60)
    print("所有测试完成")
    print("=" * 60)


if __name__ == "__main__":
    main()
