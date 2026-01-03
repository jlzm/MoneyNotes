# ================================
# 阶段1: 构建阶段
# ================================
FROM node:20-alpine as builder

# 安装 pnpm
RUN corepack enable && corepack prepare pnpm@latest --activate

WORKDIR /app

# 复制 pnpm 配置文件
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml* ./

# 复制所有包的 package.json
COPY packages/api/package.json ./packages/api/
COPY packages/shared/package.json ./packages/shared/
COPY packages/app/package.json ./packages/app/

# 安装依赖
RUN pnpm install --frozen-lockfile

# 复制源代码
COPY packages ./packages

# 构建 H5 版本
RUN pnpm --filter @money-notes/app build:h5

# ================================
# 阶段2: 运行阶段 (Nginx)
# ================================
FROM nginx:alpine

# 复制 Nginx 配置
COPY nginx.conf /etc/nginx/conf.d/default.conf

# 从构建阶段复制静态文件
COPY --from=builder /app/packages/app/dist/build/h5 /usr/share/nginx/html

# 暴露端口
EXPOSE 80

# 启动 Nginx
CMD ["nginx", "-g", "daemon off;"]
