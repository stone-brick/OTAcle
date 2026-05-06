# -*- coding: utf-8 -*-
"""
微信跳一跳 AI Agent

功能：
1. 接收截图，识别棋子位置和目标墩位置
2. 计算跳跃距离和按压时长
3. 执行跳跃动作
4. 发送决策日志

使用方式：
1. 启动 OTAcle 桌面应用，加载 example 项目
2. 在 Observe 页面选择"跳一跳"窗口并启动捕获
3. 在 Think 页面启动监听
4. 运行：python demo.py
"""

import sys
import time
import math
import hashlib

sys.path.insert(0, "../py_tool/src")

from otacle import OTAcleThinkSender, OTAcleCommand, OTAcleObserver

# 跳一跳物理参数
K = 35  # 按压时间系数: 按压时间(ms) = K × 距离(像素)

# 连续静止帧检测
STILL_FRAME_COUNT = 2  # 连续多少帧无变化认为静止


def calc_frame_hash(img_bytes: bytes) -> str:
    """计算图像数据的哈希值"""
    return hashlib.md5(img_bytes).hexdigest()


def is_still_frame(prev_hash: str | None, current_hash: str) -> bool:
    """判断是否为静止帧"""
    return prev_hash is not None and prev_hash == current_hash


def get_pixel_rgb(image_bytes: bytes, x: int, y: int, width: int) -> tuple[int, int, int]:
    """从图像字节数据中获取 (x, y) 位置的 RGB 值（BGRA 格式）"""
    idx = (y * width + x) * 4
    if idx + 2 >= len(image_bytes):
        return (0, 0, 0)
    b, g, r = image_bytes[idx], image_bytes[idx + 1], image_bytes[idx + 2]
    return (r, g, b)


def find_piece(img_bytes: bytes, w: int, h: int) -> tuple[int, int]:
    """
    在游戏画面中查找棋子位置
    返回: (x, y) 棋子中心点坐标
    """
    # 跳一跳的棋子是一个黑色的小人
    # 扫描找到颜色符合条件的最下方像素点（棋子底部）
    piece_x_sum = 0
    piece_x_count = 0
    piece_y_max = 0

    for y in range(h):
        for x in range(w):
            r, g, b = get_pixel_rgb(img_bytes, x, y, w)
            # 棋子颜色: RGB(55, 56, 97) 蓝色调黑色
            # 容差范围 ±15
            if 40 <= r <= 70 and 41 <= g <= 71 and 82 <= b <= 112:
                piece_x_sum += x
                piece_x_count += 1
                piece_y_max = max(piece_y_max, y)

    if piece_x_count > 0:
        piece_x = piece_x_sum // piece_x_count
        piece_y = piece_y_max - 10  # 粗略估算棋子中心（顶部在 y=0 附近）
        return (piece_x, piece_y)

    return (w // 2, h // 2)  # 默认值


def find_next_block(img_bytes: bytes, w: int, h: int, piece_y: int) -> tuple[int, int]:
    """
    在游戏画面中查找下一个墩的中心位置
    返回: (x, y) 目标墩中心坐标
    """
    # 从棋子所在行的上方区域开始扫描
    # 找到颜色变化点（背景到方块的边界）
    block_x_sum = 0
    block_x_count = 0
    block_y = 0

    # 扫描上半部分区域（棋子上方）
    scan_height = piece_y if piece_y > 0 else h // 2

    for y in range(scan_height):
        # 检测是否有方块边缘（从背景色突变到方块色）
        for x in range(w):
            if x + 1 >= w:
                continue
            r, g, b = get_pixel_rgb(img_bytes, x, y, w)
            r2, g2, b2 = get_pixel_rgb(img_bytes, x + 1, y, w)

            # 背景色通常是浅色/白色，方块色偏深
            # 检测到颜色突变
            color_diff = abs(int(r) - int(r2)) + abs(int(g) - int(g2)) + abs(int(b) - int(b2))
            if color_diff > 50:  # 阈值
                block_x_sum += x
                block_x_count += 1
                block_y = max(block_y, y)

    if block_x_count > 0:
        block_x = block_x_sum // block_x_count
        block_y = block_y + 15  # 估算方块中心
        return (block_x, block_y)

    # 默认返回右侧中间位置
    return (w - 30, h // 3)


def calc_distance(piece: tuple[int, int], block: tuple[int, int]) -> float:
    """计算两点欧几里得距离"""
    return math.sqrt((block[0] - piece[0]) ** 2 + (block[1] - piece[1]) ** 2)


def calc_hold_time(distance: float, k: float = K) -> int:
    """按压时间 = K × 距离"""
    return int(k * distance)


def main():
    print("=" * 50)
    print("微信跳一跳 AI Agent")
    print("=" * 50)

    step = 0
    prev_score = 0

    try:
        # 启动 Observe（接收图像帧）
        print("\n[Observe] 启动模块...")
        observer = OTAcleObserver()
        observer.start()
        print(f"    -> 监听 {observer._address}")

        # 启动 Think（发送决策日志）
        print("[Think] 启动模块...")
        sender = OTAcleThinkSender()
        sender.start()
        print(f"    -> 连接到 {sender._address}")

        # 启动 Act（发送动作命令）
        print("[Act] 启动模块...")
        cmd = OTAcleCommand()
        print(f"    -> 连接到 {cmd._address}")

        print("\n开始跳跃循环（Ctrl+C 退出）...")
        print("-" * 50)

        step = 0
        prev_score = 0
        prev_frame_hash = None
        still_count = 0

        while True:
            step += 1

            # Observe: 接收图像帧
            frame = observer.recv(timeout=500)
            if not frame:
                continue

            # 获取裁切块2（游戏画面）
            if len(frame.data) < 2:
                print(f"[Observe] 警告: 只有 {len(frame.data)} 个裁切块")
                continue

            block = frame.data[1]  # 裁切块2: 游戏画面
            img_bytes = block.decode_image()
            w, h = block.w, block.h

            # 静止帧检测：连续两帧无变化才进行决策
            current_hash = calc_frame_hash(img_bytes)
            if is_still_frame(prev_frame_hash, current_hash):
                still_count += 1
                if still_count < STILL_FRAME_COUNT:
                    print(f"[Observe] 静止帧 ({still_count}/{STILL_FRAME_COUNT})，跳过决策")
                    continue
            else:
                still_count = 0
            prev_frame_hash = current_hash

            # 识别棋子位置
            piece = find_piece(img_bytes, w, h)
            print(f"[Observe] 棋子位置: {piece}")

            # 识别目标墩位置
            block_pos = find_next_block(img_bytes, w, h, piece[1])
            print(f"[Observe] 目标墩位置: {block_pos}")

            # 计算距离
            distance = calc_distance(piece, block_pos)
            print(f"[Observe] 跳跃距离: {distance:.1f} 像素")

            # 计算按压时长
            hold_time = calc_hold_time(distance)
            print(f"[Act] 按压时长: {hold_time} ms")

            # 执行跳跃动作
            cmd.clear_execute()
            cmd.execute([0])
            cmd.set_params({"hold_time": hold_time})
            cmd.send()
            print(f"[Act] -> 执行跳跃")

            # Think: 发送决策日志
            sender.send_step(
                step,
                distance=round(distance, 2),
                hold_time=hold_time,
                piece_x=piece[0],
                piece_y=piece[1],
                block_x=block_pos[0],
                block_y=block_pos[1],
            )
            print(f"[Think] Step {step}: distance={distance:.1f}, hold_time={hold_time}")

            # 等待棋子落地
            time.sleep(1.5)

    except KeyboardInterrupt:
        print("\n\nAgent 停止")
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
