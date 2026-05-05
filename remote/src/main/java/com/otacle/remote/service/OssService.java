package com.otacle.remote.service;

import com.aliyun.oss.OSS;
import com.aliyun.oss.model.GeneratePresignedUrlRequest;
import com.otacle.remote.service.storage.StorageService;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.stereotype.Service;
import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;
import java.net.URL;
import java.util.Date;
import java.util.UUID;

@Slf4j
@Service
@ConditionalOnProperty(name = "storage.type", havingValue = "aliyun", matchIfMissing = true)
public class OssService implements StorageService {

    @Autowired
    private OSS ossClient;

    @Value("${aliyun.oss.bucket-name:}")
    private String bucketName;

    @Value("${aliyun.oss.endpoint:}")
    private String endpoint;

    @Override
    public String uploadFile(MultipartFile file, String objectKey) throws IOException {
        try {
            ossClient.putObject(bucketName, objectKey, file.getInputStream());
            log.info("文件上传成功: {}", objectKey);
            return objectKey;
        } catch (Exception e) {
            log.error("文件上传失败: {}", objectKey, e);
            throw new IOException("文件上传失败: " + e.getMessage(), e);
        }
    }

    @Override
    public String generatePresignedUrl(String objectKey) {
        Date expiration = new Date(System.currentTimeMillis() + 3600 * 1000); // 1小时
        GeneratePresignedUrlRequest request = new GeneratePresignedUrlRequest(bucketName, objectKey);
        request.setExpiration(expiration);

        URL url = ossClient.generatePresignedUrl(request);
        return url.toString();
    }

    @Override
    public String generateObjectKey(Long groupId, String characterId, Integer version) {
        String uuid = UUID.randomUUID().toString().substring(0, 8);
        return String.format("otacle-config/%s/%s/%s_v%s_%s.zip",
                groupId, characterId, groupId, version, uuid);
    }
}