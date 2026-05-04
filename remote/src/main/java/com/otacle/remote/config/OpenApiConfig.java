package com.otacle.remote.config;

import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.info.Contact;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.info.License;
import io.swagger.v3.oas.models.servers.Server;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import java.util.List;

/**
 * OpenAPI (Swagger) 配置
 */
@Configuration
public class OpenApiConfig {

    @Bean
    public OpenAPI customOpenAPI() {
        return new OpenAPI()
                .info(new Info()
                        .title("OTAcle Remote API")
                        .description("OTAcle 远程配置同步模块 API 文档\n\n" +
                                "提供用户认证、分组管理、配置上传下载和版本控制功能。\n\n" +
                                "**核心功能：**\n" +
                                "- 用户注册和登录（JWT 认证）\n" +
                                "- 分组创建和管理\n" +
                                "- 配置文件上传到阿里云 OSS\n" +
                                "- 配置版本管理和回滚\n" +
                                "- 分组内配置共享")
                        .version("1.0.0")
                        .contact(new Contact()
                                .name("OTAcle Team")
                                .email("support@otacle.com"))
                        .license(new License()
                                .name("MIT License")
                                .url("https://opensource.org/licenses/MIT")))
                .servers(List.of(
                        new Server()
                                .url("http://localhost:8080")
                                .description("本地开发环境"),
                        new Server()
                                .url("https://api.otacle.com")
                                .description("生产环境")
                ));
    }
}
