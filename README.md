# OTAcle

**OTAcle** — 强化学习流程控制桌面应用。

核心理念：只做**流程控制和可视化**，不提供任何算法、训练代码、图像处理方法。算法部分由用户自己准备，OTAcle 负责把它们串联起来。

---

## 项目概述

OTAcle 是一个基于 [Tauri 2](https://tauri.app/) 的桌面应用，用于强化学习（RL）实验的流程控制。Python 端运行 RL 算法，OTAcle 负责：

| 模块 | 职责 | 通信方向 |
|------|------|----------|
| **Act** | 动作执行（键盘、鼠标） | Python → OTAcle |
| **Observe** | 屏幕截图并传输给 Python | OTAcle → Python |
| **Think** | 接收决策日志并可视化 | Python → OTAcle |

---

## 系统架构

```
┌─────────────────┐                      ┌─────────────────┐
│   Python RL     │                      │  OTAcle Desktop  │
│   Algorithm     │                      │     (Rust)       │
└────────┬────────┘                      └────────┬────────┘
         │                                        │
         │  ZMQ PUSH  ──────────────────────────►  │
         │  tcp://127.0.0.1:5555                  │
         │                                        │  Act 模块
         │                                        │  - 键盘/鼠标动作执行
         │  ZMQ SUB  ◄──────────────────────────  │
         │  tcp://127.0.0.1:5556                  │  Observe 模块
         │                                        │  - Windows Graphics Capture
         │                                        │  - 截图 → Base64 帧
         │  ZMQ PUSH  ──────────────────────────►  │
         │  tcp://127.0.0.1:5557                  │
         │                                        │  Think 模块
         │                                        │  - 决策日志 → 可视化
         └───────────────────────────────────────┘
```

### 默认端口

| 模块 | 端口 | ZMQ 模式 | 方向 |
|------|------|----------|------|
| Act | 5555 | PULL | Python → Rust |
| Observe | 5556 | PUB | Rust → Python |
| Think | 5557 | PULL | Python → Rust |

---

## 目录结构

```
OTAcle/
├── desktop/                    # Tauri 2 桌面应用
│   ├── src/                    # Vue 3 + TypeScript 前端
│   │   ├── components/         # UI 组件
│   │   ├── composables/        # Vue Composition API 逻辑
│   │   ├── pages/              # 页面（ActPage, ObservePage, ThinkPage）
│   │   ├── types/              # TypeScript 类型定义
│   │   └── router/             # 路由配置
│   └── src-tauri/              # Rust 后端
│       └── src/
│           ├── act/            # Act 模块（动作配置与执行）
│           ├── observe/        # Observe 模块（屏幕截图）
│           ├── think/          # Think 模块（决策日志）
│           ├── communication/   # ZMQ 通信封装
│           ├── input/          # 输入控制（键盘、鼠标）
│           └── commands/       # Tauri 命令
├── py_tool/                    # Python SDK（ZMQ 通信封装）
│   └── src/otacle/
│       └── otacle.py           # 核心类
├── example/                    # Python RL 示例
│   ├── test_act.py             # Act 模块测试
│   ├── test_observe.py         # Observe 模块测试
│   ├── test_think.py           # Think 模块测试
│   └── .otacle/                # 项目配置目录
└── README.md
```

---

## 快速开始

### 环境要求

- **操作系统**：Windows 10 1903+（需要 Windows Graphics Capture API）
- **Rust**：1.70+
- **Node.js**：18+
- **Python**：3.8+

### 构建桌面应用

```bash
cd desktop
pnpm install
pnpm tauri dev    # 开发模式
# 或
pnpm tauri build  # 生产构建
```

### 安装 Python SDK

```bash
# 可编辑模式安装（开发用）
pip install -e /path/to/OTAcle/py_tool

# 或直接引用（不安装）
import sys
sys.path.insert(0, "path/to/OTAcle/py_tool/src")
from otacle import OTAcleCommand
```

---

## Act 模块

Act 模块负责接收 Python 端的动作命令并执行键盘、鼠标操作。

### 动作配置

动作配置文件为 `.otacle/actions.json`：

```json
{
  "default_backend": "win32",
  "actions": [
    {
      "name": "jump",
      "type": "key",
      "key": "space",
      "hold_time_ms": 5
    },
    {
      "name": "move_to_target",
      "type": "mouse_move",
      "x": 0,
      "y": 0,
      "variables": [
        { "param_name": "target_x", "field_name": "x" },
        { "param_name": "target_y", "field_name": "y" }
      ]
    },
    {
      "name": "attack",
      "type": "mouse_click",
      "button": "left",
      "count": 1,
      "hold_time_ms": 10
    }
  ]
}
```

### 动作类型

| 类型 | 说明 | 关键字段 |
|------|------|----------|
| `key` | 单次按键 | `key`, `hold_time_ms` |
| `key_sequence` | 按键序列 | `keys[]`, `default_interval_ms` |
| `mouse_click` | 鼠标点击 | `button`, `count`, `interval_ms`, `hold_time_ms` |
| `mouse_move` | 鼠标移动 | `x`, `y`, `duration_ms`, `variables` |
| `mouse_scroll` | 鼠标滚动 | `direction`, `amount` |
| `delay` | 延迟等待 | `duration_ms` |
| `text` | 文本输入 | `content` |

#### 字段说明

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `name` | string | null | 动作名称（可选） |
| `backend` | string | null | 覆盖默认后端（`win32` 或 `enigo`） |
| `key` | string | - | 按键名称，如 `"space"`, `"ctrl+c"` |
| `hold_time_ms` | u64 | 5 | 按住时长（毫秒），0=瞬时点击 |
| `button` | string | - | 鼠标按钮：`left`, `right`, `middle` |
| `count` | u32 | 1 | 点击次数 |
| `direction` | string | - | 滚动方向：`up`, `down`, `left`, `right` |
| `amount` | u32 | 1 | 滚动量（Windows 滚轮增量=120） |
| `duration_ms` | u64 | null | 移动持续时间（毫秒），0=瞬移 |
| `content` | string | - | 要输入的文本内容 |

### 动态参数替换

动作配置中的 `variables` 字段定义可动态替换的参数：

```json
{
  "type": "mouse_move",
  "x": 0,
  "y": 0,
  "variables": [
    { "param_name": "target_x", "field_name": "x" },
    { "param_name": "target_y", "field_name": "y" }
  ]
}
```

Python 端发送的 `params` 中的键名对应 `param_name`，执行时替换对应 `field_name` 的值。

### 输入后端

| 后端 | 说明 | 特点 |
|------|------|------|
| `win32` | Windows API | 可定向发送到后台窗口，使用 `PostMessageW` |
| `enigo` | 跨平台库 | 依赖前台窗口，模拟 USB HID 输入 |

**优先级**：`action.backend` > `default_backend` > `win32`

### Python SDK

```python
from otacle import OTAcleCommand

with OTAcleCommand() as cmd:
    cmd.execute([0, 2])           # 执行 index 0 和 2 的动作
    cmd.set_params({"target_x": 100, "target_y": 200})
    cmd.send()
```

OTAcleCommand 发送的 JSON 消息：

```json
{
  "execute": [true, false, true, false, false, false, false, false, false, false],
  "params": {"target_x": 100, "target_y": 200}
}
```

### ZMQ 消息格式

```json
{
  "execute": [true, false, true],
  "params": {"target_x": 100, "target_y": 200}
}
```

---

## Observe 模块

Observe 模块负责捕获窗口截图并传输给 Python 端。

### 截图配置

配置文件为 `.otacle/observe.json`：

```json
{
  "capture": {
    "frame_rate": 3,
    "target_width": 640,
    "target_height": 480
  },
  "crop_regions": [
    { "x": 200, "y": 300, "w": 100, "h": 100 },
    { "x": 0, "y": 0, "w": 100, "h": 100 }
  ]
}
```

| 字段 | 说明 |
|------|------|
| `frame_rate` | 帧率（1-120），默认 3 |
| `target_width` | 输出宽度（1-7680），默认 640 |
| `target_height` | 输出高度（1-4320），默认 480 |
| `crop_regions` | 裁切区域数组 |
| `x`, `y` | 裁切区域左上角坐标 |
| `w`, `h` | 裁切区域宽高 |

### ZMQ PUB 帧消息格式

Rust 端通过 ZMQ PUB 发送 Base64 编码的图像帧：

```json
{
  "width": 1920,
  "height": 1080,
  "timestamp": 1713000000000,
  "frame_id": 12345,
  "data": [
    {
      "x": 200,
      "y": 300,
      "w": 100,
      "h": 100,
      "image": "base64-encoded-image..."
    }
  ]
}
```

### Python SDK

```python
from otacle import OTAcleObserver

with OTAcleObserver() as obs:
    for frame in obs:
        print(f"帧 {frame.frame_id}: {frame.width}x{frame.height}")
        for block in frame.data:
            print(f"  裁切块 {block.x},{block.y} {block.w}x{block.h}")
```

---

## Think 模块

Think 模块负责接收 Python 端的决策日志并可视化显示。

### 可视化配置

配置文件为 `.otacle/think.json`：

```json
{
  "max_data_points": 500,
  "display_fields": [
    { "name": "step", "title": "训练步数", "chart_type": "line" },
    { "name": "reward", "title": "Reward", "chart_type": "line" },
    { "name": "epsilon", "title": "Epsilon", "chart_type": "gauge" }
  ]
}
```

| 字段 | 说明 |
|------|------|
| `max_data_points` | 环形缓冲区大小，默认 1000 |
| `display_fields` | 显示字段配置 |
| `name` | 字段名（对应 DecisionLog.custom 中的键） |
| `title` | 显示标题 |
| `chart_type` | 图表类型：`line`, `bar`, `area`, `gauge` |

### 决策日志格式

Python 端发送的决策日志：

```json
{
  "step": 100,
  "custom": {
    "reward": 1.0,
    "epsilon": 0.1,
    "q_values": {"a": 0.5, "b": 0.3}
  }
}
```

### Python SDK

```python
from otacle import OTAcleThinkSender

with OTAcleThinkSender() as sender:
    sender.send_step(1, reward=1.0, epsilon=0.9)
    sender.send_step(2, reward=0.8, epsilon=0.8)
```

---

## 通信配置

配置文件为 `.otacle/comm.json`：

```json
{
  "act_pull_address": "tcp://127.0.0.1:5555",
  "observe_pub_address": "tcp://127.0.0.1:5556",
  "think_pull_address": "tcp://127.0.0.1:5557"
}
```

地址必须以 `tcp://`、`ipc://` 或 `inproc://` 开头。

---

## 窗口定位语法

执行动作时可指定目标窗口，使用以下格式：

| 格式 | 说明 | 示例 |
|------|------|------|
| `"A"` 或空 | 当前前台窗口 | `"A"` |
| `"id:395542"` | 窗口句柄（十进制） | `"id:395542"` |
| `"id:0x9999"` | 窗口句柄（十六进制） | `"id:0x60916"` |
| `"class:Notepad"` | 窗口类名 | `"class:Notepad"` |
| `"exe:notepad.exe"` | 进程名 | `"exe:notepad"` |
| `"pid:1234"` | 进程 ID | `"pid:1234"` |
| `"Notepad"` | 窗口标题前缀匹配 | `"Untitled - Notepad"` |

---

## Python SDK 参考

### 核心类

| 类 | 说明 |
|---|------|
| `OTAcleCommand` | 发送动作命令到 Act 模块 |
| `OTAcleObserver` | 接收来自 Observe 模块的图像帧 |
| `OTAcleThinkSender` | 发送决策日志到 Think 模块 |

### 数据类

| 类 | 字段 | 说明 |
|---|------|------|
| `CropBlock` | `x, y, w, h, image` | 裁切块，image 为 Base64 |
| `FrameMessage` | `width, height, timestamp, frame_id, data[]` | 图像帧 |
| `DecisionLog` | `step, custom` | 决策日志 |

### 便捷函数

```python
from otacle import send_command, send_decision_log, observe_frames

# 发送动作命令
send_command([0, 1], {"target_x": 100})

# 发送决策日志
send_decision_log(100, {"reward": 1.0, "epsilon": 0.1})

# 接收图像帧（生成器）
for frame in observe_frames():
    print(frame.frame_id)
```

### 常量

```python
from otacle import ZMQ_ACT_ADDR, ZMQ_OBSERVE_ADDR, ZMQ_THINK_ADDR
# tcp://127.0.0.1:5555, tcp://127.0.0.1:5556, tcp://127.0.0.1:5557
```

---

## 项目配置

创建项目后，目录结构如下：

```
my_rl_project/
├── .otacle/                # OTAcle 配置目录（自动创建）
│   ├── actions.json         # Act 模块动作配置
│   ├── observe.json         # Observe 模块截图配置
│   ├── comm.json            # 通信地址配置
│   ├── think.json           # Think 模块可视化配置
│   └── project.json         # 项目元数据
├── train.py                 # 你的 RL 训练代码
└── model.py                 # 你的模型代码
```

---

## 故障排除

### 常见问题

| 问题 | 可能原因 | 解决方案 |
|------|----------|----------|
| 连接失败 | 防火墙阻止端口 | 确认 5555-5557 端口开放 |
| 截图黑屏 | 目标窗口最小化或不可见 | 确保窗口可见且未最小化 |
| 动作执行无效 | 动作索引错误 | 检查 `execute` 数组长度是否与配置的动作数量一致 |
| 配置文件加载失败 | JSON 格式错误 | 检查配置文件语法 |

### 日志位置

- Tauri 日志：`%APPDATA%/com.otacle.desktop/logs/`

---

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri 2 |
| 前端 | Vue 3 + TypeScript + Vite |
| 后端 | Rust |
| 样式 | Tailwind CSS v4 |
| 图表 | ECharts |
| 跨语言通信 | ZMQ |
| 截图 | windows-capture 2.0 + Windows Graphics Capture API |
| 输入控制 | enigo / Win32 API |

---

## 开发命令

### 前端

```bash
cd desktop
pnpm dev              # 启动 Vite 开发服务器（端口 1420）
pnpm build            # TypeScript 类型检查 + Vite 构建
pnpm tauri dev        # 以开发模式启动完整的 Tauri 应用
pnpm tauri build      # 构建生产版本 Tauri 应用
```

### Rust 后端

```bash
cd desktop/src-tauri
cargo check           # Rust 类型检查
cargo build           # Rust 编译
cargo test           # 运行测试
```

---

## License

MIT
