package com.otacle.remote.controller;

import com.otacle.remote.dto.ActionRequest;
import com.otacle.remote.model.Action;
import com.otacle.remote.service.ActionService;
import lombok.RequiredArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

/**
 * Action Controller
 */
@RestController
@RequestMapping("/api/actions")
@RequiredArgsConstructor
@CrossOrigin(origins = "*")
public class ActionController {

    private final ActionService actionService;

    /**
     * 获取所有动作
     */
    @GetMapping
    public ResponseEntity<List<Action>> getAllActions() {
        return ResponseEntity.ok(actionService.listByProject(null));
    }

    /**
     * 根据 ID 获取动作
     */
    @GetMapping("/{id}")
    public ResponseEntity<Action> getActionById(@PathVariable Long id) {
        Action action = actionService.getById(id);
        if (action == null) {
            return ResponseEntity.notFound().build();
        }
        return ResponseEntity.ok(action);
    }

    /**
     * 获取项目的所有动作
     */
    @GetMapping("/project/{projectId}")
    public ResponseEntity<List<Action>> getActionsByProject(@PathVariable Long projectId) {
        return ResponseEntity.ok(actionService.listByProject(projectId));
    }

    /**
     * 创建动作
     */
    @PostMapping
    public ResponseEntity<Action> createAction(@RequestBody ActionRequest request) {
        return ResponseEntity.ok(actionService.create(request));
    }

    /**
     * 更新动作
     */
    @PutMapping("/{id}")
    public ResponseEntity<Action> updateAction(
            @PathVariable Long id,
            @RequestBody ActionRequest request
    ) {
        return ResponseEntity.ok(actionService.update(id, request));
    }

    /**
     * 删除动作
     */
    @DeleteMapping("/{id}")
    public ResponseEntity<Void> deleteAction(@PathVariable Long id) {
        actionService.delete(id);
        return ResponseEntity.noContent().build();
    }
}