package com.otacle.remote.config;

import com.aliyun.oss.OSS;
import com.aliyun.oss.OSSClientBuilder;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.beans.factory.annotation.Value;

@Configuration
public class OssConfig {

    @Value("${aliyun.oss.endpoint:}")
    private String endpoint;

    @Value("${aliyun.oss.access-key-id:}")
    private String accessKeyId;

    @Value("${aliyun.oss.access-key-secret:}")
    private String accessKeySecret;

    @Value("${aliyun.oss.bucket-name:}")
    private String bucketName;

    @Bean
    @ConditionalOnProperty(name = "storage.type", havingValue = "aliyun", matchIfMissing = true)
    public OSS ossClient() {
        if (accessKeyId == null || accessKeyId.isEmpty()
                || accessKeySecret == null || accessKeySecret.isEmpty()
                || bucketName == null || bucketName.isEmpty()) {
            throw new IllegalStateException(
                "使用阿里云 OSS 存储需要配置 aliyun.oss.access-key-id、aliyun.oss.access-key-secret、aliyun.oss.bucket-name");
        }
        return new OSSClientBuilder().build(endpoint, accessKeyId, accessKeySecret);
    }
}