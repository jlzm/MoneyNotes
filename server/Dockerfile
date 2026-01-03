# ================================
# 阶段1: 构建阶段
# ================================
FROM rust:1.85-slim AS builder

# 使用国内镜像源
RUN sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list.d/debian.sources

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 配置 Rust 国内镜像源 (字节跳动)
RUN mkdir -p /usr/local/cargo && \
    echo '[source.crates-io]' > /usr/local/cargo/config.toml && \
    echo 'replace-with = "rsproxy"' >> /usr/local/cargo/config.toml && \
    echo '[source.rsproxy]' >> /usr/local/cargo/config.toml && \
    echo 'registry = "sparse+https://rsproxy.cn/index/"' >> /usr/local/cargo/config.toml && \
    echo '[registries.rsproxy]' >> /usr/local/cargo/config.toml && \
    echo 'index = "sparse+https://rsproxy.cn/index/"' >> /usr/local/cargo/config.toml && \
    echo '[net]' >> /usr/local/cargo/config.toml && \
    echo 'git-fetch-with-cli = true' >> /usr/local/cargo/config.toml

WORKDIR /app

# 复制 Cargo 文件，利用 Docker 缓存
COPY Cargo.toml Cargo.lock ./

# 创建虚拟 src 以缓存依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# 复制实际源码
COPY src ./src
COPY config ./config

# 重新编译（只编译变更的代码）
RUN touch src/main.rs && cargo build --release

# ================================
# 阶段2: 运行阶段
# ================================
FROM debian:bookworm-slim

# 使用国内镜像源
RUN sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list.d/debian.sources

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/money-notes-server /app/server
COPY --from=builder /app/config /app/config

# 创建非 root 用户
RUN useradd -r -s /bin/false appuser && chown -R appuser:appuser /app
USER appuser

# 暴露端口
EXPOSE 3000

# 环境变量
ENV RUST_LOG=info
ENV RUN_MODE=production

# 启动命令
CMD ["./server"]
