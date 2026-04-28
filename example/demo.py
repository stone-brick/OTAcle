# -*- coding: utf-8 -*-
"""
OTAcle 完整演示脚本

演示 Observe → Think → Act 闭环：
1. Observe 模块：接收图像帧（ZMQ SUB）
2. Think 模块：发送模拟决策日志（强化学习训练数据）
3. Act 模块：发送动作命令执行动作

使用方式：
1. 启动 OTAcle 桌面应用，加载 example 项目
2. 在 Observe 页面选择目标窗口并启动捕获
3. 在 Think 页面启动监听
4. 运行：python demo.py
"""

import sys
import time

# 添加 SDK 路径（根据实际目录结构）
sys.path.insert(0, "../py_tool/src")

from otacle import OTAcleThinkSender, OTAcleCommand, OTAcleObserver


def get_pixel_rgb(image_bytes: bytes, x: int, y: int, width: int) -> tuple[int, int, int]:
    """从图像字节数据中获取 (x, y) 位置的 RGB 值（BGRA 格式）"""
    idx = (y * width + x) * 4
    if idx + 2 >= len(image_bytes):
        return (0, 0, 0)
    b, g, r = image_bytes[idx], image_bytes[idx + 1], image_bytes[idx + 2]
    return (r, g, b)


def main():
    """主演示流程"""
    print("=" * 50)
    print("OTAcle 完整演示")
    print("=" * 50)

    step = 0

    try:
        # 启动 Observe（接收图像帧）
        print("\n[0] 启动 Observe 模块...")
        observer = OTAcleObserver()
        observer.start()
        print(f"    -> 监听 {observer._address}")

        # 启动 Think（发送决策日志）
        print("[1] 启动 Think 模块...")
        sender = OTAcleThinkSender()
        sender.start()
        print(f"    -> 连接到 {sender._address}")

        # 启动 Act（发送动作命令）
        print("[2] 启动 Act 模块...")
        cmd = OTAcleCommand()
        print(f"    -> 连接到 {cmd._address}")

        print("\n开始循环演示（Ctrl+C 退出）...")
        print("-" * 50)

        while True:
            step += 1

            # Observe: 尝试接收一帧图像
            frame = observer.recv(timeout=100)
            if frame:
                print(f"[Observe] 帧 {frame.frame_id}: {frame.width}x{frame.height}, blocks={len(frame.data)}")

                # 计算每个块 (1, 1) 位置的 RGB 总和
                total_rgb_sum = 0
                for i, block in enumerate(frame.data):
                    img_bytes = block.decode_image()
                    r, g, b = get_pixel_rgb(img_bytes, 1, 1, block.w)
                    print(f"    块 {i}: (1,1) 位置 RGB = ({r}, {g}, {b})")
                    total_rgb_sum += r + g + b

                print(f"    RGB 总和: {total_rgb_sum}")

                # 如果 RGB 总和能被 4 整除，触发动作 0
                if total_rgb_sum % 4 == 0:
                    cmd.clear_execute()
                    cmd.execute([0])
                    cmd.send()
                    print(f"    [Act]   -> 触发动作 [0] (RGB 总和 {total_rgb_sum} 可被 4 整除)")

            # Think: 发送决策日志
            # 模拟强化学习环境，随时间衰减 epsilon
            reward = round(1.0 / step, 4)
            epsilon = max(0.01, 1.0 / step)

            sender.send_step(
                step,
                reward=reward,
                epsilon=epsilon,
                action=0,
                loss=round(0.5 / step, 4),
            )
            print(f"[Think] Step {step}: reward={reward}, epsilon={epsilon:.4f}")

            time.sleep(1)

    except KeyboardInterrupt:
        print("\n\n演示结束")
    except Exception as e:
        import traceback
        traceback.print_exc()
        print(f"\n错误: {e}")
    finally:
        observer.stop() if 'observer' in dir() else None
        sender.stop() if 'sender' in dir() else None
        cmd.close() if 'cmd' in dir() else None


if __name__ == "__main__":
    main()