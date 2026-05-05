package com.otacle.remote.service.storage;

import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;

/**
 * 存储服务接口，抽象化存储操作
 */
public interface StorageService {

    /**
     * 上传文件
     *
     * @param file      上传的文件
     * @param objectKey 对象键（路径）
     * @return 存储后的路径（作为数据库 record 的 file_path）
     * @throws IOException 上传失败
     */
    String uploadFile(MultipartFile file, String objectKey) throws IOException;

    /**
     * 生成下载/访问 URL
     *
     * @param objectKey 对象键
     * @return 可访问的 URL 字符串
     */
    String generatePresignedUrl(String objectKey);

    /**
     * 生成对象键（路径）
     * 格式: otacle-config/{group_id}/{character_id}/{group_id}_v{version}_{uuid}.zip
     *
     * @param groupId     分组 ID
     * @param characterId 角色 ID
     * @param version     版本号
     * @return 对象键字符串
     */
    String generateObjectKey(Long groupId, String characterId, Integer version);
}