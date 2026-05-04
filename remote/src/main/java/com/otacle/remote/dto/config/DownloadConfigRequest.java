package com.otacle.remote.dto.config;

import jakarta.validation.constraints.NotBlank;
import lombok.Data;

@Data
public class DownloadConfigRequest {
    @NotBlank(message = "分组ID不能为空")
    private Long groupId;
    
    @NotBlank(message = "角色ID不能为空")
    private String characterId;
    
    private Integer version; // 可选，不传则获取最新版本
}
