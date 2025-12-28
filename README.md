# Money Notes - 记账应用

一个支持个人和群组记账的全栈应用。

## 项目特点

- **双模式使用**：支持访客模式（本地存储）和账号登录（云端同步）
- **群组记账**：支持家庭/团队共享账本，权限分级管理
- **多端支持**：Web、App、小程序一套代码
- **多数据库**：支持 MySQL 和 MongoDB

## 技术栈

### 后端 (server/)
- **语言**: Rust
- **框架**: Axum
- **数据库**: MySQL / MongoDB (可切换)
- **认证**: JWT

### 前端 (client/)
- **框架**: UniApp X
- **包管理**: pnpm workspace (monorepo)
- **状态管理**: Pinia

## 项目结构

```
money-notes/
├── README.md                 # 项目总览
├── DEVELOPMENT.md            # 开发进度追踪
├── docs/                     # 设计文档
│   ├── api.md               # API 设计
│   ├── database.md          # 数据库设计
│   └── architecture.md      # 架构设计
│
├── server/                   # Rust 后端
│   ├── Cargo.toml
│   └── src/
│
└── client/                   # UniApp X 前端
    ├── package.json
    └── packages/
```

## 快速开始

### 后端

```bash
cd server
cargo run
```

### 前端

```bash
cd client
pnpm install
pnpm dev
```

## 开发进度

详见 [DEVELOPMENT.md](./DEVELOPMENT.md)

## License

MIT
