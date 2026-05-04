package com.otacle.remote.controller;

import com.otacle.remote.dto.ApiResponse;
import com.otacle.remote.dto.config.ConfigDownloadResponse;
import com.otacle.remote.model.ConfigVersion;
import com.otacle.remote.service.ConfigService;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.tags.Tag;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

import java.util.List;

@Slf4j
@RestController
@RequestMapping("/api/configs")
@CrossOrigin(origins = "*")
@Tag(name = "配置管理", description = "配置文件的上传、下载、版本管理和历史查询")
public class ConfigController {

    private final ConfigService configService;

    public ConfigController(ConfigService configService) {
        this.configService = configService;
    }

    /**
     * 上传配置 zip 文件
     */
    @PostMapping("/upload")
    @Operation(
        summary = "上传配置 zip 文件",
        description = "将 .otacle 文件夹打包的 zip 文件上传到指定分组和角色。\n\n" +
                "**要求：**\n" +
                "- 文件格式必须为 .zip\n" +
                "- zip 内必须包含 .otacle 目录\n" +
                "- 文件大小不超过 50MB\n\n" +
                "**流程：**\n" +
                "1. 验证分组成员权限\n" +
                "2. 验证 zip 格式\n" +
                "3. 计算 SHA-256 校验和\n" +
                "4. 自动递增版本号\n" +
                "5. 上传至阿里云 OSS\n" +
                "6. 保存版本记录"
    )
    public ApiResponse<ConfigVersion> uploadConfig(
            @Parameter(hidden = true) @RequestAttribute("userId") Long userId,
            @Parameter(description = "分组 ID", required = true, example = "1")
            @RequestParam("groupId") Long groupId,
            @Parameter(description = "角色 ID", required = true, example = "char001")
            @RequestParam("characterId") String characterId,
            @Parameter(description = "zip 配置文件（必须包含 .otacle 目录）", required = true)
            @RequestParam("file") @jakarta.validation.constraints.NotNull(message = "配置文件不能为空") MultipartFile file) {
        try {
            ConfigVersion configVersion = configService.uploadConfig(groupId, characterId, userId, file);
            return ApiResponse.success(configVersion);
        } catch (IllegalArgumentException e) {
            return ApiResponse.error(400, e.getMessage());
        } catch (Exception e) {
            log.error("上传配置失败", e);
            return ApiResponse.error(500, "上传配置失败: " + e.getMessage());
        }
    }

    /**
     * 下载配置（获取签名 URL）
     */
    @GetMapping("/download")
    @Operation(
        summary = "下载配置（获取签名 URL）",
        description = "获取配置文件的 OSS 签名下载链接（有效期 1 小时）。\n\n" +
                "如果不指定 version 参数，则返回最新版本。"
    )
    public ApiResponse<ConfigDownloadResponse> downloadConfig(
            @Parameter(hidden = true) @RequestAttribute("userId") Long userId,
            @Parameter(description = "分组 ID", required = true, example = "1")
            @RequestParam("groupId") Long groupId,
            @Parameter(description = "角色 ID", required = true, example = "char001")
            @RequestParam("characterId") String characterId,
            @Parameter(description = "版本号（可选，不传则获取最新版本）", example = "1")
            @RequestParam(value = "version", required = false) Integer version) {
        try {
            ConfigDownloadResponse response;
            if (version == null) {
                // 获取最新版本
                response = configService.getLatestConfig(groupId, userId, characterId);
            } else {
                // 获取指定版本
                response = configService.getConfigByVersion(groupId, userId, characterId, version);
            }
            return ApiResponse.success(response);
        } catch (IllegalArgumentException e) {
            return ApiResponse.error(404, e.getMessage());
        } catch (Exception e) {
            log.error("下载配置失败", e);
            return ApiResponse.error(500, "下载配置失败");
        }
    }

    /**
     * 获取最新版本
     */
    @GetMapping("/latest")
    @Operation(
        summary = "获取最新版本配置",
        description = "获取指定分组和角色的最新配置版本的签名下载链接。\n\n" +
                "**返回内容：**\n" +
                "- 配置元数据（版本号文件名校验和等）\n" +
                "- OSS 签名 URL（有效期 1 小时）"
    )
    public ApiResponse<ConfigDownloadResponse> getLatestConfig(
            @Parameter(hidden = true) @RequestAttribute("userId") Long userId,
            @Parameter(description = "分组 ID", required = true, example = "1")
            @RequestParam("groupId") Long groupId,
            @Parameter(description = "角色 ID", required = true, example = "char001")
            @RequestParam("characterId") String characterId) {
        try {
            ConfigDownloadResponse response = configService.getLatestConfig(groupId, userId, characterId);
            return ApiResponse.success(response);
        } catch (IllegalArgumentException e) {
            return ApiResponse.error(404, e.getMessage());
        } catch (Exception e) {
            log.error("获取最新版本失败", e);
            return ApiResponse.error(500, "获取最新版本失败");
        }
    }

    /**
     * 获取版本历史列表
     */
    @GetMapping("/versions")
    @Operation(
        summary = "获取版本历史列表",
        description = "返回指定分组和角色的所有配置版本列表（按版本号降序排列）"
    )
    public ApiResponse<List<ConfigVersion>> getVersionHistory(
            @Parameter(hidden = true) @RequestAttribute("userId") Long userId,
            @Parameter(description = "分组 ID", required = true, example = "1")
            @RequestParam("groupId") Long groupId,
            @Parameter(description = "角色 ID", required = true, example = "char001")
            @RequestParam("characterId") String characterId) {
        try {
            List<ConfigVersion> versions = configService.getVersionHistory(groupId, userId, characterId);
            return ApiResponse.success(versions);
        } catch (IllegalArgumentException e) {
            return ApiResponse.error(403, e.getMessage());
        } catch (Exception e) {
            log.error("获取版本历史失败", e);
            return ApiResponse.error(500, "获取版本历史失败");
        }
    }
}
