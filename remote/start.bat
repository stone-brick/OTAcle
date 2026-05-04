@echo off
echo Starting OTAcle Remote Server...
cd /d %~dp0
call mvnw.cmd spring-boot:run
pause

REM OTAcle SpringBoot Backend - Quick Start Script for Windows

echo ========================================
echo OTAcle Backend Quick Start
echo ========================================
echo.

REM Check if Maven is installed
where mvn >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Maven is not installed or not in PATH
    echo Please install Maven from https://maven.apache.org/download.cgi
    pause
    exit /b 1
)

REM Check if Java is installed
where java >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Java is not installed or not in PATH
    echo Please install JDK 21 or higher
    pause
    exit /b 1
)

echo [INFO] Starting SpringBoot Backend...
echo [INFO] Server will be available at http://localhost:8080
echo [INFO] H2 Console: http://localhost:8080/h2-console
echo.

cd /d "%~dp0"
mvn spring-boot:run

pause
