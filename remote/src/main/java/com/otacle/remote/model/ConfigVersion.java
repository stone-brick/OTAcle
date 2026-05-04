package com.otacle.remote.model;

import lombok.Data;
import java.time.LocalDateTime;

@Data
public class ConfigVersion {
    private Long id;
    private Long groupId;
    private Long uploaderId;
    private String characterId;
    private Integer version;
    private String filePath;
    private String fileName;
    private Long fileSize;
    private String checksum;
    private LocalDateTime createdAt;
}
