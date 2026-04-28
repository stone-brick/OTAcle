package com.otacle.remote.controller;

import com.otacle.remote.dto.DecisionLogRequest;
import com.otacle.remote.model.DecisionLog;
import com.otacle.remote.service.DecisionLogService;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

/**
 * DecisionLog Controller (Think Module)
 */
@RestController
@RequestMapping("/api/think")
@CrossOrigin(origins = "*")
public class DecisionLogController {

    // 去掉 Lombok，手动写构造器
    private final DecisionLogService decisionLogService;

    public DecisionLogController(DecisionLogService decisionLogService) {
        this.decisionLogService = decisionLogService;
    }

    /**
     * 获取所有决策日志
     */
    @GetMapping
    public ResponseEntity<List<DecisionLog>> getAllLogs() {
        return ResponseEntity.ok(decisionLogService.getAllLogs());
    }

    /**
     * 根据 ID 获取决策日志
     */
    @GetMapping("/{id}")
    public ResponseEntity<DecisionLog> getLogById(@PathVariable Long id) {
        DecisionLog log = decisionLogService.getLogById(id);
        if (log == null) {
            return ResponseEntity.notFound().build();
        }
        return ResponseEntity.ok(log);
    }

    /**
     * 获取项目的决策日志
     */
    @GetMapping("/project/{projectId}")
    public ResponseEntity<List<DecisionLog>> getLogsByProject(@PathVariable Long projectId) {
        return ResponseEntity.ok(decisionLogService.getLogsByProject(projectId));
    }

    /**
     * 获取项目指定范围的决策日志
     */
    @GetMapping("/project/{projectId}/steps")
    public ResponseEntity<List<DecisionLog>> getLogsByStepRange(
            @PathVariable Long projectId,
            @RequestParam Integer startStep,
            @RequestParam Integer endStep
    ) {
        return ResponseEntity.ok(
                decisionLogService.getLogsByStepRange(projectId, startStep, endStep)
        );
    }

    /**
     * 创建决策日志
     */
    @PostMapping
    public ResponseEntity<DecisionLog> createLog(@RequestBody DecisionLogRequest request) {
        return ResponseEntity.ok(decisionLogService.createLog(request));
    }

    /**
     * 批量创建决策日志
     */
    @PostMapping("/batch")
    public ResponseEntity<List<DecisionLog>> createLogs(@RequestBody List<DecisionLogRequest> requests) {
        return ResponseEntity.ok(decisionLogService.createLogs(requests));
    }
}