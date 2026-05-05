package com.otacle.remote.service.storage;

import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.core.io.Resource;
import org.springframework.core.io.UrlResource;
import org.springframework.stereotype.Service;
import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;
import java.net.MalformedURLException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;
import java.util.UUID;

/**
 * 本地文件系统存储实现
 */
@Slf4j
@Service
@ConditionalOnProperty(name = "storage.type", havingValue = "local")
public class LocalStorageService implements StorageService {

    @Value("${storage.local.base-path:./local-storage}")
    private String basePath;

    private Path rootLocation;

    private Path getRootLocation() {
        if (rootLocation == null) {
            rootLocation = Paths.get(basePath).toAbsolutePath().normalize();
            try {
                Files.createDirectories(rootLocation);
                log.info("本地存储目录初始化: {}", rootLocation);
            } catch (IOException e) {
                throw new RuntimeException("无法创建本地存储目录: " + rootLocation, e);
            }
        }
        return rootLocation;
    }

    @Override
    public String uploadFile(MultipartFile file, String objectKey) throws IOException {
        Path destinationDir = getRootLocation().resolve(objectKey).getParent();
        Files.createDirectories(destinationDir);

        Path destinationFile = getRootLocation().resolve(objectKey).normalize();
        Files.copy(file.getInputStream(), destinationFile, StandardCopyOption.REPLACE_EXISTING);

        log.info("文件上传到本地: {}", destinationFile);
        return objectKey;
    }

    @Override
    public String generatePresignedUrl(String objectKey) {
        // 本地存储返回内置访问端点，由 StorageController 提供文件访问
        // 使用 query parameter 传递路径，避免路径参数解析问题
        return "/api/storage/files?path=" + objectKey;
    }

    @Override
    public String generateObjectKey(Long groupId, String characterId, Integer version) {
        String uuid = UUID.randomUUID().toString().substring(0, 8);
        return String.format("otacle-config/%s/%s/%s_v%s_%s.zip",
                groupId, characterId, groupId, version, uuid);
    }

    /**
     * 根据 objectKey 获取本地文件 Resource
     * 供 StorageController 调用
     */
    public Resource loadFileAsResource(String objectKey) throws MalformedURLException {
        Path file = getRootLocation().resolve(objectKey);
        Resource resource = new UrlResource(file.toUri());
        if (resource.exists() && resource.isReadable()) {
            return resource;
        } else {
            throw new RuntimeException("文件不存在或不可读: " + objectKey);
        }
    }
}