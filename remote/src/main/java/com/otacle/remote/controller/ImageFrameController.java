package com.otacle.remote.controller;

import com.otacle.remote.dto.ImageFrameRequest;
import com.otacle.remote.model.ImageFrame;
import com.otacle.remote.service.ImageFrameService;
import lombok.RequiredArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

/**
 * ImageFrame Controller (Observe Module)
 */
@RestController
@RequestMapping("/api/observe")
@RequiredArgsConstructor
@CrossOrigin(origins = "*")
public class ImageFrameController {

    private final ImageFrameService imageFrameService;

    /**
     * 获取所有图像帧
     */
    @GetMapping
    public ResponseEntity<List<ImageFrame>> getAllFrames() {
        return ResponseEntity.ok(imageFrameService.listAll());
    }

    /**
     * 根据 ID 获取图像帧
     */
    @GetMapping("/{id}")
    public ResponseEntity<ImageFrame> getFrameById(@PathVariable Long id) {
        ImageFrame frame = imageFrameService.getById(id);
        if (frame == null) {
            return ResponseEntity.notFound().build();
        }
        return ResponseEntity.ok(frame);
    }

    /**
     * 获取项目的图像帧
     */
    @GetMapping("/project/{projectId}")
    public ResponseEntity<List<ImageFrame>> getFramesByProject(@PathVariable Long projectId) {
        return ResponseEntity.ok(imageFrameService.listByProject(projectId));
    }

    /**
     * 获取项目最新的图像帧
     */
    @GetMapping("/project/{projectId}/latest")
    public ResponseEntity<ImageFrame> getLatestFrameByProject(@PathVariable Long projectId) {
        ImageFrame frame = imageFrameService.getLatestByProject(projectId);
        if (frame == null) {
            return ResponseEntity.notFound().build();
        }
        return ResponseEntity.ok(frame);
    }

    /**
     * 创建图像帧
     */
    @PostMapping
    public ResponseEntity<ImageFrame> createFrame(@RequestBody ImageFrameRequest request) {
        return ResponseEntity.ok(imageFrameService.create(request));
    }
}