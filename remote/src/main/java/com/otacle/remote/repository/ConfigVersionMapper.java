package com.otacle.remote.repository;

import com.otacle.remote.model.ConfigVersion;
import org.apache.ibatis.annotations.*;
import java.util.List;

@Mapper
public interface ConfigVersionMapper {
    
    @Insert("INSERT INTO config_version (group_id, uploader_id, character_id, version, file_path, file_name, file_size, checksum) " +
            "VALUES (#{groupId}, #{uploaderId}, #{characterId}, #{version}, #{filePath}, #{fileName}, #{fileSize}, #{checksum})")
    @Options(useGeneratedKeys = true, keyProperty = "id")
    int insert(ConfigVersion configVersion);

    @Select("SELECT * FROM config_version WHERE group_id = #{groupId} AND character_id = #{characterId} AND version = #{version}")
    ConfigVersion findByGroupAndCharacterAndVersion(
            @Param("groupId") Long groupId, 
            @Param("characterId") String characterId, 
            @Param("version") Integer version);

    @Select("SELECT * FROM config_version WHERE group_id = #{groupId} AND character_id = #{characterId} ORDER BY version DESC LIMIT 1")
    ConfigVersion findLatestVersion(
            @Param("groupId") Long groupId, 
            @Param("characterId") String characterId);

    @Select("SELECT * FROM config_version WHERE group_id = #{groupId} AND character_id = #{characterId} ORDER BY version DESC")
    List<ConfigVersion> findVersionsByGroupAndCharacter(
            @Param("groupId") Long groupId, 
            @Param("characterId") String characterId);

    @Select("SELECT IFNULL(MAX(version), 0) FROM config_version WHERE group_id = #{groupId} AND character_id = #{characterId}")
    Integer findMaxVersion(
            @Param("groupId") Long groupId, 
            @Param("characterId") String characterId);
}
