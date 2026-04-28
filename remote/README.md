# OTAcle SpringBoot Backend

OTAcle 远程后端服务，提供 REST API 和 WebSocket 接口，支持分布式部署和远程协作。

## 功能特性

### 1. REST API
- **项目管理**: CRUD 操作
- **动作配置管理**: 创建、查询、更新、删除动作配置
- **决策日志查询**: 按项目、步骤范围查询历史日志
- **图像帧查询**: 获取历史图像帧数据

### 2. WebSocket 实时通信
- **Observe 模块**: 接收和广播图像帧（`/app/observe/frame` → `/topic/observe/frames`）
- **Think 模块**: 接收和广播决策日志（`/app/think/log` → `/topic/think/logs`）
- **Act 模块**: 接收和广播动作命令（`/app/act/command` → `/topic/act/commands`）

### 3. 数据存储
- H2 数据库（开发环境）
- PostgreSQL（生产环境，需配置）
- JPA + Hibernate ORM

## 技术栈

- **框架**: Spring Boot 4.0.5
- **语言**: Java 21
- **数据库**: H2 / PostgreSQL
- **WebSocket**: STOMP over WebSocket
- **构建工具**: Maven

## 快速开始

### 前置要求

- JDK 21+
- Maven 3.6+
- （可选）PostgreSQL 12+

### 启动后端

```bash
cd remote
mvn spring-boot:run
```

后端将在 `http://localhost:8080` 启动。

### 访问 H2 控制台

打开浏览器访问：`http://localhost:8080/h2-console`

- JDBC URL: `jdbc:h2:file:./data/otacle`
- 用户名: `sa`
- 密码: (空)

## API 端点

### 项目管理

```http
GET    /api/projects              # 获取所有项目
GET    /api/projects/{id}         # 获取指定项目
POST   /api/projects              # 创建项目
PUT    /api/projects/{id}         # 更新项目
DELETE /api/projects/{id}         # 删除项目
```

**创建项目示例:**
```json
POST /api/projects
{
  "name": "My Project",
  "description": "Project description",
  "projectPath": "/path/to/project"
}
```

### 动作配置管理

```http
GET    /api/actions                    # 获取所有动作
GET    /api/actions/{id}               # 获取指定动作
GET    /api/actions/project/{projectId} # 获取项目的所有动作
POST   /api/actions                    # 创建动作
PUT    /api/actions/{id}               # 更新动作
DELETE /api/actions/{id}               # 删除动作
```

**创建动作示例:**
```json
POST /api/actions
{
  "name": "Space Key",
  "projectId": 1,
  "actionType": "key",
  "config": "{\"key\": \"space\"}",
  "variables": "[]"
}
```

### 决策日志（Think 模块）

```http
GET    /api/think                              # 获取所有日志
GET    /api/think/{id}                         # 获取指定日志
GET    /api/think/project/{projectId}          # 获取项目的日志
GET    /api/think/project/{projectId}/steps    # 按步骤范围查询
POST   /api/think                              # 创建日志
POST   /api/think/batch                        # 批量创建日志
```

**查询步骤范围示例:**
```http
GET /api/think/project/1/steps?startStep=1&endStep=100
```

### 图像帧（Observe 模块）

```http
GET    /api/observe                              # 获取所有帧
GET    /api/observe/{id}                         # 获取指定帧
GET    /api/observe/project/{projectId}          # 获取项目的帧
GET    /api/observe/project/{projectId}/latest   # 获取最新帧
POST   /api/observe                              # 创建帧
```

### 动作命令（Act 模块）

```http
GET    /api/act                              # 获取所有命令
GET    /api/act/{id}                         # 获取指定命令
GET    /api/act/project/{projectId}          # 获取项目的命令
POST   /api/act                              # 创建命令
```

## WebSocket 使用

### 连接端点

```
ws://localhost:8080/ws
```

### 订阅主题

```javascript
// 订阅图像帧
stompClient.subscribe('/topic/observe/frames', callback);

// 订阅决策日志
stompClient.subscribe('/topic/think/logs', callback);

// 订阅动作命令
stompClient.subscribe('/topic/act/commands', callback);
```

### 发送消息

```javascript
// 发送图像帧
stompClient.send('/app/observe/frame', {}, JSON.stringify({
  frameId: 1,
  width: 800,
  height: 600,
  timestamp: Date.now(),
  data: [...],
  projectId: 1
}));

// 发送决策日志
stompClient.send('/app/think/log', {}, JSON.stringify({
  step: 1,
  custom: { reward: 1.0, epsilon: 0.1 },
  projectId: 1
}));

// 发送动作命令
stompClient.send('/app/act/command', {}, JSON.stringify({
  execute: [true, false, true],
  params: { target_x: 100, target_y: 200 },
  projectId: 1
}));
```

## Python 客户端示例

### 安装依赖

```bash
pip install requests websocket-client
```

### 运行演示

```bash
cd example
python demo_remote.py
```

### 代码示例

```python
from demo_remote import OTAcleRestClient, OTAcleWebSocketClient

# REST API 客户端
rest = OTAcleRestClient()
project = rest.create_project("My Project")
actions = rest.get_actions(project['id'])

# WebSocket 客户端
ws = OTAcleWebSocketClient()
ws.connect()
ws.send_think_log(step=1, custom={"reward": 1.0}, project_id=project['id'])
ws.close()
```

## 与 ZeroMQ 架构对比

| 特性 | ZeroMQ (本地) | SpringBoot (远程) |
|------|--------------|-------------------|
| 通信方式 | PUSH-PULL / PUB-SUB | REST API + WebSocket |
| 部署模式 | 单机 | 分布式 |
| 数据持久化 | 无 | 数据库存储 |
| 跨语言支持 | 需要 ZMQ 绑定 | HTTP/WebSocket 标准协议 |
| 安全性 | 无 | JWT + Spring Security (可扩展) |
| 适用场景 | 本地开发、低延迟 | 远程协作、数据持久化 |

## 混合架构建议

可以同时使用 ZeroMQ 和 SpringBoot：

1. **本地高性能场景**: 使用 ZeroMQ（原有架构）
2. **数据持久化和远程访问**: 使用 SpringBoot
3. **桌面应用作为网关**: Tauri 应用同时连接 ZMQ 和 SpringBoot，负责数据转发

## 配置说明

### 切换数据库

编辑 `src/main/resources/application.properties`:

```properties
# PostgreSQL 配置
spring.datasource.url=jdbc:postgresql://localhost:5432/otacle
spring.datasource.username=postgres
spring.datasource.password=your_password
spring.jpa.database-platform=org.hibernate.dialect.PostgreSQLDialect
```

### 修改端口

```properties
server.port=8080
```

### CORS 配置

默认允许所有来源，生产环境应限制：

```java
// CorsConfig.java
.allowedOriginPatterns("https://your-domain.com")
```

## 开发指南

### 项目结构

```
remote/src/main/java/com/otacle/remote/
├── controller/          # REST 控制器
├── service/             # 业务逻辑层
├── repository/          # 数据访问层
├── model/               # JPA 实体
├── dto/                 # 数据传输对象
├── websocket/           # WebSocket 控制器和消息类
└── config/              # 配置类
```

### 添加新功能

1. 创建 Entity: `model/YourEntity.java`
2. 创建 Repository: `repository/YourRepository.java`
3. 创建 Service: `service/YourService.java`
4. 创建 Controller: `controller/YourController.java`
5. （可选）创建 WebSocket: `websocket/YourWebSocketController.java`

## 故障排除

### 端口被占用

修改 `application.properties`:
```properties
server.port=8081
```

### 数据库连接失败

检查 H2 文件路径或 PostgreSQL 配置是否正确。

### WebSocket 连接失败

确保防火墙允许 WebSocket 端口，检查 CORS 配置。

## 许可证

MIT License
