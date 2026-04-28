package com.otacle.remote.dto;

import java.time.LocalDateTime;
import java.util.List;
import java.util.Map;

/**
 * 动作命令请求 DTO
 */
public class ActionCommandRequest {
    private List<Boolean> execute;
    private Map<String, Object> params;
    private Long projectId;
    private String executeFlags;
    private LocalDateTime executedAt;

    // Getter & Setter
    public List<Boolean> getExecute() {
        return execute;
    }

    public void setExecute(List<Boolean> execute) {
        this.execute = execute;
    }

    public Map<String, Object> getParams() {
        return params;
    }

    public void setParams(Map<String, Object> params) {
        this.params = params;
    }

    public Long getProjectId() {
        return projectId;
    }

    public void setProjectId(Long projectId) {
        this.projectId = projectId;
    }

    public String getExecuteFlags() {
        return executeFlags;
    }

    public void setExecuteFlags(String executeFlags) {
        this.executeFlags = executeFlags;
    }

    public LocalDateTime getExecutedAt() {
        return executedAt;
    }

    public void setExecutedAt(LocalDateTime executedAt) {
        this.executedAt = executedAt;
    }
}