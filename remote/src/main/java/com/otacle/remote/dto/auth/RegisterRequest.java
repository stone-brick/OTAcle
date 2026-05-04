package com.otacle.remote.dto.auth;

import io.swagger.v3.oas.annotations.media.Schema;
import jakarta.validation.constraints.Email;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;
import lombok.Data;

@Data
@Schema(description = "用户注册请求")
public class RegisterRequest {
    
    @NotBlank(message = "用户名不能为空")
    @Size(min = 3, max = 50, message = "用户名长度必须在3-50之间")
    @Schema(description = "用户名", example = "user123", requiredMode = Schema.RequiredMode.REQUIRED)
    private String username;
    
    @NotBlank(message = "密码不能为空")
    @Size(min = 6, max = 100, message = "密码长度必须在6-100之间")
    @Schema(description = "密码", example = "password123", requiredMode = Schema.RequiredMode.REQUIRED)
    private String password;
    
    @Email(message = "邮箱格式不正确")
    @Schema(description = "邮箱地址", example = "user@example.com", requiredMode = Schema.RequiredMode.NOT_REQUIRED)
    private String email;
}
