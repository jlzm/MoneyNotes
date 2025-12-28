# 系统架构设计

## 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                        客户端层                              │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐        │
│  │   Web   │  │   App   │  │ 小程序   │  │   H5    │        │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘        │
│       └────────────┴────────────┴────────────┘              │
│                         ▼                                    │
│              ┌─────────────────────┐                        │
│              │   UniApp X 前端     │                        │
│              │   (pnpm monorepo)   │                        │
│              └──────────┬──────────┘                        │
└─────────────────────────┼───────────────────────────────────┘
                          │ HTTP/HTTPS
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                        服务端层                              │
│              ┌─────────────────────┐                        │
│              │      API 网关       │                        │
│              │   (Axum Router)     │                        │
│              └──────────┬──────────┘                        │
│                         │                                    │
│    ┌────────────────────┼────────────────────┐              │
│    ▼                    ▼                    ▼              │
│ ┌──────────┐     ┌──────────┐     ┌──────────┐             │
│ │ 用户服务  │     │ 账单服务  │     │ 群组服务  │             │
│ └────┬─────┘     └────┬─────┘     └────┬─────┘             │
│      └────────────────┼────────────────┘                    │
│                       ▼                                      │
│           ┌───────────────────────┐                         │
│           │    数据访问层 (DAL)    │                         │
│           │  Repository Pattern   │                         │
│           └───────────┬───────────┘                         │
└───────────────────────┼─────────────────────────────────────┘
                        │
         ┌──────────────┴──────────────┐
         ▼                             ▼
┌─────────────────┐          ┌─────────────────┐
│     MySQL       │          │    MongoDB      │
│  (关系型存储)    │          │  (文档型存储)    │
└─────────────────┘          └─────────────────┘
```

## 后端模块划分

```
server/
├── src/
│   ├── main.rs              # 应用入口
│   ├── lib.rs               # 库导出
│   │
│   ├── config/              # 配置管理
│   │   ├── mod.rs
│   │   └── settings.rs      # 配置结构
│   │
│   ├── api/                 # API 层 (Controllers)
│   │   ├── mod.rs
│   │   ├── routes.rs        # 路由注册
│   │   ├── auth.rs          # 认证相关 API
│   │   ├── user.rs          # 用户 API
│   │   ├── bill.rs          # 账单 API
│   │   ├── ledger.rs        # 账本 API
│   │   ├── group.rs         # 群组 API
│   │   └── category.rs      # 分类 API
│   │
│   ├── services/            # 业务逻辑层
│   │   ├── mod.rs
│   │   ├── auth_service.rs
│   │   ├── user_service.rs
│   │   ├── bill_service.rs
│   │   ├── ledger_service.rs
│   │   ├── group_service.rs
│   │   └── category_service.rs
│   │
│   ├── repositories/        # 数据访问层
│   │   ├── mod.rs
│   │   ├── traits.rs        # Repository trait 定义
│   │   ├── mysql/           # MySQL 实现
│   │   │   ├── mod.rs
│   │   │   ├── user_repo.rs
│   │   │   └── ...
│   │   └── mongodb/         # MongoDB 实现
│   │       ├── mod.rs
│   │       ├── user_repo.rs
│   │       └── ...
│   │
│   ├── models/              # 数据模型
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── bill.rs
│   │   ├── ledger.rs
│   │   ├── group.rs
│   │   └── category.rs
│   │
│   ├── middleware/          # 中间件
│   │   ├── mod.rs
│   │   ├── auth.rs          # JWT 验证
│   │   ├── cors.rs          # 跨域
│   │   └── logger.rs        # 日志
│   │
│   ├── dto/                 # 数据传输对象
│   │   ├── mod.rs
│   │   ├── request.rs       # 请求 DTO
│   │   └── response.rs      # 响应 DTO
│   │
│   ├── error/               # 错误处理
│   │   ├── mod.rs
│   │   └── app_error.rs
│   │
│   └── utils/               # 工具函数
│       ├── mod.rs
│       ├── jwt.rs
│       ├── hash.rs
│       └── validator.rs
│
├── migrations/              # 数据库迁移 (MySQL)
│   └── ...
│
├── Cargo.toml
└── config/                  # 配置文件
    ├── default.toml
    ├── development.toml
    └── production.toml
```

## 前端模块划分

```
client/
├── package.json
├── pnpm-workspace.yaml
│
├── packages/
│   ├── app/                 # 主应用
│   │   ├── src/
│   │   │   ├── pages/       # 页面
│   │   │   │   ├── index/   # 首页
│   │   │   │   ├── login/   # 登录
│   │   │   │   ├── bill/    # 记账
│   │   │   │   ├── stats/   # 统计
│   │   │   │   ├── group/   # 群组
│   │   │   │   └── mine/    # 我的
│   │   │   ├── components/  # 页面级组件
│   │   │   ├── store/       # Pinia 状态
│   │   │   └── App.vue
│   │   ├── pages.json
│   │   └── manifest.json
│   │
│   ├── shared/              # 共享模块
│   │   ├── components/      # 公共组件
│   │   ├── composables/     # 组合式函数
│   │   ├── utils/           # 工具函数
│   │   └── styles/          # 公共样式
│   │
│   └── api/                 # API 封装
│       ├── src/
│       │   ├── index.ts
│       │   ├── request.ts   # 请求封装
│       │   ├── auth.ts
│       │   ├── bill.ts
│       │   ├── ledger.ts
│       │   └── group.ts
│       └── package.json
│
└── scripts/                 # 构建脚本
```

## 数据库切换设计

使用 **Repository Pattern** + **Trait** 实现数据库抽象：

```rust
// traits.rs - 定义接口
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: CreateUser) -> Result<User>;
    async fn find_by_id(&self, id: &str) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    // ...
}

// mysql/user_repo.rs - MySQL 实现
pub struct MySqlUserRepository { pool: MySqlPool }

#[async_trait]
impl UserRepository for MySqlUserRepository {
    async fn create(&self, user: CreateUser) -> Result<User> {
        // MySQL 实现
    }
}

// mongodb/user_repo.rs - MongoDB 实现
pub struct MongoUserRepository { db: Database }

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn create(&self, user: CreateUser) -> Result<User> {
        // MongoDB 实现
    }
}
```

通过配置文件选择数据库：

```toml
# config/development.toml
[database]
driver = "mysql"  # 或 "mongodb"
url = "mysql://user:pass@localhost/money_notes"
```

## 认证流程

```
┌────────┐                              ┌────────┐
│ Client │                              │ Server │
└───┬────┘                              └───┬────┘
    │                                       │
    │  1. POST /api/auth/login              │
    │  { email, password }                  │
    │──────────────────────────────────────>│
    │                                       │
    │  2. 验证成功，返回 Token              │
    │  { access_token, refresh_token }      │
    │<──────────────────────────────────────│
    │                                       │
    │  3. 请求带上 Authorization Header     │
    │  Authorization: Bearer <token>        │
    │──────────────────────────────────────>│
    │                                       │
    │  4. JWT 中间件验证                    │
    │                                       │
    │  5. 返回数据                          │
    │<──────────────────────────────────────│
```

## 群组权限模型

```
角色层级:
  OWNER (群主)
    ├── 解散群组
    ├── 转让群组
    ├── 管理所有成员
    └── 所有 ADMIN 权限
        │
  ADMIN (管理员)
    ├── 邀请/移除成员
    ├── 编辑群组信息
    └── 所有 MEMBER 权限
        │
  MEMBER (成员)
    ├── 创建账单
    ├── 编辑自己的账单
    ├── 查看群组账单
    └── 退出群组
```
