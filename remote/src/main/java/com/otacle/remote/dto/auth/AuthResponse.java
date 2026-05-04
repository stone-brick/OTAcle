package com.otacle.remote.dto.auth;

import io.swagger.v3.oas.annotations.media.Schema;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@NoArgsConstructor
@AllArgsConstructor
@Schema(description = "认证响应")
public class AuthResponse {
    
    @Schema(description = "JWT Token", example = "eyJhbGciOiJIUzI1NiJ9...")
    private String token;
    
    @Schema(description = "用户 ID", example = "1")
    private Long userId;
    
    @Schema(description = "用户名", example = "user123")
    private String username;
}
