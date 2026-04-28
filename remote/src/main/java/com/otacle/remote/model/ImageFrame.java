package com.otacle.remote.model;
import java.time.LocalDateTime;

public class ImageFrame {
    private Long id;
    private Long projectId;
    private Integer frameId;
    private Integer width;
    private Integer height;
    private Long timestamp;
    private String cropBlocks;
    private LocalDateTime createdAt;

    // Getter & Setter
    public Long getId() { return id; }
    public void setId(Long id) { this.id = id; }
    public Long getProjectId() { return projectId; }
    public void setProjectId(Long projectId) { this.projectId = projectId; }
    public Integer getFrameId() { return frameId; }
    public void setFrameId(Integer frameId) { this.frameId = frameId; }
    public Integer getWidth() { return width; }
    public void setWidth(Integer width) { this.width = width; }
    public Integer getHeight() { return height; }
    public void setHeight(Integer height) { this.height = height; }
    public Long getTimestamp() { return timestamp; }
    public void setTimestamp(Long timestamp) { this.timestamp = timestamp; }
    public String getCropBlocks() { return cropBlocks; }
    public void setCropBlocks(String cropBlocks) { this.cropBlocks = cropBlocks; }
    public LocalDateTime getCreatedAt() { return createdAt; }
    public void setCreatedAt(LocalDateTime createdAt) { this.createdAt = createdAt; }
}