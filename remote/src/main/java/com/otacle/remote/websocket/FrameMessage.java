package com.otacle.remote.websocket;

public class FrameMessage {
    private Long projectId;
    private Integer frameId;
    private Integer width;
    private Integer height;
    private Long timestamp;
    private java.util.List<java.util.Map<String, Object>> data;

    public Long getProjectId() {
        return projectId;
    }

    public void setProjectId(Long projectId) {
        this.projectId = projectId;
    }

    public Integer getFrameId() {
        return frameId;
    }

    public void setFrameId(Integer frameId) {
        this.frameId = frameId;
    }

    public Integer getWidth() {
        return width;
    }

    public void setWidth(Integer width) {
        this.width = width;
    }

    public Integer getHeight() {
        return height;
    }

    public void setHeight(Integer height) {
        this.height = height;
    }

    public Long getTimestamp() {
        return timestamp;
    }

    public void setTimestamp(Long timestamp) {
        this.timestamp = timestamp;
    }

    public java.util.List<java.util.Map<String, Object>> getData() {
        return data;
    }

    public void setData(java.util.List<java.util.Map<String, Object>> data) {
        this.data = data;
    }
}