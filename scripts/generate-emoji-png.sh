#!/bin/bash
# 生成水獭 emoji PNG 图片
# 使用 macOS 自带的工具

OUTPUT_DIR="../client/packages/app/src/static/logo"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

# 创建输出目录
mkdir -p "$OUTPUT_DIR"

# 创建一个临时 HTML 文件
cat > /tmp/otter-emoji.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
<style>
body {
  margin: 0;
  padding: 0;
  width: 1024px;
  height: 1024px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: white;
}
.emoji {
  font-size: 800px;
  line-height: 1;
}
</style>
</head>
<body>
<span class="emoji">🦦</span>
</body>
</html>
EOF

echo "请手动完成以下步骤："
echo ""
echo "1. 在浏览器中打开: file:///tmp/otter-emoji.html"
echo "2. 使用截图工具截取 emoji 部分"
echo "3. 保存到: $SCRIPT_DIR/$OUTPUT_DIR/"
echo ""
echo "或者使用在线工具: https://emoji.aranja.com/"
echo "搜索 'otter' 下载水獭 emoji PNG"
