package com.otacle.remote.dto;

import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.AllArgsConstructor;
import java.util.List;
import java.util.Map;

/**
 * 图像帧请求 DTO
 */
@Data
@NoArgsConstructor
@AllArgsConstructor
public class ImageFrameRequest {
    private Integer frameId;
    private Integer width;
    private Integer height;
    private Long timestamp;
    private List<Map<String, Object>> data; // crop blocks
    private Long projectId;
}
