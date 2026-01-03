#!/bin/bash
# 部署前端到服务器

SERVER="root@120.76.238.8"
IMAGE_FILE="/tmp/money-notes-web.tar.gz"

echo "=== 上传镜像到服务器 ==="
scp $IMAGE_FILE $SERVER:/root/

echo "=== 在服务器上加载镜像并重启 ==="
ssh $SERVER << 'EOF'
cd /root/money-notes
docker load < /root/money-notes-web.tar.gz
docker compose up -d client
docker ps
echo "=== 部署完成 ==="
EOF
