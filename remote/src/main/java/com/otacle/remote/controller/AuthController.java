package com.otacle.remote.controller;

import com.otacle.remote.dto.ApiResponse;
import com.otacle.remote.dto.auth.AuthResponse;
import com.otacle.remote.dto.auth.LoginRequest;
import com.otacle.remote.dto.auth.RegisterRequest;
import com.otacle.remote.model.Account;
import com.otacle.remote.service.AuthService;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.validation.Valid;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.*;

@Slf4j
@RestController
@RequestMapping("/api/auth")
@CrossOrigin(origins = "*")
@Tag(name = "认证管理", description = "用户注册、登录和身份验证接口")
public class AuthController {

    private final AuthService authService;

    public AuthController(AuthService authService) {
        this.authService = authService;
    }

    /**
     * 用户注册
     */
    @PostMapping("/register")
    @Operation(
        summary = "用户注册",
        description = "创建新用户账号，成功后自动登录并返回 JWT Token"
    )
    public ApiResponse<AuthResponse> register(@Valid @RequestBody RegisterRequest request) {
        try {
            AuthResponse response = authService.register(request);
            return ApiResponse.success(response);
        } catch (IllegalArgumentException e) {
            return ApiResponse.error(400, e.getMessage());
        } catch (Exception e) {
            log.error("注册失败", e);
            return ApiResponse.error(500, "注册失败，请稍后重试");
        }
    }

    /**
     * 用户登录
     */
    @PostMapping("/login")
    @Operation(
        summary = "用户登录",
        description = "使用用户名和密码登录，返回 JWT Token 用于后续请求认证"
    )
    public ApiResponse<AuthResponse> login(@Valid @RequestBody LoginRequest request) {
        try {
            AuthResponse response = authService.login(request);
            return ApiResponse.success(response);
        } catch (IllegalArgumentException e) {
            return ApiResponse.error(401, e.getMessage());
        } catch (Exception e) {
            log.error("登录失败", e);
            return ApiResponse.error(500, "登录失败，请稍后重试");
        }
    }

    /**
     * 获取当前用户信息
     */
    @GetMapping("/me")
    @Operation(
        summary = "获取当前用户信息",
        description = "根据 JWT Token 获取当前登录用户的详细信息"
    )
    public ApiResponse<Account> getCurrentUser(@Parameter(hidden = true) @RequestAttribute("userId") Long userId) {
        try {
            Account account = authService.getCurrentUser(userId);
            return ApiResponse.success(account);
        } catch (Exception e) {
            log.error("获取用户信息失败", e);
            return ApiResponse.error(500, "获取用户信息失败");
        }
    }
}
