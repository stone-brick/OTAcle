package com.otacle.remote.config;

import com.aliyun.oss.OSS;
import com.aliyun.oss.OSSClientBuilder;
import jakarta.annotation.PostConstruct;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class OssConfig {

    @Value("${aliyun.oss.endpoint}")
    private String endpoint;

    @Value("${aliyun.oss.access-key-id:}")
    private String accessKeyId;

    @Value("${aliyun.oss.access-key-secret:}")
    private String accessKeySecret;

    @Value("${aliyun.oss.bucket-name:}")
    private String bucketName;

    @PostConstruct
    public void validate() {
        if (accessKeyId == null || accessKeyId.isEmpty()
                || accessKeySecret == null || accessKeySecret.isEmpty()
                || bucketName == null || bucketName.isEmpty()) {
            throw new IllegalStateException(
                "OSS 配置不完整，请设置 OSS_ACCESS_KEY_ID、OSS_ACCESS_KEY_SECRET、OSS_BUCKET_NAME 环境变量");
        }
    }

    @Bean
    public OSS ossClient() {
        return new OSSClientBuilder().build(endpoint, accessKeyId, accessKeySecret);
    }
}
