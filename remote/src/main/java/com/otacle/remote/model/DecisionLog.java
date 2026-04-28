package com.otacle.remote.model;

import java.time.LocalDateTime;

public class DecisionLog {
    private Long id;
    private Long projectId;
    private Integer step;
    private String customData;
    private LocalDateTime createdAt;

    // Getter & Setter
    public Long getId() { return id; }
    public void setId(Long id) { this.id = id; }

    public Long getProjectId() { return projectId; }
    public void setProjectId(Long projectId) { this.projectId = projectId; }

    @SuppressWarnings("unused")
    public Integer getStep() { return step; }

    public void setStep(Integer step) { this.step = step; }

    @SuppressWarnings("unused")
    public String getCustomData() { return customData; }

    public void setCustomData(String customData) { this.customData = customData; }

    public LocalDateTime getCreatedAt() { return createdAt; }
    public void setCreatedAt(LocalDateTime createdAt) { this.createdAt = createdAt; }
}