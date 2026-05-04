@echo off
chcp 65001 >nul
echo ========================================
echo   OTAcle Remote - 导出 OpenAPI 文档
echo ========================================
echo.

REM 检查应用是否正在运行
echo [1/3] 检查应用状态...
curl -s http://localhost:8080/v3/api-docs >nul 2>&1
if %errorlevel% neq 0 (
    echo ⚠️  应用未运行，正在启动...
    echo.
    start "OTAcle Remote" mvnw.cmd spring-boot:run
    echo 等待应用启动（约 30 秒）...
    timeout /t 30 /nobreak >nul
)

echo [2/3] 导出 OpenAPI JSON...
curl -s http://localhost:8080/v3/api-docs -o openapi.json
if %errorlevel% equ 0 (
    echo ✅ OpenAPI JSON 已导出: openapi.json
) else (
    echo ❌ 导出失败
    exit /b 1
)

echo.
echo [3/3] 导出 OpenAPI YAML...
curl -s http://localhost:8080/v3/api-docs.yaml -o openapi.yaml
if %errorlevel% equ 0 (
    echo ✅ OpenAPI YAML 已导出: openapi.yaml
) else (
    echo ⚠️  YAML 导出失败（可选）
)

echo.
echo ========================================
echo   导出完成！
echo ========================================
echo.
echo 📄 文件位置:
echo    - openapi.json
echo    - openapi.yaml
echo.
echo 🦊 导入 Apifox:
echo    1. 打开 Apifox
echo    2. 点击"导入数据"
echo    3. 选择"文件导入"
echo    4. 选择 openapi.json
echo.
echo 🌐 Swagger UI:
echo    http://localhost:8080/swagger-ui.html
echo.
pause
