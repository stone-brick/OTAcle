package com.otacle.remote.service;

import com.otacle.remote.dto.ActionRequest;
import com.otacle.remote.model.Action;
import com.otacle.remote.repository.ActionMapper;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import java.time.LocalDateTime;
import java.util.List;

@Service
public class ActionService {
    private final ActionMapper actionMapper;

    public ActionService(ActionMapper actionMapper) {
        this.actionMapper = actionMapper;
    }

    public List<Action> listByProject(Long projectId) {
        return actionMapper.selectByProjectId(projectId);
    }

    public Action getById(Long id) {
        return actionMapper.selectById(id);
    }

    @Transactional
    public Action create(ActionRequest request) {
        Action action = new Action();
        action.setActionType(request.getActionType());
        action.setName(request.getName());
        action.setDescription(request.getDescription());
        action.setConfig(request.getConfig());
        action.setVariables(request.getVariables());
        action.setProjectId(request.getProjectId());
        action.setCreatedAt(LocalDateTime.now());
        action.setUpdatedAt(LocalDateTime.now());
        actionMapper.insert(action);
        return action;
    }

    @Transactional
    public Action update(Long id, ActionRequest request) {
        Action action = actionMapper.selectById(id);
        if (action == null) {
            throw new RuntimeException("动作不存在");
        }
        action.setActionType(request.getActionType());
        action.setName(request.getName());
        action.setDescription(request.getDescription());
        action.setConfig(request.getConfig());
        action.setVariables(request.getVariables());
        action.setUpdatedAt(LocalDateTime.now());
        actionMapper.update(action);
        return action;
    }

    @Transactional
    public void delete(Long id) {
        actionMapper.deleteById(id);
    }
}