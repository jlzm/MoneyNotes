# Money Notes 服务端架构文档

> 本文档面向 Rust 初学者，从系统层面理解项目架构，帮助你在遇到问题时知道去哪里找解决方案。

## 目录

1. [项目总览](#1-项目总览)
2. [分层架构](#2-分层架构)
3. [核心模块详解](#3-核心模块详解)
4. [关键技术点](#4-关键技术点)
5. [数据流向](#5-数据流向)
6. [常见问题定位指南](#6-常见问题定位指南)
7. [Rust 知识点索引](#7-rust-知识点索引)

---

## 1. 项目总览

### 1.1 这是什么项目？

一个记账应用的后端服务，支持：
- 用户认证（注册、登录、JWT 令牌）
- 个人账本和群组账本
- 收入/支出账单管理
- 分类管理
- 统计分析

### 1.2 技术栈一览

| 类别 | 技术 | 作用 | 你需要知道的 |
|------|------|------|-------------|
| Web 框架 | Axum 0.7 | 处理 HTTP 请求 | 路由、提取器、中间件 |
| 异步运行时 | Tokio | 异步 I/O | `async/await`、`#[tokio::main]` |
| 数据库 | SQLx + MySQL | 数据持久化 | 连接池、异步查询 |
| 认证 | JWT | 身份验证 | Token 生成与验证 |
| 密码 | Argon2 | 密码哈希 | 安全存储密码 |
| 序列化 | Serde | JSON 转换 | `#[derive(Serialize, Deserialize)]` |

### 1.3 目录结构

```
server/
├── Cargo.toml              # 依赖配置（类似 package.json）
├── config/                 # 配置文件
│   ├── default.toml       # 默认配置
│   ├── development.toml   # 开发环境
│   └── production.toml    # 生产环境
├── migrations/            # 数据库迁移脚本
└── src/
    ├── main.rs            # 程序入口
    ├── lib.rs             # 库入口
    ├── api/               # HTTP 接口层
    ├── models/            # 数据模型
    ├── repositories/      # 数据访问层
    ├── services/          # 业务逻辑层
    ├── dto/               # 数据传输对象
    ├── middleware/        # 中间件
    ├── error/             # 错误处理
    ├── config/            # 配置加载
    └── utils/             # 工具函数
```

---

## 2. 分层架构

### 2.1 架构图

```
┌─────────────────────────────────────────────────────────┐
│                      HTTP 请求                          │
└─────────────────────────┬───────────────────────────────┘
                          ▼
┌─────────────────────────────────────────────────────────┐
│  中间件层 (middleware/)                                  │
│  - CORS 跨域处理                                        │
│  - 日志追踪                                             │
│  - JWT 认证 → 提取当前用户                              │
└─────────────────────────┬───────────────────────────────┘
                          ▼
┌─────────────────────────────────────────────────────────┐
│  API 层 (api/)                                          │
│  - 接收请求，验证参数                                   │
│  - 调用 Service 或 Repository                          │
│  - 返回 JSON 响应                                       │
└─────────────────────────┬───────────────────────────────┘
                          ▼
┌─────────────────────────────────────────────────────────┐
│  业务逻辑层 (services/)                                 │
│  - 复杂业务逻辑（如：注册 = 检查邮箱 + 哈希密码 + 创建）│
│  - 跨多个 Repository 的操作                            │
└─────────────────────────┬───────────────────────────────┘
                          ▼
┌─────────────────────────────────────────────────────────┐
│  数据访问层 (repositories/)                             │
│  - 单一实体的 CRUD 操作                                │
│  - SQL 查询封装                                        │
│  - 支持多种数据库实现                                  │
└─────────────────────────┬───────────────────────────────┘
                          ▼
┌─────────────────────────────────────────────────────────┐
│  数据库 (MySQL)                                         │
└─────────────────────────────────────────────────────────┘
```

### 2.2 为什么要分层？

**当出问题时，你能快速定位：**

| 问题类型 | 去哪里找 |
|---------|---------|
| 路由 404、参数解析错误 | `api/routes.rs` |
| 权限问题、Token 无效 | `middleware/auth.rs` |
| 业务逻辑错误 | `services/` 或 `api/` 中的处理函数 |
| 数据库查询错误 | `repositories/mysql/` |
| 数据格式问题 | `dto/` 或 `models/` |

---

## 3. 核心模块详解

### 3.1 入口文件 `main.rs`

```rust
#[tokio::main]  // 宏：让 main 函数支持异步
async fn main() {
    // 1. 初始化日志
    // 2. 加载配置
    // 3. 连接数据库
    // 4. 初始化各层组件
    // 5. 启动 HTTP 服务器
}
```

**遇到启动问题时：** 从这里开始排查，看是哪一步失败。

### 3.2 API 层 (`api/`)

每个文件对应一组 API：

| 文件 | 职责 | 主要接口 |
|------|------|---------|
| `auth.rs` | 认证 | 注册、登录、刷新 Token |
| `user.rs` | 用户 | 获取/更新个人信息 |
| `ledger.rs` | 账本 | 增删改查账本 |
| `bill.rs` | 账单 | 增删改查账单、统计 |
| `category.rs` | 分类 | 增删改查分类 |
| `group.rs` | 群组 | 群组管理、成员管理 |

**典型 API 处理函数结构：**

```rust
pub async fn create_bill(
    State(state): State<AppState>,              // 应用状态
    Extension(current_user): Extension<CurrentUser>, // 当前用户
    Json(req): Json<CreateBillRequest>,         // 请求体
) -> Result<Json<ApiResponse<BillResponse>>, AppError> {
    // 1. 参数验证
    // 2. 权限检查
    // 3. 调用 Repository 执行操作
    // 4. 返回响应
}
```

### 3.3 Repository 层 (`repositories/`)

**设计模式：Repository Pattern**

```
repositories/
├── traits/           # 接口定义（抽象）
│   ├── user_repository.rs
│   ├── bill_repository.rs
│   └── ...
├── mysql/            # MySQL 实现
├── mongodb/          # MongoDB 实现（预留）
└── memory/           # 内存实现（测试用）
```

**为什么这样设计？**
- 数据库可替换：想换 PostgreSQL？只需新增一个实现
- 方便测试：测试时用内存实现，不依赖真实数据库

**Trait 示例：**

```rust
#[async_trait]  // 因为 trait 的方法是异步的
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: CreateUser) -> AppResult<User>;
    async fn find_by_id(&self, id: &str) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    // ...
}
```

### 3.4 数据模型 (`models/`)

定义业务实体：

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**关键 derive 宏：**
- `Serialize, Deserialize`：JSON 转换
- `sqlx::FromRow`：数据库行 → 结构体

### 3.5 DTO 层 (`dto/`)

**DTO = Data Transfer Object**

分离内部模型和外部接口：

```rust
// 请求 DTO（用于接收客户端数据）
#[derive(Deserialize, Validate)]
pub struct CreateBillRequest {
    #[validate(length(min = 1))]
    pub ledger_id: String,
    pub amount: f64,
    // ...
}

// 响应 DTO（用于返回给客户端）
#[derive(Serialize)]
pub struct BillResponse {
    pub id: String,
    pub amount: f64,
    pub category: CategoryBriefResponse,  // 嵌套
    // ...
}
```

**为什么不直接用 Model？**
- 安全：不暴露 `password_hash` 等敏感字段
- 灵活：响应可以组合多个模型的数据
- 验证：请求 DTO 可以添加验证规则

### 3.6 错误处理 (`error/`)

统一的错误类型：

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("验证错误: {0}")]
    Validation(String),      // 400

    #[error("未授权")]
    Unauthorized,            // 401

    #[error("禁止访问: {0}")]
    Forbidden(String),       // 403

    #[error("未找到: {0}")]
    NotFound(String),        // 404

    #[error("内部错误: {0}")]
    Internal(String),        // 500
    // ...
}
```

**好处：** 使用 `?` 操作符自动传播错误，最终统一转换为 HTTP 响应。

---

## 4. 关键技术点

### 4.1 异步编程 (Async/Await)

```rust
// async 函数返回 Future
async fn fetch_user(id: &str) -> Result<User, Error> {
    // await 等待异步操作完成
    let user = db.query("SELECT ...").await?;
    Ok(user)
}
```

**要点：**
- `async fn` 定义异步函数
- `.await` 等待 Future 完成
- Tokio 是异步运行时，负责调度

### 4.2 Trait 与泛型

```rust
// 定义接口
#[async_trait]
pub trait Repository<T> {
    async fn find_by_id(&self, id: &str) -> Option<T>;
}

// 实现接口
impl Repository<User> for MySqlUserRepository {
    async fn find_by_id(&self, id: &str) -> Option<User> {
        // 具体实现
    }
}
```

### 4.3 智能指针

| 类型 | 用途 | 项目中的使用 |
|------|------|-------------|
| `Arc<T>` | 线程安全的共享所有权 | Repository 实例共享 |
| `Box<dyn Trait>` | Trait 对象（动态分发） | - |
| `Arc<dyn Trait>` | 共享的 Trait 对象 | `Arc<dyn UserRepository>` |

```rust
// 多个 API 共享同一个 Repository 实例
let user_repo = Arc::new(MySqlUserRepository::new(pool.clone()));
```

### 4.4 错误处理

```rust
// Result 类型
type AppResult<T> = Result<T, AppError>;

// ? 操作符：遇到 Err 提前返回
async fn get_user(id: &str) -> AppResult<User> {
    let user = repo.find_by_id(id).await?  // 失败则返回 Err
        .ok_or(AppError::NotFound("用户不存在".into()))?;
    Ok(user)
}
```

### 4.5 宏 (Macros)

项目中常用的宏：

| 宏 | 来源 | 作用 |
|---|------|------|
| `#[derive(...)]` | Rust 标准 | 自动实现 trait |
| `#[tokio::main]` | Tokio | 异步 main 函数 |
| `#[async_trait]` | async-trait | 异步 trait 方法 |
| `#[serde(rename_all = "camelCase")]` | Serde | JSON 字段命名 |

---

## 5. 数据流向

### 5.1 请求处理流程（以创建账单为例）

```
1. POST /api/v1/bills
   │
2. ├─ CORS 中间件 → 允许跨域
   │
3. ├─ Auth 中间件 → 验证 JWT，提取用户 ID
   │
4. ├─ Axum 路由 → 匹配到 create_bill 处理函数
   │
5. ├─ 请求解析 → JSON body → CreateBillRequest
   │
6. ├─ 参数验证 → validator 检查
   │
7. ├─ 权限检查 → 用户是否有权操作该账本
   │
8. ├─ 调用 BillRepository::create()
   │
9. ├─ 执行 SQL INSERT
   │
10.├─ 转换响应 → Bill → BillResponse
   │
11.└─ 返回 JSON
```

### 5.2 模块间依赖关系

```
main.rs
   │
   ├── 创建 → MySqlPool (数据库连接池)
   │              │
   ├── 创建 → Repositories (注入 Pool)
   │              │
   ├── 创建 → Services (注入 Repositories)
   │              │
   ├── 创建 → APIs (注入 Services + Repositories)
   │              │
   └── 组装 → AppState → 传递给路由
```

---

## 6. 常见问题定位指南

### 6.1 编译错误

| 错误类型 | 可能原因 | 解决方向 |
|---------|---------|---------|
| `trait bound not satisfied` | 缺少 derive 或 trait 实现 | 检查是否 derive 了必要的 trait |
| `lifetime error` | 引用生命周期问题 | 考虑使用 `'static`、`Clone` 或 `Arc` |
| `type mismatch` | 类型不匹配 | 检查函数签名和返回值 |
| `cannot find` | 模块/类型未导入 | 检查 `mod.rs` 和 `use` 语句 |

### 6.2 运行时错误

| 错误现象 | 去哪里查 | 常见原因 |
|---------|---------|---------|
| 连接数据库失败 | `config/*.toml` | 数据库 URL 配置错误 |
| 401 Unauthorized | `middleware/auth.rs` | JWT 过期或格式错误 |
| 404 Not Found | `api/routes.rs` | 路由未注册或路径错误 |
| 500 Internal Error | 看日志中的 `AppError` | 数据库查询失败、空指针等 |
| JSON 解析失败 | `dto/request.rs` | 请求格式与 DTO 不匹配 |

### 6.3 调试技巧

**1. 查看日志**
```bash
RUST_LOG=debug cargo run  # 开启 debug 日志
```

**2. 添加临时日志**
```rust
tracing::debug!("变量值: {:?}", some_var);
```

**3. 检查 SQL**
```rust
// 在 repository 中打印 SQL
tracing::debug!("SQL: {}", query);
```

---

## 7. Rust 知识点索引

### 7.1 本项目用到的 Rust 特性

| 特性 | 位置 | 学习优先级 |
|------|------|-----------|
| `async/await` | 全项目 | ⭐⭐⭐⭐⭐ |
| `Result` 和 `?` 操作符 | 全项目 | ⭐⭐⭐⭐⭐ |
| `derive` 宏 | models/, dto/ | ⭐⭐⭐⭐⭐ |
| Trait 定义和实现 | repositories/traits/ | ⭐⭐⭐⭐ |
| 泛型 | 各处 | ⭐⭐⭐⭐ |
| `Arc` 智能指针 | main.rs, api/ | ⭐⭐⭐⭐ |
| 模式匹配 | error/, api/ | ⭐⭐⭐ |
| 生命周期 | 少量使用 | ⭐⭐⭐ |
| 过程宏 | derive | ⭐⭐ |

### 7.2 推荐学习路径

1. **先理解数据流**：从 `main.rs` 开始，跟踪一个请求的处理过程
2. **掌握错误处理**：理解 `Result`、`Option`、`?` 操作符
3. **理解 Trait**：看 `repositories/traits/` 中的接口定义
4. **学习异步**：理解 `async/await` 和 Tokio 的基本概念
5. **深入所有权**：当遇到生命周期问题时再深入

### 7.3 当遇到不懂的代码时

1. **先看函数签名**：输入什么、输出什么
2. **看 `use` 语句**：引入了哪些外部库
3. **查文档**：`cargo doc --open` 或 docs.rs
4. **问 AI**：把代码片段发给 Claude 解释

---

## 附录：API 路由表

```
公开接口：
POST   /api/v1/auth/register     注册
POST   /api/v1/auth/login        登录
POST   /api/v1/auth/refresh      刷新 Token
GET    /api/v1/categories        获取分类列表

需认证接口：
GET    /api/v1/users/me          获取当前用户
PUT    /api/v1/users/me          更新用户信息
PUT    /api/v1/users/me/password 修改密码

GET    /api/v1/ledgers           账本列表
POST   /api/v1/ledgers           创建账本
GET    /api/v1/ledgers/:id       账本详情
PUT    /api/v1/ledgers/:id       更新账本
DELETE /api/v1/ledgers/:id       删除账本

GET    /api/v1/bills             账单列表
POST   /api/v1/bills             创建账单
GET    /api/v1/bills/:id         账单详情
PUT    /api/v1/bills/:id         更新账单
DELETE /api/v1/bills/:id         删除账单
GET    /api/v1/bills/statistics  统计数据

GET    /api/v1/groups            群组列表
POST   /api/v1/groups            创建群组
POST   /api/v1/groups/join       加入群组
...
```

---

## 写在最后

在 AI 时代，你的目标很对：**理解系统架构，知道如何定位问题**。

当遇到问题时：
1. 先看错误信息，判断是哪一层的问题
2. 找到对应的模块代码
3. 如果看不懂，把代码和错误发给 AI 帮你解释

不需要记住每一行代码的细节，但要建立起「问题 → 模块」的映射关系。
