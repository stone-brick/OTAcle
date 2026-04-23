# OTAcle Python SDK

与 [OTAcle](https://github.com/yourname/otacle) 桌面应用通信的 Python 工具库。

## 安装

### 从本地目录安装（开发模式）

```bash
pip install -e /path/to/py_tool
```

### 构建 wheel 后安装

```bash
cd py_tool
pip install build
python -m build
pip install dist/otacle-0.1.0-py3-none-any.whl
```

## 配置

SDK 会自动从项目目录的 `.otacle/comm.json` 读取通信地址：

```json
{
  "act_pull_address": "tcp://127.0.0.1:5555",
  "observe_pub_address": "tcp://127.0.0.1:5556",
  "think_pull_address": "tcp://127.0.0.1:5557"
}
```

若配置文件不存在或字段缺失，使用硬编码默认值。

## 功能

| 类 | 用途 | ZMQ 模式 |
|---|---|---|
| `OTAcleCommand` | 发送动作命令 | PUSH → Act |
| `OTAcleObserver` | 接收图像帧 | SUB ← Observe |
| `OTAcleThinkSender` | 发送决策日志 | PUSH → Think |

## 使用示例

### Act 模块 - 发送动作命令

```python
from otacle import OTAcleCommand

with OTAcleCommand() as cmd:
    cmd.execute([0, 2])                      # 执行 index 0 和 2
    cmd.set_params({"target_x": 100, "target_y": 200})
    cmd.send()
```

### Observe 模块 - 接收图像帧

```python
from otacle import OTAcleObserver

with OTAcleObserver() as observer:
    for frame in observer:
        print(f"帧 {frame.frame_id}: {frame.width}x{frame.height}")
        # frame.data 是裁切块列表 CropBlock
```

### Think 模块 - 发送决策日志

```python
from otacle import OTAcleThinkSender

with OTAcleThinkSender() as sender:
    sender.send_step(1, reward=1.0, epsilon=0.1)
    sender.send_step(2, reward=0.5, loss=0.2)
```

### 便捷函数

```python
from otacle import send_command, send_decision_log

# 一次性发送动作命令
send_command([0, 1], {"target_x": 100, "target_y": 200})

# 一次性发送决策日志
send_decision_log(100, {"reward": 1.0, "epsilon": 0.1})
```

### 手动指定地址

覆盖配置文件中的地址：

```python
from otacle import OTAcleCommand

# 手动指定地址
cmd = OTAcleCommand(address="tcp://192.168.1.100:5555")
cmd.execute([0])
cmd.send()
```

### 手动读取配置

```python
from otacle import get_default_addresses, load_comm_config, find_project_root

# 获取项目根目录
root = find_project_root()
print(f"项目目录: {root}")

# 加载通信配置
config = load_comm_config()
print(f"Act 地址: {config.get('act_pull_address')}")

# 获取默认地址元组 (act, observe, think)
addrs = get_default_addresses()
print(f"全部地址: {addrs}")
```

## 数据类

| 类 | 说明 |
|---|---|
| `CropBlock` | 裁切块 `{x, y, w, h, image(base64)}` |
| `FrameMessage` | 图像帧 `{width, height, timestamp, frame_id, data[]}` |
| `DecisionLog` | 决策日志 `{step, custom}` |

## 许可证

MIT