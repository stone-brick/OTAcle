package com.otacle.remote.service;

import com.otacle.remote.dto.config.ConfigDownloadResponse;
import com.otacle.remote.model.ConfigVersion;
import com.otacle.remote.repository.ConfigVersionMapper;
import com.otacle.remote.repository.GroupMemberMapper;
import com.otacle.remote.service.storage.StorageService;
import lombok.extern.slf4j.Slf4j;
import org.apache.commons.compress.archivers.zip.ZipArchiveEntry;
import org.apache.commons.compress.archivers.zip.ZipArchiveInputStream;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;
import java.io.InputStream;
import java.security.MessageDigest;
import java.time.format.DateTimeFormatter;
import java.util.List;

@Slf4j
@Service
public class ConfigService {

    private final ConfigVersionMapper configVersionMapper;
    private final GroupMemberMapper groupMemberMapper;
    private final StorageService storageService;

    public ConfigService(ConfigVersionMapper configVersionMapper,
                        GroupMemberMapper groupMemberMapper,
                        StorageService storageService) {
        this.configVersionMapper = configVersionMapper;
        this.groupMemberMapper = groupMemberMapper;
        this.storageService = storageService;
    }

    /**
     * 上传配置文件
     */
    @Transactional
    public ConfigVersion uploadConfig(Long groupId, String characterId, Long uploaderId, MultipartFile file) throws IOException {
        log.info("开始上传配置: groupId={}, characterId={}, uploaderId={}, fileName={}", 
                groupId, characterId, uploaderId, file.getOriginalFilename());
        
        // 1. 验证分组成员权限
        validateGroupMember(groupId, uploaderId);
        
        // 2. 验证 zip 文件格式
        validateZipFile(file);
        
        // 3. 计算文件校验和
        String checksum = calculateChecksum(file);
        
        // 4. 获取下一个版本号
        Integer nextVersion = getNextVersion(groupId, characterId);
        
        // 5. 生成 OSS 对象键
        String objectKey = storageService.generateObjectKey(groupId, characterId, nextVersion);
        
        // 6. 上传到 OSS
        storageService.uploadFile(file, objectKey);
        
        // 7. 保存版本记录
        ConfigVersion configVersion = new ConfigVersion();
        configVersion.setGroupId(groupId);
        configVersion.setUploaderId(uploaderId);
        configVersion.setCharacterId(characterId);
        configVersion.setVersion(nextVersion);
        configVersion.setFilePath(objectKey);
        configVersion.setFileName(file.getOriginalFilename());
        configVersion.setFileSize(file.getSize());
        configVersion.setChecksum(checksum);
        
        configVersionMapper.insert(configVersion);
        
        log.info("配置上传成功: groupId={}, characterId={}, version={}, filePath={}", 
                groupId, characterId, nextVersion, objectKey);
        
        return configVersion;
    }

    /**
     * 获取最新版本配置（返回签名 URL）
     */
    public ConfigDownloadResponse getLatestConfig(Long groupId, Long userId, String characterId) {
        log.debug("获取最新配置: groupId={}, characterId={}", groupId, characterId);
        
        // 验证权限
        validateGroupMember(groupId, userId);
        
        // 查询最新版本
        ConfigVersion latestVersion = configVersionMapper.findLatestVersion(groupId, characterId);
        
        if (latestVersion == null) {
            throw new IllegalArgumentException("未找到配置版本");
        }
        
        // 生成签名 URL
        String downloadUrl = storageService.generatePresignedUrl(latestVersion.getFilePath());
        
        return convertToDownloadResponse(latestVersion, downloadUrl);
    }

    /**
     * 获取指定版本配置
     */
    public ConfigDownloadResponse getConfigByVersion(Long groupId, Long userId, String characterId, Integer version) {
        log.debug("获取指定版本配置: groupId={}, characterId={}, version={}", groupId, characterId, version);
        
        // 验证权限
        validateGroupMember(groupId, userId);
        
        // 查询指定版本
        ConfigVersion configVersion = configVersionMapper.findByGroupAndCharacterAndVersion(groupId, characterId, version);
        
        if (configVersion == null) {
            throw new IllegalArgumentException("配置版本不存在");
        }
        
        // 生成签名 URL
        String downloadUrl = storageService.generatePresignedUrl(configVersion.getFilePath());
        
        return convertToDownloadResponse(configVersion, downloadUrl);
    }

    /**
     * 获取版本历史列表
     */
    public List<ConfigVersion> getVersionHistory(Long groupId, Long userId, String characterId) {
        log.debug("获取版本历史: groupId={}, characterId={}", groupId, characterId);
        
        // 验证权限
        validateGroupMember(groupId, userId);
        
        return configVersionMapper.findVersionsByGroupAndCharacter(groupId, characterId);
    }

    /**
     * 验证分组成员
     */
    private void validateGroupMember(Long groupId, Long userId) {
        int count = groupMemberMapper.countByAccountAndGroup(userId, groupId);
        if (count == 0) {
            log.warn("用户不是分组成员: userId={}, groupId={}", userId, groupId);
            throw new IllegalArgumentException("您不是该分组的成员");
        }
    }

    /**
     * 验证 zip 文件
     */
    private void validateZipFile(MultipartFile file) throws IOException {
        String filename = file.getOriginalFilename();
        if (filename == null || !filename.toLowerCase().endsWith(".zip")) {
            throw new IllegalArgumentException("只支持 zip 格式文件");
        }
        
        // 验证 zip 内容是否包含 .otacle 目录
        try (ZipArchiveInputStream zis = new ZipArchiveInputStream(file.getInputStream())) {
            boolean hasOtacleDir = false;
            ZipArchiveEntry entry;
            
            while ((entry = zis.getNextZipEntry()) != null) {
                if (entry.getName().startsWith(".otacle/") || entry.getName().equals(".otacle")) {
                    hasOtacleDir = true;
                    break;
                }
            }
            
            if (!hasOtacleDir) {
                throw new IllegalArgumentException("zip 文件必须包含 .otacle 目录");
            }
        }
    }

    /**
     * 计算文件 SHA-256 校验和
     */
    private String calculateChecksum(MultipartFile file) throws IOException {
        try (InputStream is = file.getInputStream()) {
            MessageDigest digest = MessageDigest.getInstance("SHA-256");
            byte[] buffer = new byte[8192];
            int read;
            while ((read = is.read(buffer)) != -1) {
                digest.update(buffer, 0, read);
            }
            byte[] hash = digest.digest();

            StringBuilder hexString = new StringBuilder();
            for (byte b : hash) {
                String hex = Integer.toHexString(0xff & b);
                if (hex.length() == 1) hexString.append('0');
                hexString.append(hex);
            }

            return hexString.toString();
        } catch (Exception e) {
            throw new IOException("计算校验和失败", e);
        }
    }

    /**
     * 获取下一个版本号
     */
    private Integer getNextVersion(Long groupId, String characterId) {
        Integer maxVersion = configVersionMapper.findMaxVersion(groupId, characterId);
        return (maxVersion == null) ? 1 : maxVersion + 1;
    }

    /**
     * 转换为下载响应
     */
    private ConfigDownloadResponse convertToDownloadResponse(ConfigVersion configVersion, String downloadUrl) {
        ConfigDownloadResponse response = new ConfigDownloadResponse();
        response.setConfigId(configVersion.getId());
        response.setCharacterId(configVersion.getCharacterId());
        response.setVersion(configVersion.getVersion());
        response.setFileName(configVersion.getFileName());
        response.setFileSize(configVersion.getFileSize());
        response.setChecksum(configVersion.getChecksum());
        response.setDownloadUrl(downloadUrl);
        response.setCreatedAt(configVersion.getCreatedAt().format(DateTimeFormatter.ISO_LOCAL_DATE_TIME));
        
        return response;
    }
}
