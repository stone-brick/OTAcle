#!/bin/bash

echo "========================================"
echo "  OTAcle Remote - 导出 OpenAPI 文档"
echo "========================================"
echo ""

# 检查应用是否正在运行
echo "[1/3] 检查应用状态..."
if ! curl -s http://localhost:8080/v3/api-docs > /dev/null 2>&1; then
    echo "⚠️  应用未运行，正在启动..."
    echo ""
    ./mvnw spring-boot:run &
    APP_PID=$!
    echo "等待应用启动（约 30 秒）..."
    sleep 30
fi

echo "[2/3] 导出 OpenAPI JSON..."
if curl -s http://localhost:8080/v3/api-docs -o openapi.json; then
    echo "✅ OpenAPI JSON 已导出: openapi.json"
else
    echo "❌ 导出失败"
    exit 1
fi

echo ""
echo "[3/3] 导出 OpenAPI YAML..."
if curl -s http://localhost:8080/v3/api-docs.yaml -o openapi.yaml; then
    echo "✅ OpenAPI YAML 已导出: openapi.yaml"
else
    echo "⚠️  YAML 导出失败（可选）"
fi

echo ""
echo "========================================"
echo "  导出完成！"
echo "========================================"
echo ""
echo "📄 文件位置:"
echo "   - openapi.json"
echo "   - openapi.yaml"
echo ""
echo "🦊 导入 Apifox:"
echo "   1. 打开 Apifox"
echo "   2. 点击'导入数据'"
echo "   3. 选择'文件导入'"
echo "   4. 选择 openapi.json"
echo ""
echo "🌐 Swagger UI:"
echo "   http://localhost:8080/swagger-ui.html"
echo ""
