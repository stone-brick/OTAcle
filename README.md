# OTAcle

强化学习流程控制桌面应用。

## Act 模块 - 动作配置

### 配置文件格式

动作配置文件为 JSON 格式，定义数字动作 ID 到具体操作的映射。

```json
{
  "0": {"type": "key", "key": "space"},
  "1": {"type": "key", "key": "ctrl+c"},
  "2": {"type": "key_sequence", "keys": ["ctrl", "v"], "interval_ms": 10},
  "3": {"type": "mouse_click", "button": "left", "count": 2},
  "4": {"type": "mouse_move", "x": 500, "y": 300},
  "5": {"type": "text", "content": "Hello!"}
}
```

### 动作类型

| type | 说明 | 字段 |
|------|------|------|
| `key` | 单键或组合键 | `key`, `hold_time_ms?` |
| `key_sequence` | 顺序按键 | `keys[]`, `interval_ms?` |
| `mouse_click` | 鼠标点击 | `button`, `count?`, `interval_ms?` |
| `mouse_move` | 鼠标移动 | `x`, `y`, `duration_ms?` |
| `text` | 文本输入 | `content` |

### 组合键语法

使用 `+` 分隔按键，例如 `"ctrl+c"`、`"ctrl+shift+a"`。

**重要**：修饰键应写在前面。执行时按键释放顺序是反向的：
1. 按下所有键（从左到右）
2. 释放所有键（从右到左）

### 按键名称

参考 [enigo 文档](https://docs.rs/enigo)，常用按键：

- 字母：`a` - `z`
- 数字：`0` - `9`
- 功能键：`f1` - `f12`
- 方向键：`up`、`down`、`left`、`right`
- 控制键：`ctrl`、`shift`、`alt`、`space`、`enter`、`escape`
- 编辑键：`tab`、`backspace`、`delete`、`home`、`end`

### 鼠标按钮

`button` 字段支持：`left`、`right`、`middle`

### 窗口定位

执行动作时可指定目标窗口，使用以下格式：

- `"Notepad"` - 窗口标题（前缀匹配）
- `"id:395542"` - 直接指定 HWND
- `"class:Notepad"` - 窗口类名
- `"exe:notepad.exe"` - 进程名
- `"A"` 或空 - 当前前台窗口

## 开发

```bash
# 前端开发
cd frontend && pnpm tauri dev

# Rust 检查
cd frontend/src-tauri && cargo check
```
