# OTAcle Remote - 远程配置同步模块

基于 Spring Boot、阿里云 OSS 和 JWT 认证的远程配置同步系统，支持账号管理、分组共享和配置版本控制。

## 核心功能

1. **用户认证**: JWT Token 认证，支持注册和登录
2. **分组管理**: 创建分组、邀请成员、权限控制
3. **配置上传**: 将 .otacle 文件夹打包为 zip 上传到分组
4. **配置同步**: 获取分组内某角色的最新配置（签名 URL）
5. **版本管理**: 支持按版本精确获取和历史版本回滚
6. **分组共享**: 分组内配置对所有成员可见

## 技术栈

- **后端框架**: Spring Boot 3.4.3
- **数据库**: MySQL 8.0+
- **ORM**: MyBatis 3.0.3
- **对象存储**: 阿里云 OSS SDK 3.17.4
- **认证**: JWT (JJWT 0.12.5)
- **安全**: Spring Security
- **API 文档**: SpringDoc OpenAPI 2.3.0 (Swagger UI)
- **Java 版本**: 17+

## API 文档

### Swagger UI

启动应用后访问：
```
http://localhost:8080/swagger-ui.html
```

**功能**：
- 📖 交互式 API 文档
- 🧪 在线测试接口
- 🔑 支持 JWT Token 认证
- 📥 导出 OpenAPI 规范

### 导出 OpenAPI 文档

**Windows**:
```bash
export-openapi.bat
```

**Linux/Mac**:
```bash
chmod +x export-openapi.sh
./export-openapi.sh
```

**手动导出**:
```bash
# JSON 格式
curl http://localhost:8080/v3/api-docs -o openapi.json

# YAML 格式
curl http://localhost:8080/v3/api-docs.yaml -o openapi.yaml
```

### 导入到 Apifox

1. 打开 Apifox
2. 点击 **导入数据**
3. 选择 **文件导入**
4. 选择 `openapi.json`
5. 点击 **确定**

详见：[OPENAPI_GUIDE.md](OPENAPI_GUIDE.md)

## 快速开始

### 1. 环境要求

- JDK 17+
- Maven 3.6+
- MySQL 8.0+
- 阿里云 OSS 账号

### 2. 配置阿里云 OSS

编辑 `src/main/resources/application.yml`:

```yaml
aliyun:
  oss:
    endpoint: oss-cn-hangzhou.aliyuncs.com
    access-key-id: YOUR_ACCESS_KEY_ID
    access-key-secret: YOUR_ACCESS_KEY_SECRET
    bucket-name: your-bucket-name
```

### 3. 配置数据库

```yaml
spring:
  datasource:
    url: jdbc:mysql://localhost:3306/otacle_remote?useUnicode=true&characterEncoding=utf8&serverTimezone=Asia/Shanghai&useSSL=false&allowPublicKeyRetrieval=true
    username: root
    password: your_password
```

### 4. 配置 JWT

```yaml
jwt:
  secret: your-jwt-secret-key-change-this-in-production-min-32-chars
  expiration: 86400000 # 24小时
```

### 5. 初始化数据库

```bash
mysql -u root -p < src/main/resources/schema.sql
```

### 6. 启动应用

```bash
mvn spring-boot:run
```

应用将在 `http://localhost:8080` 启动。

## API 接口

### 认证接口

#### 注册
```
POST /api/auth/register
Content-Type: application/json

{
  "username": "user1",
  "password": "password123",
  "email": "user@example.com"
}
```

#### 登录
```
POST /api/auth/login
Content-Type: application/json

{
  "username": "user1",
  "password": "password123"
}

响应:
{
  "code": 200,
  "message": "success",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiJ9...",
    "userId": 1,
    "username": "user1"
  }
}
```

#### 获取当前用户信息
```
GET /api/auth/me
Authorization: Bearer {token}
```

### 分组接口

所有分组接口需要在 Header 中携带 JWT Token:
```
Authorization: Bearer {token}
```

#### 获取我的分组列表
```
GET /api/groups
```

#### 创建分组
```
POST /api/groups
Content-Type: application/json

{
  "name": "开发团队",
  "description": "项目开发分组"
}
```

#### 获取分组详情
```
GET /api/groups/{id}
```

#### 更新分组
```
PUT /api/groups/{id}
Content-Type: application/json

{
  "name": "新名称",
  "description": "新描述"
}
```

#### 删除分组
```
DELETE /api/groups/{id}
```

#### 获取分组成员
```
GET /api/groups/{id}/members
```

#### 加入分组（通过邀请码）
```
POST /api/groups/{id}/join
Content-Type: application/json

{
  "inviteCode": "ABC12345"
}
```

#### 离开分组
```
DELETE /api/groups/{id}/leave
```

### 配置接口

所有配置接口需要在 Header 中携带 JWT Token。

#### 上传配置 zip
```
POST /api/configs/upload
Content-Type: multipart/form-data

groupId: 1
characterId: char001
file: config.zip (必须包含 .otacle 目录)
```

**响应**:
```json
{
  "code": 200,
  "data": {
    "id": 1,
    "groupId": 1,
    "uploaderId": 1,
    "characterId": "char001",
    "version": 1,
    "filePath": "otacle-config/1/char001/1_v1_abc12345.zip",
    "fileName": "config.zip",
    "fileSize": 102400,
    "checksum": "sha256...",
    "createdAt": "2024-01-01T12:00:00"
  }
}
```

#### 获取最新版本配置
```
GET /api/configs/latest?groupId=1&characterId=char001
```

**响应**:
```json
{
  "code": 200,
  "data": {
    "configId": 1,
    "characterId": "char001",
    "version": 3,
    "fileName": "config.zip",
    "fileSize": 102400,
    "checksum": "sha256...",
    "downloadUrl": "https://bucket.oss.cn-hangzhou.aliyuncs.com/...?signature=...",
    "createdAt": "2024-01-03T12:00:00"
  }
}
```

#### 下载指定版本配置
```
GET /api/configs/download?groupId=1&characterId=char001&version=2
```

#### 获取版本历史
```
GET /api/configs/versions?groupId=1&characterId=char001
```

**响应**:
```json
{
  "code": 200,
  "data": [
    {
      "id": 3,
      "version": 3,
      "uploaderId": 1,
      "createdAt": "2024-01-03T12:00:00"
    },
    {
      "id": 2,
      "version": 2,
      "uploaderId": 2,
      "createdAt": "2024-01-02T12:00:00"
    }
  ]
}
```

## 数据库设计

### account (账号表)
- id, username, password (加密), email, created_at, updated_at

### group (分组表)
- id, name, description, invite_code, created_by, created_at, updated_at

### group_member (组成员关系表)
- id, account_id, group_id, role (admin/member), joined_at

### config_version (配置版本表)
- id, group_id, uploader_id, character_id, version, file_path, file_name, file_size, checksum, created_at

**唯一索引**: (group_id, character_id, version)

## OSS 存储结构

```
otacle-config/
└── {group_id}/
    └── {character_id}/
        └── {group_id}_v{version}_{uuid}.zip
```

示例: `otacle-config/1/char001/1_v1_abc12345.zip`

## 使用流程

### 1. 注册和登录
```javascript
// 注册
const registerRes = await fetch('http://localhost:8080/api/auth/register', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    username: 'user1',
    password: 'password123',
    email: 'user@example.com'
  })
});

// 登录
const loginRes = await fetch('http://localhost:8080/api/auth/login', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    username: 'user1',
    password: 'password123'
  })
});

const { token } = loginRes.data;
```

### 2. 创建分组
```javascript
const groupRes = await fetch('http://localhost:8080/api/groups', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${token}`
  },
  body: JSON.stringify({
    name: '开发团队',
    description: '项目开发分组'
  })
});

const groupId = groupRes.data.id;
const inviteCode = groupRes.data.inviteCode;
```

### 3. 其他用户加入分组
```javascript
await fetch(`http://localhost:8080/api/groups/${groupId}/join`, {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${otherUserToken}`
  },
  body: JSON.stringify({
    inviteCode: inviteCode
  })
});
```

### 4. 上传配置
```javascript
const formData = new FormData();
formData.append('groupId', groupId);
formData.append('characterId', 'char001');
formData.append('file', zipFile); // 包含 .otacle 目录的 zip 文件

await fetch('http://localhost:8080/api/configs/upload', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${token}`
  },
  body: formData
});
```

### 5. 获取最新配置并下载
```javascript
// 获取签名 URL
const latestRes = await fetch(
  `http://localhost:8080/api/configs/latest?groupId=${groupId}&characterId=char001`,
  {
    headers: {
      'Authorization': `Bearer ${otherUserToken}`
    }
  }
);

const { downloadUrl, version } = latestRes.data;

// 从 OSS 下载
const fileRes = await fetch(downloadUrl);
const blob = await fileRes.blob();

// 解压并使用
const zip = new JSZip();
const content = await zip.loadAsync(blob);
const otacleDir = content.folder('.otacle');
// 处理配置内容...
```

## 项目结构

```
remote/
├── src/main/java/com/otacle/remote/
│   ├── RemoteApplication.java
│   ├── config/
│   │   ├── OssConfig.java
│   │   ├── SecurityConfig.java
│   │   └── WebSecurityConfig.java
│   ├── controller/
│   │   ├── AuthController.java
│   │   ├── GroupController.java
│   │   └── ConfigController.java
│   ├── service/
│   │   ├── AuthService.java
│   │   ├── GroupService.java
│   │   ├── ConfigService.java
│   │   └── OssService.java
│   ├── repository/
│   │   ├── AccountMapper.java
│   │   ├── GroupMapper.java
│   │   ├── GroupMemberMapper.java
│   │   └── ConfigVersionMapper.java
│   ├── model/
│   │   ├── Account.java
│   │   ├── Group.java
│   │   ├── GroupMember.java
│   │   └── ConfigVersion.java
│   ├── dto/
│   │   ├── ApiResponse.java
│   │   ├── auth/
│   │   └── config/
│   └── security/
│       └── JwtAuthenticationFilter.java
└── src/main/resources/
    ├── application.yml
    └── schema.sql
```

## 安全特性

1. **密码加密**: 使用 BCrypt 加密存储
2. **JWT 认证**: Stateless 认证，有效期 24 小时
3. **权限控制**: 分组成员验证，管理员权限检查
4. **文件校验**: SHA-256 校验和验证文件完整性
5. **签名 URL**: OSS 签名 URL 有效期 1 小时

## 注意事项

1. **生产环境**: 
   - 修改 JWT secret 为强随机字符串
   - 启用 HTTPS
   - 配置 CORS 白名单
   - 不要将 AccessKey 提交到版本控制

2. **文件大小**: 默认限制 50MB，可在 application.yml 中调整

3. **zip 格式**: 上传的文件必须包含 `.otacle` 目录

4. **版本管理**: 版本号自动递增，同一分组+角色组合的版本号唯一

## 许可证

MIT License
