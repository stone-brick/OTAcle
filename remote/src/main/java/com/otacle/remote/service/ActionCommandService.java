package com.otacle.remote.service;

import com.otacle.remote.dto.ActionCommandRequest;
import com.otacle.remote.model.ActionCommand;
import com.otacle.remote.repository.ActionCommandMapper;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import java.util.List;

@Service
public class ActionCommandService {
    private final ActionCommandMapper commandMapper;

    public ActionCommandService(ActionCommandMapper commandMapper) {
        this.commandMapper = commandMapper;
    }

    public List<ActionCommand> getAllCommands() {
        return commandMapper.selectAll();
    }

    public ActionCommand getCommandById(Long id) {
        return commandMapper.selectById(id);
    }

    public List<ActionCommand> getCommandsByProject(Long projectId) {
        return commandMapper.selectByProjectId(projectId);
    }

    @Transactional
    public ActionCommand createCommand(ActionCommandRequest request) {
        ActionCommand command = new ActionCommand();
        command.setProjectId(request.getProjectId());
        command.setParams(request.getParams() != null ? request.getParams().toString() : null);
        command.setExecuteFlags(request.getExecuteFlags());
        command.setExecutedAt(request.getExecutedAt());
        commandMapper.insert(command);
        return command;
    }
}