#!/bin/bash
# OTAcle SpringBoot Backend - Quick Start Script for Linux/Mac

echo "========================================"
echo "OTAcle Backend Quick Start"
echo "========================================"
echo ""

# Check if Maven is installed
if ! command -v mvn &> /dev/null; then
    echo "[ERROR] Maven is not installed or not in PATH"
    echo "Please install Maven from https://maven.apache.org/download.cgi"
    exit 1
fi

# Check if Java is installed
if ! command -v java &> /dev/null; then
    echo "[ERROR] Java is not installed or not in PATH"
    echo "Please install JDK 21 or higher"
    exit 1
fi

echo "[INFO] Starting SpringBoot Backend..."
echo "[INFO] Server will be available at http://localhost:8080"
echo "[INFO] H2 Console: http://localhost:8080/h2-console"
echo ""

cd "$(dirname "$0")"
./mvnw spring-boot:run
