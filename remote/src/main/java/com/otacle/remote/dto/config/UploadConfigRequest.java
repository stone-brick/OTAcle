package com.otacle.remote.dto.config;

import jakarta.validation.constraints.NotBlank;
import lombok.Data;
import org.springframework.web.multipart.MultipartFile;

@Data
public class UploadConfigRequest {
    @NotBlank(message = "分组ID不能为空")
    private Long groupId;
    
    @NotBlank(message = "角色ID不能为空")
    private String characterId;
    
    private MultipartFile file; // zip 文件
}
