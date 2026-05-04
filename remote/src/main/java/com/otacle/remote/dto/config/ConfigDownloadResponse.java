package com.otacle.remote.dto.config;

import io.swagger.v3.oas.annotations.media.Schema;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@NoArgsConstructor
@AllArgsConstructor
@Schema(description = "配置下载响应")
public class ConfigDownloadResponse {
    
    @Schema(description = "配置 ID", example = "1")
    private Long configId;
    
    @Schema(description = "角色 ID", example = "char001")
    private String characterId;
    
    @Schema(description = "版本号", example = "3")
    private Integer version;
    
    @Schema(description = "文件名", example = "config.zip")
    private String fileName;
    
    @Schema(description = "文件大小（字节）", example = "102400")
    private Long fileSize;
    
    @Schema(description = "SHA-256 校验和", example = "a1b2c3d4e5f6...")
    private String checksum;
    
    @Schema(description = "OSS 签名下载 URL（有效期 1 小时）", 
            example = "https://bucket.oss.cn-hangzhou.aliyuncs.com/otacle-config/1/char001/1_v3_abc12345.zip?signature=xxx&expires=xxx")
    private String downloadUrl;
    
    @Schema(description = "创建时间", example = "2024-01-03T12:00:00")
    private String createdAt;
}
