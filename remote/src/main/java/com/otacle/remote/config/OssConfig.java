package com.otacle.remote.config;

import com.aliyun.oss.OSS;
import com.aliyun.oss.OSSClientBuilder;
import jakarta.annotation.PostConstruct;
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

    @Value("${aliyun.oss.enabled:false}")
    private boolean enabled;

    @PostConstruct
    public void validate() {
        if (!enabled) {
            return;
        }
        if (accessKeyId == null || accessKeyId.isEmpty()
                || accessKeySecret == null || accessKeySecret.isEmpty()
                || bucketName == null || bucketName.isEmpty()) {
            throw new IllegalStateException(
                "OSS 配置不完整，请设置 OSS_ACCESS_KEY_ID、OSS_ACCESS_KEY_SECRET、OSS_BUCKET_NAME 环境变量，或设置 aliyun.oss.enabled=false 禁用 OSS");
        }
    }

    @Bean
    @ConditionalOnProperty(name = "aliyun.oss.enabled", havingValue = "true", matchIfMissing = false)
    public OSS ossClient() {
        return new OSSClientBuilder().build(endpoint, accessKeyId, accessKeySecret);
    }
}