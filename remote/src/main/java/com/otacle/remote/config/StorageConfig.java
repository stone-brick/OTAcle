package com.otacle.remote.config;

import com.otacle.remote.service.OssService;
import com.otacle.remote.service.storage.StorageService;
import org.springframework.boot.autoconfigure.condition.ConditionalOnProperty;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

/**
 * 存储服务配置
 * 根据 storage.type 配置创建对应的存储服务 Bean
 */
@Configuration
public class StorageConfig {

    @Bean
    @ConditionalOnProperty(name = "storage.type", havingValue = "aliyun", matchIfMissing = true)
    public StorageService aliyunStorageService(OssService ossService) {
        return ossService;
    }
}