package com.otacle.remote.model;
import java.time.LocalDateTime;

public class ActionCommand {
    private Long id;
    private Long projectId;
    private String params;
    private String executeFlags;
    private LocalDateTime createdAt;
    private LocalDateTime executedAt;

    // Getter & Setter
    public Long getId() { return id; }
    public void setId(Long id) { this.id = id; }
    public Long getProjectId() { return projectId; }
    public void setProjectId(Long projectId) { this.projectId = projectId; }
    public String getParams() { return params; }
    public void setParams(String params) { this.params = params; }
    public String getExecuteFlags() { return executeFlags; }
    public void setExecuteFlags(String executeFlags) { this.executeFlags = executeFlags; }
    public LocalDateTime getCreatedAt() { return createdAt; }
    public void setCreatedAt(LocalDateTime createdAt) { this.createdAt = createdAt; }
    public LocalDateTime getExecutedAt() { return executedAt; }
    public void setExecutedAt(LocalDateTime executedAt) { this.executedAt = executedAt; }
}