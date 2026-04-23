# OTAcle Example

本目录包含 OTAcle 的功能演示，用于展示 Observe → Think → Act 闭环。

## 快速开始

### 1. 启动 OTAcle 桌面应用

```bash
cd desktop
pnpm tauri dev
```

### 2. 加载项目

在桌面应用中：
1. 点击"加载项目"
2. 选择 `example` 目录

### 3. 配置并启动各模块

**Observe 模块（截图采集）：**
1. 打开 Observe 页面
2. 选择目标窗口（需要截图的窗口）
3. 点击"开始捕获"

**Think 模块（决策日志可视化）：**
1. 打开 Think 页面
2. 点击"开始监听"

**Act 模块（动作执行）：**
1. 打开 Act 页面
2. 查看动作列表（已配置 1 个 space 键动作）
3. 目标窗口设为需要执行动作的窗口

### 4. 运行演示脚本

```bash
python demo.py
```

脚本将每秒钟：
- 通过 Think 模块发送一条决策日志（step、reward、epsilon、loss）
- 通过 Act 模块发送一个动作命令（执行 space 键）

### 5. 观察结果

- **Think 页面**：应看到决策图表随时间更新（reward 逐渐减小，epsilon 逐渐衰减）
- **目标窗口**：每秒执行一次 space 键点击

## 目录结构

```
example/
├── demo.py              # 演示脚本
├── .otacle/             # OTAcle 配置
│   ├── actions.json     # 动作配置（1 个 space 键动作）
│   ├── observe.json     # Observe 配置（135x256，2 个裁切区域）
│   ├── think.json       # Think 配置
│   └── comm.json        # ZMQ 通信地址配置
└── README.md            # 本文档
```

## 配置文件说明

### actions.json

```json
{
  "default_backend": "enigo",
  "actions": [
    {
      "name": null,
      "type": "key",
      "key": "space",
      "hold_time_ms": 5,
      "backend": null
    }
  ]
}
```

定义了 1 个动作：按下 space 键，保持 5ms。

### observe.json

```json
{
  "capture": {
    "frame_rate": 3,
    "target_width": 135,
    "target_height": 256
  },
  "crop_regions": [
    {"x": 10, "y": 40, "w": 115, "h": 18},
    {"x": 0, "y": 100, "w": 135, "h": 110}
  ]
}
```

配置了 2 个裁切区域，用于从目标窗口截取特定区域。

### think.json

```json
{
  "max_data_points": 500,
  "display_fields": [
    {"name": "step", "title": "训练步数", "chart_type": "line"}
  ]
}
```

配置了决策日志的可视化显示。

### comm.json

```json
{
  "act_pull_address": "tcp://127.0.0.1:5555",
  "observe_pub_address": "tcp://127.0.0.1:5556",
  "think_pull_address": "tcp://127.0.0.1:5557"
}
```

ZMQ 通信地址配置。

## 自定义演示

### 添加更多动作

编辑 `.otacle/actions.json`，添加更多动作配置：

```json
{
  "actions": [
    {"name": "jump", "type": "key", "key": "space"},
    {"name": "left", "type": "key", "key": "a"},
    {"name": "right", "type": "key", "key": "d"}
  ]
}
```

然后在 `demo.py` 中修改 `cmd.execute([0])` 为 `cmd.execute([0, 1])` 来执行多个动作。

### 修改决策日志内容

在 `demo.py` 中修改 `sender.send_step()` 的参数：

```python
sender.send_step(
    step,
    reward=reward,
    epsilon=epsilon,
    action=0,
    loss=0.5 / step,
    # 添加更多自定义字段
    q_value=0.9,
    policy="epsilon_greedy",
)
```

### 调整执行间隔

修改 `time.sleep(1)` 为其他值（单位：秒）。

## 故障排除

### 连接失败

确保 OTAcle 桌面应用已启动，且各模块已正确启动（点击"开始"按钮）。

### 动作未执行

1. 检查目标窗口是否设置正确
2. 检查 actions.json 中的动作配置是否有效
3. 尝试切换输入后端（win32 或 enigo）

### 决策日志未显示

1. 确保 Think 模块已启动（点击"开始监听"）
2. 检查 comm.json 中的 think_pull_address 是否正确