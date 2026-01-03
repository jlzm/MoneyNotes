# Money Notes 部署手册

## 目录

- [快速开始](#快速开始)
- [环境要求](#环境要求)
- [部署方式](#部署方式)
  - [Docker Compose 部署](#docker-compose-部署)
  - [手动部署](#手动部署)
  - [云平台部署](#云平台部署)
- [配置说明](#配置说明)
- [运维操作](#运维操作)
- [故障排除](#故障排除)

---

## 快速开始

```bash
# 1. 克隆代码
git clone https://github.com/jlzm/MoneyNotes.git
cd MoneyNotes

# 2. 配置环境变量
cp .env.example .env
# 编辑 .env 文件，修改必要的配置（尤其是 JWT_SECRET）

# 3. 启动服务
docker compose up -d

# 4. 查看状态
docker compose ps

# 5. 访问应用
# 前端: http://localhost
# API:  http://localhost:3000/api/v1/
```

---

## 环境要求

### 服务器配置（最低要求）

| 配置项 | 最低配置 | 推荐配置 |
|--------|---------|---------|
| CPU    | 1 核    | 2 核    |
| 内存   | 2 GB    | 4 GB    |
| 硬盘   | 20 GB   | 50 GB   |
| 系统   | Linux   | Ubuntu 22.04 / Debian 12 |

### 软件依赖

- Docker 24.0+
- Docker Compose 2.20+

安装 Docker（Ubuntu/Debian）:

```bash
# 安装 Docker
curl -fsSL https://get.docker.com | sh

# 添加当前用户到 docker 组
sudo usermod -aG docker $USER

# 重新登录后验证
docker --version
docker compose version
```

---

## 部署方式

### Docker Compose 部署

#### 1. 开发环境

```bash
# 启动所有服务
docker compose up -d

# 查看日志
docker compose logs -f

# 停止服务
docker compose down
```

#### 2. 生产环境

```bash
# 使用生产配置启动
docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d

# 构建并推送镜像到私有仓库
docker compose build
docker compose push
```

#### 3. 仅启动特定服务

```bash
# 仅启动后端和数据库
docker compose up -d mysql server

# 启动带 Redis 的配置
docker compose --profile with-redis up -d
```

---

### 手动部署

#### 后端部署

```bash
# 1. 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. 编译
cd server
cargo build --release

# 3. 配置
cp config/default.toml config/production.toml
# 编辑 production.toml

# 4. 运行
./target/release/test_server
```

#### 前端部署

```bash
# 1. 安装 Node.js 和 pnpm
curl -fsSL https://get.pnpm.io/install.sh | sh

# 2. 安装依赖
cd client
pnpm install

# 3. 构建 H5 版本
pnpm --filter @money-notes/app build:h5

# 4. 部署静态文件
# 将 packages/app/dist/build/h5 目录部署到 Nginx
```

#### Nginx 配置示例

```nginx
server {
    listen 80;
    server_name your-domain.com;
    root /var/www/money-notes;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    location /api/ {
        proxy_pass http://127.0.0.1:3000/api/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

---

### 云平台部署

#### 阿里云 ECS

```bash
# 1. 安装 Docker
yum install -y docker
systemctl start docker
systemctl enable docker

# 2. 安装 Docker Compose
curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose

# 3. 部署应用
cd /opt
git clone https://github.com/jlzm/MoneyNotes.git
cd MoneyNotes
cp .env.example .env
vim .env  # 修改配置
docker compose up -d
```

#### 使用 Systemd 管理

创建 `/etc/systemd/system/money-notes.service`:

```ini
[Unit]
Description=Money Notes Application
Requires=docker.service
After=docker.service

[Service]
Type=oneshot
RemainAfterExit=yes
WorkingDirectory=/opt/MoneyNotes
ExecStart=/usr/local/bin/docker-compose up -d
ExecStop=/usr/local/bin/docker-compose down

[Install]
WantedBy=multi-user.target
```

```bash
systemctl daemon-reload
systemctl enable money-notes
systemctl start money-notes
```

---

## 配置说明

### 环境变量

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| `MYSQL_ROOT_PASSWORD` | MySQL root 密码 | root123456 |
| `MYSQL_DATABASE` | 数据库名 | money_notes |
| `MYSQL_USER` | 数据库用户 | money_notes |
| `MYSQL_PASSWORD` | 数据库密码 | money_notes123 |
| `JWT_SECRET` | JWT 签名密钥（**必须修改**） | - |
| `JWT_EXPIRES_IN` | Access Token 有效期（秒） | 3600 |
| `JWT_REFRESH_EXPIRES_IN` | Refresh Token 有效期（秒） | 604800 |
| `RUST_LOG` | 日志级别 | info |
| `SERVER_PORT` | 后端端口 | 3000 |
| `CLIENT_PORT` | 前端端口 | 80 |

### 生产环境安全配置

```bash
# 生成强随机 JWT 密钥
openssl rand -base64 32

# .env 配置示例
JWT_SECRET=aB3xK9mN2pQ5rT8wZ1vY4uC7iO0eS6dF
MYSQL_ROOT_PASSWORD=StrongP@ssw0rd!2024
MYSQL_PASSWORD=AnotherStr0ng!Pass
```

---

## 运维操作

### 日志查看

```bash
# 查看所有服务日志
docker compose logs -f

# 查看特定服务日志
docker compose logs -f server
docker compose logs -f mysql

# 查看最近 100 行
docker compose logs --tail=100 server
```

### 数据备份

```bash
# 备份 MySQL 数据
docker compose exec mysql mysqldump -u root -p money_notes > backup_$(date +%Y%m%d).sql

# 恢复数据
docker compose exec -T mysql mysql -u root -p money_notes < backup.sql

# 备份数据卷
docker run --rm -v money-notes_mysql_data:/data -v $(pwd):/backup alpine tar czf /backup/mysql_data.tar.gz /data
```

### 更新部署

```bash
# 拉取最新代码
git pull origin main

# 重新构建并更新
docker compose build
docker compose up -d

# 或者一键更新
docker compose up -d --build
```

### 扩容

```bash
# 扩展后端服务到 3 个实例
docker compose up -d --scale server=3
```

---

## 故障排除

### 常见问题

#### 1. 数据库连接失败

```bash
# 检查 MySQL 是否正常运行
docker compose ps mysql
docker compose logs mysql

# 测试连接
docker compose exec mysql mysql -u money_notes -p -e "SELECT 1"
```

#### 2. 端口被占用

```bash
# 查看端口占用
lsof -i :3000
lsof -i :80

# 修改 .env 中的端口配置
SERVER_PORT=3001
CLIENT_PORT=8080
```

#### 3. 容器启动失败

```bash
# 查看详细日志
docker compose logs --tail=50 server

# 进入容器调试
docker compose exec server sh

# 重新构建镜像
docker compose build --no-cache server
```

#### 4. 内存不足

```bash
# 查看容器资源使用
docker stats

# 清理无用镜像和容器
docker system prune -a
```

### 健康检查

```bash
# 检查后端 API
curl http://localhost:3000/api/v1/categories

# 检查前端
curl http://localhost/health

# 检查数据库
docker compose exec mysql mysqladmin ping -h localhost
```

---

## 架构图

```
                    ┌─────────────────┐
                    │     Nginx       │
                    │   (Port 80)     │
                    └────────┬────────┘
                             │
              ┌──────────────┴──────────────┐
              │                             │
              ▼                             ▼
    ┌─────────────────┐           ┌─────────────────┐
    │  Static Files   │           │   API Proxy     │
    │   (H5 App)      │           │   /api/*        │
    └─────────────────┘           └────────┬────────┘
                                           │
                                           ▼
                                  ┌─────────────────┐
                                  │  Rust Server    │
                                  │  (Port 3000)    │
                                  └────────┬────────┘
                                           │
                                           ▼
                                  ┌─────────────────┐
                                  │     MySQL       │
                                  │  (Port 3306)    │
                                  └─────────────────┘
```

---

## 联系支持

- GitHub Issues: https://github.com/jlzm/MoneyNotes/issues
- 开发文档: 查看项目 `DEVELOPMENT.md`
