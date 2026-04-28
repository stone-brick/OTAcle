package com.otacle.remote.dto;

import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.AllArgsConstructor;
import java.util.Map;

/**
 * 决策日志请求 DTO
 */
@Data
@NoArgsConstructor
@AllArgsConstructor
public class DecisionLogRequest {
    private Integer step;
    private Map<String, Object> custom;
    private Long projectId;
}
