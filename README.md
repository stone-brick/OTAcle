# OTAcle

强化学习流程控制桌面应用。

## Act 模块 - 动作配置

### 配置文件格式

动作配置文件为 JSON 格式，支持通过 `index` 索引执行对应动作。

```json
{
  "default_backend": "win32",
  "actions": [
    {"index": 0, "name": "jump", "type": "key", "key": "space"},
    {"index": 1, "name": "enter", "type": "key", "key": "enter"},
    {"index": 2, "name": "copy", "type": "key", "key": "ctrl+c"},
    {"index": 3, "name": "hello", "type": "text", "content": "Hello from ZMQ!"},
    {
      "index": 4,
      "name": "scroll_down",
      "type": "key_sequence",
      "default_interval_ms": 5,
      "keys": [
        {"key": "end"},
        {"key": "space", "interval_ms": 10}
      ]
    },
    {
      "index": 5,
      "name": "move",
      "type": "mouse_move",
      "x": 0,
      "y": 0,
      "variables": [
        {"param_name": "target_x", "field_name": "x"},
        {"param_name": "target_y", "field_name": "y"}
      ]
    },
    {"index": 6, "name": "scroll", "type": "mouse_scroll", "direction": "down", "amount": 3},
    {"index": 7, "name": "wait", "type": "delay", "duration_ms": 1000}
  ]
}
```

> 参考示例：`example/python/test_actions.json`

### 全局配置

| 字段                | 类型     | 默认值       | 说明                          |
| ----------------- | ------ | --------- | --------------------------- |
| `default_backend` | string | `"win32"` | 默认输入后端，可选 `win32` 或 `enigo` |
| `actions`         | array  | (必填)      | 动作列表                        |

### 动作类型详解

#### 1. `key` - 单键或组合键

| 字段             | 类型     | 必填  | 默认值 | 说明                        |
| -------------- | ------ | --- | --- | ------------------------- |
| `type`         | string | 是   | -   | 固定为 `"key"`               |
| `key`          | string | 是   | -   | 按键名称，支持组合键如 `"ctrl+c"`    |
| `hold_time_ms` | number | 否   | `5` | 按住时长（毫秒），防止事件被丢弃          |
| `backend`      | string | 否   | 继承  | 输入后端，可选 `win32` 或 `enigo` |

**组合键语法**：使用 `+` 分隔按键，例如 `"ctrl+c"`、`"ctrl+shift+a"`。
执行时按键释放顺序是反向的：先按下所有键，再从右到左释放。

#### 2. `key_sequence` - 顺序按键序列

| 字段                    | 类型     | 必填  | 默认值 | 说明                    |
| --------------------- | ------ | --- | --- | --------------------- |
| `type`                | string | 是   | -   | 固定为 `"key_sequence"`  |
| `keys`                | array  | 是   | -   | 按键序列列表                |
| `default_interval_ms` | number | 否   | `5` | keys 中每个按键之间的默认间隔（毫秒） |
| `backend`             | string | 否   | 继承  | 输入后端                  |

**keys 数组中的每个元素**：

| 字段             | 类型     | 必填  | 默认值                   | 说明                   |
| -------------- | ------ | --- | --------------------- | -------------------- |
| `key`          | string | 是   | -                     | 按键名称                 |
| `interval_ms`  | number | 否   | `default_interval_ms` | 到下一个按键的间隔（毫秒）        |
| `hold_time_ms` | number | 否   | `5`                   | 此按键的按住时长（毫秒），防止事件被丢弃 |

#### 3. `mouse_click` - 鼠标点击

| 字段             | 类型     | 必填  | 默认值 | 说明                           |
| -------------- | ------ | --- | --- | ---------------------------- |
| `type`         | string | 是   | -   | 固定为 `"mouse_click"`          |
| `button`       | string | 是   | -   | 鼠标按钮：`left`、`right`、`middle` |
| `count`        | number | 否   | `1` | 点击次数                         |
| `interval_ms`  | number | 否   | `0` | 每次点击之间的间隔（毫秒）                |
| `hold_time_ms` | number | 否   | `5` | 按住时长（毫秒），防止事件被丢弃             |
| `backend`      | string | 否   | 继承  | 输入后端                         |
| `variables`    | array | 否   | -   | 动态参数列表，见下方说明               |

#### 4. `mouse_move` - 鼠标移动

| 字段            | 类型     | 必填  | 默认值 | 说明                    |
| ------------- | ------ | --- | --- | --------------------- |
| `type`        | string | 是   | -   | 固定为 `"mouse_move"`    |
| `x`           | number | 是   | -   | 目标 X 坐标               |
| `y`           | number | 是   | -   | 目标 Y 坐标               |
| `duration_ms` | number | 否   | `0` | 平滑移动过渡时长（毫秒），0 表示瞬间移动 |
| `backend`     | string | 否   | 继承  | 输入后端                  |
| `variables`    | array | 否   | -   | 动态参数列表，见下方说明               |

#### 5. `mouse_scroll` - 鼠标滚轮

| 字段          | 类型     | 必填  | 默认值 | 说明                              |
| ----------- | ------ | --- | --- | ------------------------------- |
| `type`      | string | 是   | -   | 固定为 `"mouse_scroll"`            |
| `direction` | string | 是   | -   | 滚动方向：`up`、`down`、`left`、`right` |
| `amount`    | number | 否   | `1` | 滚动量（Windows 系统中 120 = 1 格）      |
| `backend`   | string | 否   | 继承  | 输入后端                            |

#### 6. `delay` - 延迟等待

| 字段            | 类型     | 必填  | 默认值 | 说明            |
| ------------- | ------ | --- | --- | ------------- |
| `type`        | string | 是   | -   | 固定为 `"delay"` |
| `duration_ms` | number | 是   | -   | 等待时长（毫秒）      |

#### 7. `text` - 文本输入

| 字段        | 类型     | 必填  | 默认值 | 说明           |
| --------- | ------ | --- | --- | ------------ |
| `type`    | string | 是   | -   | 固定为 `"text"` |
| `content` | string | 是   | -   | 要输入的文本内容     |
| `backend` | string | 否   | 继承  | 输入后端         |

### 动态参数 (variables)

支持动态参数的动作类型（`mouse_move`、`mouse_click`）可以通过 `variables` 字段声明可动态覆盖的参数。

| 字段           | 类型     | 说明                     |
| ------------ | ------ | ---------------------- |
| `param_name` | string | 运行时传参时的参数名称           |
| `field_name` | string | 动作配置中要覆盖的字段名         |

**示例**：

```json
{
  "type": "mouse_move",
  "x": 0,
  "y": 0,
  "variables": [
    {"param_name": "target_x", "field_name": "x"},
    {"param_name": "target_y", "field_name": "y"}
  ]
}
```

**运行时不传参**：使用配置文件中的默认值 `x: 0, y: 0`

```json
{"execute": [true]}
```

**运行时传参**：通过 ZMQ `params` 动态覆盖

```json
{"execute": [true], "params": {"target_x": 500, "target_y": 300}}
```

### 输入后端 (InputBackend)

| 后端      | 说明                                       | 适用场景              |
| ------- | ---------------------------------------- | ----------------- |
| `win32` | Windows API (`PostMessageW`)，直接发送消息到目标窗口 | 无需前台窗口，可定向发送到后台窗口 |
| `enigo` | 跨平台库，依赖前台窗口                              | 需要硬件模拟的场景，如反作弊游戏  |

**后端优先级**：动作自身 `backend` > `default_backend`

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
- `"id:395542"` 或 `"id:0x9999"` - 直接指定 HWND（十进制或十六进制）
- `"class:Notepad"` - 窗口类名
- `"exe:notepad.exe"` - 进程名
- `"pid:1234"` - 进程 ID
- `"A"` 或空 - 当前前台窗口

### 执行时序参数汇总

| 参数                    | 说明        | 适用动作类型                                      |
| --------------------- | --------- | ------------------------------------------- |
| `hold_time_ms`        | 按住时长（毫秒）  | `key`, `key_sequence` 中的单个按键, `mouse_click` |
| `interval_ms`         | 间隔时长（毫秒）  | `key_sequence` 中的单个按键, `mouse_click`        |
| `default_interval_ms` | 序列按键的默认间隔 | `key_sequence`                              |
| `duration_ms`         | 平滑过渡时长    | `mouse_move`                                |
| `count`               | 连续点击次数    | `mouse_click`                               |
| `amount`              | 滚轮滚动量     | `mouse_scroll`（Windows 120 = 1 格）           |

## ZMQ 命令格式

Python 端通过 ZeroMQ PUSH-PULL 向 Rust 端发送控制命令。

### Python 辅助模块

参考实现 `example/python/otacle.py`：

```python
from otacle import OTAcleCommand

# 方式1: 上下文管理器（推荐）
with OTAcleCommand() as cmd:
    cmd.execute_by_indices([0, 2])      # 执行 index 0 和 2
    cmd.set_params({"target_x": 100, "target_y": 200})
    cmd.send()

# 方式2: 显式 close()
cmd = OTAcleCommand()
cmd.execute_by_indices([0])
cmd.send()
cmd.close()

# 便捷函数
from otacle import send_command
send_command([0, 2], {"target_x": 100})
```

### 连接复用

`OTAcleCommand` 内部维护持久连接，多次 `send()` 复用同一 ZMQ socket，无需每次创建销毁。

程序退出时自动通过 `atexit` 关闭连接，也支持 `with` 语句或显式 `close()`。

### 消息格式

```json
{
  "execute": [true, false, true, false, true],
  "params": {
    "target_x": 100,
    "target_y": 200,
    "click_count": 3
  }
}
```

| 字段        | 必填  | 类型          | 说明                                                      |
| --------- | --- | ----------- | ------------------------------------------------------- |
| `execute` | 是   | `Vec<bool>` | 执行列表，按 index 对应配置中的 action。<br>`true` = 执行，`false` = 跳过 |
| `params`  | 否   | `object`    | 动态参数映射，用于 variables 参数替换。如果无动态参数可省略。                        |

### execute 数组规则

- 长度任意，按 index 顺序对应配置文件中的 action
- `true` = 执行该动作，`false` = 跳过
- 缺失 index 视为 `false`

**示例**：`{"execute": [true, false, true]}` 表示执行 index=0 和 index=2 的动作。

### params 参数

`params` 中的键名对应动作 `variables` 中定义的 `param_name`：

```json
// 动作配置（test_actions.json）
{"type": "mouse_move", "x": 0, "y": 0, "variables": [
  {"param_name": "target_x", "field_name": "x"},
  {"param_name": "target_y", "field_name": "y"}
]}

// ZMQ 消息 - 传参
{"execute": [true], "params": {"target_x": 500, "target_y": 300}}

// ZMQ 消息 - 不传参，使用默认值
{"execute": [true]}
```

**约束**：

- `params` 可省略：表示不使用动态参数，所有动作使用配置文件中的默认值
- `params` 可为空对象 `{}`：效果同省略
- 支持数字和字符串类型的参数值

## 开发

```bash
# 前端开发
cd desktop && pnpm tauri dev

# Rust 检查
cd desktop/src-tauri && cargo check
```
