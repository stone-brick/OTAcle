-- 创建数据库
CREATE DATABASE IF NOT EXISTS otacle_remote DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

USE otacle_remote;

-- 账号表
CREATE TABLE IF NOT EXISTS account (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '主键ID',
    username VARCHAR(50) NOT NULL UNIQUE COMMENT '用户名',
    password VARCHAR(255) NOT NULL COMMENT '密码（加密）',
    email VARCHAR(100) COMMENT '邮箱',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    INDEX idx_username (username),
    INDEX idx_email (email)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='账号表';

-- 分组表
CREATE TABLE IF NOT EXISTS `group` (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '主键ID',
    name VARCHAR(100) NOT NULL COMMENT '分组名称',
    description TEXT COMMENT '分组描述',
    invite_code VARCHAR(20) NOT NULL UNIQUE COMMENT '邀请码',
    created_by BIGINT NOT NULL COMMENT '创建者ID',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    FOREIGN KEY (created_by) REFERENCES account(id) ON DELETE CASCADE,
    INDEX idx_invite_code (invite_code)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='分组表';

-- 组成员关系表
CREATE TABLE IF NOT EXISTS group_member (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '主键ID',
    account_id BIGINT NOT NULL COMMENT '用户ID',
    group_id BIGINT NOT NULL COMMENT '分组ID',
    role VARCHAR(20) NOT NULL DEFAULT 'member' COMMENT '角色: admin, member',
    joined_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '加入时间',
    FOREIGN KEY (account_id) REFERENCES account(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES `group`(id) ON DELETE CASCADE,
    UNIQUE KEY uk_account_group (account_id, group_id),
    INDEX idx_group_id (group_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='组成员关系表';

-- 配置版本表
CREATE TABLE IF NOT EXISTS config_version (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '主键ID',
    group_id BIGINT NOT NULL COMMENT '分组ID',
    uploader_id BIGINT NOT NULL COMMENT '上传者ID',
    character_id VARCHAR(100) NOT NULL COMMENT '角色ID',
    version INT NOT NULL COMMENT '版本号（自动递增）',
    file_path VARCHAR(500) NOT NULL COMMENT 'OSS文件路径',
    file_name VARCHAR(200) NOT NULL COMMENT '文件名',
    file_size BIGINT NOT NULL COMMENT '文件大小（字节）',
    checksum VARCHAR(64) NOT NULL COMMENT 'SHA-256校验和',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    FOREIGN KEY (group_id) REFERENCES `group`(id) ON DELETE CASCADE,
    FOREIGN KEY (uploader_id) REFERENCES account(id) ON DELETE CASCADE,
    UNIQUE KEY uk_group_character_version (group_id, character_id, version),
    INDEX idx_group_character (group_id, character_id),
    INDEX idx_uploader (uploader_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='配置版本表';
