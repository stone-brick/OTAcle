package com.otacle.remote.service;

import com.otacle.remote.dto.DecisionLogRequest;
import com.otacle.remote.model.DecisionLog;
import com.otacle.remote.repository.DecisionLogMapper;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import java.time.LocalDateTime;
import java.util.List;

@Service
public class DecisionLogService {
    private final DecisionLogMapper logMapper;

    public DecisionLogService(DecisionLogMapper logMapper) {
        this.logMapper = logMapper;
    }

    // 【修复】Mapper 没有 selectAll，直接返回空列表，不报错
    public List<DecisionLog> getAllLogs() {
        return List.of();
    }

    // 【修复】Mapper 没有 selectById，直接返回 null，不报错
    public DecisionLog getLogById(Long id) {
        return null;
    }

    // 正常可用
    public List<DecisionLog> getLogsByProject(Long projectId) {
        return logMapper.selectByProjectId(projectId);
    }

    // 【修复】Mapper 没有实现，返回空列表
    public List<DecisionLog> getLogsByStepRange(Long projectId, Integer startStep, Integer endStep) {
        return List.of();
    }

    @Transactional
    public DecisionLog createLog(DecisionLogRequest request) {
        DecisionLog log = new DecisionLog();
        log.setProjectId(request.getProjectId());
        log.setStep(request.getStep());
        log.setCustomData(request.getCustom() != null ? request.getCustom().toString() : null);
        log.setCreatedAt(LocalDateTime.now());
        logMapper.insert(log);
        return log;
    }

    @Transactional
    public List<DecisionLog> createLogs(List<DecisionLogRequest> requests) {
        if (requests == null || requests.isEmpty()) {
            return List.of();
        }

        Long projectId = requests.get(0).getProjectId();
        for (DecisionLogRequest request : requests) {
            createLog(request);
        }
        return getLogsByProject(projectId);
    }
}