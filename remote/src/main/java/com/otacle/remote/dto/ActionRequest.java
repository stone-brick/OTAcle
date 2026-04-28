package com.otacle.remote.dto;

import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.AllArgsConstructor;

/**
 * 动作请求 DTO
 */
@Data
@NoArgsConstructor
@AllArgsConstructor
public class ActionRequest {
    private String name;
    private String description;
    private Long projectId;
    private String actionType;
    private String config;
    private String variables;
}
