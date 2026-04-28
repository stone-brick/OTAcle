# OTAcle 混合架构说明

本文档说明如何将 ZeroMQ 本地架构与 SpringBoot 远程架构结合使用。

## 架构对比

### 1. ZeroMQ 架构（原有）

```
┌─────────────┐         ZMQ          ┌──────────────┐
│  Python     │ ◄─────────────────► │  Tauri App   │
│  (AI Agent) │    PUSH-PULL/PUB    │  (Desktop)   │
└─────────────┘       -SUB           └──────────────┘
                         │
                    Localhost only
                   (5555/5556/5557)
```

**优点:**
- ✅ 低延迟（微秒级）
- ✅ 简单直接
- ✅ 无需额外服务

**缺点:**
- ❌ 仅限本地通信
- ❌ 无数据持久化
- ❌ 无法远程协作

### 2. SpringBoot 架构（新增）

```
┌─────────────┐      HTTP/WS       ┌──────────────┐      ZMQ       ┌──────────────┐
│  Python     │ ◄───────────────► │ SpringBoot   │ ◄───────────► │  Tauri App   │
│  (AI Agent) │   REST + WebSocket │  Backend     │   Optional   │  (Desktop)   │
└─────────────┘                    └──────────────┘              └──────────────┘
                                          │
                                    ┌──────────────┐
                                    │  Database    │
                                    │  (H2/PGSQL)  │
                                    └──────────────┘
```

**优点:**
- ✅ 支持分布式部署
- ✅ 数据持久化
- ✅ 可远程访问
- ✅ 标准 HTTP/WebSocket 协议

**缺点:**
- ❌ 延迟较高（毫秒级）
- ❌ 需要额外服务
- ❌ 复杂度增加

## 推荐方案：混合架构

### 方案 A：桌面应用作为网关（推荐）

```
┌─────────────┐      ZMQ         ┌──────────────┐      HTTP/WS     ┌──────────────┐
│  Python     │ ◄─────────────► │  Tauri App   │ ◄─────────────► │ SpringBoot   │
│  (AI Agent) │   Low Latency    │  (Gateway)   │   Persistence  │  Backend     │
└─────────────┘                  └──────────────┘                └──────────────┘
                                       │                                │
                                  Local execution                 Cloud/Remote DB
```

**工作流程:**
1. Python 通过 ZMQ 发送指令到 Tauri（低延迟执行）
2. Tauri 同时转发数据到 SpringBoot（持久化存储）
3. 其他客户端可通过 HTTP/WS 查询历史数据

**实现步骤:**

在 Tauri Rust 后端添加 SpringBoot 客户端：

```rust
// desktop/src-tauri/src/communication/backend_client.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct BackendClient {
    client: Client,
    base_url: String,
}

impl BackendClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }
    
    pub async fn send_decision_log(&self, step: i32, custom: serde_json::Value) -> Result<(), Error> {
        self.client
            .post(format!("{}/api/think", self.base_url))
            .json(&serde_json::json!({
                "step": step,
                "custom": custom
            }))
            .send()
            .await?;
        Ok(())
    }
}
```

### 方案 B：Python 双通道发送

```
┌─────────────┐      ZMQ         ┌──────────────┐
│  Python     │ ──────────────► │  Tauri App   │
│  (AI Agent) │   (Execution)   │  (Desktop)   │
│             │                                  
│             │      HTTP/WS     ┌──────────────┐
└───────────┼─► │ SpringBoot   │
            │   │  Backend     │
            │   └──────────────┘
         Dual Send
```

**工作流程:**
1. Python 通过 ZMQ 发送执行指令（快速响应）
2. Python 同时通过 HTTP/WS 发送日志到 SpringBoot（异步记录）

**示例代码:**

```python
from otacle import OTAcleCommand, OTAcleThinkSender
import requests

# ZMQ for execution (low latency)
cmd = OTAcleCommand()
cmd.execute([0])
cmd.send()

# HTTP for logging (async)
requests.post("http://localhost:8080/api/think", json={
    "step": 1,
    "custom": {"reward": 1.0}
})
```

### 方案 C：纯 SpringBoot 架构

```
┌─────────────┐      HTTP/WS     ┌──────────────┐      HTTP/WS     ┌──────────────┐
│  Python     │ ◄─────────────► │ SpringBoot   │ ◄─────────────► │  Web/Mobile  │
│  (AI Agent) │   Full Stack     │  Backend     │   Clients      │  Clients     │
└─────────────┘                  └──────────────┘                └──────────────┘
```

**适用场景:**
- 完全云端部署
- 多用户协作
- 跨平台访问

## 配置指南

### 启用混合模式

1. **启动 SpringBoot 后端:**
```bash
cd remote
mvn spring-boot:run
```

2. **配置 Tauri 应用连接后端:**

编辑 `desktop/src/utils/config.js`:
```javascript
export const BACKEND_CONFIG = {
  enabled: true,
  baseUrl: 'http://localhost:8080',
  autoSync: true  // 自动同步数据到后端
};
```

3. **修改 Python SDK 支持双通道:**

编辑 `py_tool/src/otacle/otacle.py`:
```python
class OTAcleThinkSender:
    def __init__(self, address=None, backend_url=None):
        # ZMQ connection
        self._address = address or get_default_addresses()[2]
        
        # Backend connection (optional)
        self._backend_url = backend_url
        if self._backend_url:
            import requests
            self._backend_session = requests.Session()
    
    def send_step(self, step: int, **kwargs):
        # Send via ZMQ (original)
        self.send(DecisionLog(step=step, custom=kwargs))
        
        # Send to backend (if configured)
        if self._backend_url:
            try:
                self._backend_session.post(
                    f"{self._backend_url}/api/think",
                    json={"step": step, "custom": kwargs}
                )
            except Exception as e:
                print(f"Backend sync failed: {e}")
```

## 性能对比

| 指标 | ZeroMQ | SpringBoot REST | SpringBoot WS |
|------|--------|-----------------|---------------|
| 延迟 | ~10μs | ~5-20ms | ~1-5ms |
| 吞吐量 | 100K+ msg/s | 1-5K req/s | 10-50K msg/s |
| 持久化 | ❌ | ✅ | ✅ |
| 可靠性 | 中等 | 高 | 中等 |
| 复杂度 | 低 | 中 | 中 |

## 最佳实践

### 1. 本地开发
- 使用 ZeroMQ（高性能）
- 可选启用 SpringBoot（调试和数据查看）

### 2. 团队协作
- 启用 SpringBoot 后端
- 所有成员连接到同一后端
- 共享项目和决策日志

### 3. 生产部署
- 使用 PostgreSQL 替代 H2
- 添加 JWT 认证
- 启用 HTTPS
- 使用负载均衡器

### 4. 数据同步策略

```python
# 异步批量同步
class BatchSyncer:
    def __init__(self, backend_url, batch_size=100, interval=5):
        self.backend_url = backend_url
        self.batch_size = batch_size
        self.interval = interval
        self.buffer = []
    
    def add_log(self, log):
        self.buffer.append(log)
        if len(self.buffer) >= self.batch_size:
            self.flush()
    
    def flush(self):
        if self.buffer:
            requests.post(
                f"{self.backend_url}/api/think/batch",
                json=self.buffer
            )
            self.buffer.clear()
```

## 故障排除

### ZMQ 和后端数据不一致

**原因:** 网络延迟或后端服务不可用

**解决方案:**
1. 实现重试机制
2. 使用本地缓存队列
3. 定期检查同步状态

### 后端性能瓶颈

**优化建议:**
1. 使用连接池
2. 启用数据库索引
3. 添加 Redis 缓存
4. 使用批量操作

## 总结

- **追求极致性能**: 使用 ZeroMQ
- **需要数据持久化**: 使用 SpringBoot
- **最佳方案**: 混合架构，ZMQ 用于执行，SpringBoot 用于存储和协作

根据实际需求选择合适的架构或组合使用。
