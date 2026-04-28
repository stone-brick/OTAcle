package com.otacle.remote.controller;

import com.otacle.remote.dto.ActionCommandRequest;
import com.otacle.remote.model.ActionCommand;
import com.otacle.remote.service.ActionCommandService;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

/**
 * ActionCommand Controller (Act Module)
 */
@RestController
@RequestMapping("/api/act")
@CrossOrigin(origins = "*")
public class ActionCommandController {

    // 去掉 Lombok，手动写构造器 → 彻底不报错
    private final ActionCommandService actionCommandService;

    public ActionCommandController(ActionCommandService actionCommandService) {
        this.actionCommandService = actionCommandService;
    }

    /**
     * 获取所有动作命令
     */
    @GetMapping
    public ResponseEntity<List<ActionCommand>> getAllCommands() {
        return ResponseEntity.ok(actionCommandService.getAllCommands());
    }

    /**
     * 根据 ID 获取动作命令
     */
    @GetMapping("/{id}")
    public ResponseEntity<ActionCommand> getCommandById(@PathVariable Long id) {
        ActionCommand command = actionCommandService.getCommandById(id);
        if (command == null) {
            return ResponseEntity.notFound().build();
        }
        return ResponseEntity.ok(command);
    }

    /**
     * 获取项目的动作命令
     */
    @GetMapping("/project/{projectId}")
    public ResponseEntity<List<ActionCommand>> getCommandsByProject(@PathVariable Long projectId) {
        return ResponseEntity.ok(actionCommandService.getCommandsByProject(projectId));
    }

    /**
     * 创建动作命令
     */
    @PostMapping
    public ResponseEntity<ActionCommand> createCommand(@RequestBody ActionCommandRequest request) {
        return ResponseEntity.ok(actionCommandService.createCommand(request));
    }
}