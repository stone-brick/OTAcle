package com.otacle.remote.service;

import com.otacle.remote.dto.ProjectRequest;
import com.otacle.remote.model.Project;
import com.otacle.remote.repository.ProjectMapper;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import java.time.LocalDateTime;
import java.util.List;

@Service
public class ProjectService {
    private final ProjectMapper projectMapper;

    public ProjectService(ProjectMapper projectMapper) {
        this.projectMapper = projectMapper;
    }

    public List<Project> list() {
        return projectMapper.selectAll();
    }

    public Project getById(Long id) {
        return projectMapper.selectById(id);
    }

    @Transactional
    public Project create(ProjectRequest request) {
        if (projectMapper.selectByName(request.getName()) != null) {
            throw new RuntimeException("项目名称已存在");
        }
        Project project = new Project();
        project.setName(request.getName());
        project.setDescription(request.getDescription());
        project.setProjectPath(request.getProjectPath());
        project.setCreatedAt(LocalDateTime.now());
        project.setUpdatedAt(LocalDateTime.now());
        projectMapper.insert(project);
        return project;
    }

    @Transactional
    public Project update(Long id, ProjectRequest request) {
        Project project = projectMapper.selectById(id);
        if (project == null) {
            throw new RuntimeException("项目不存在");
        }
        project.setName(request.getName());
        project.setDescription(request.getDescription());
        project.setProjectPath(request.getProjectPath());
        project.setUpdatedAt(LocalDateTime.now());
        projectMapper.update(project);
        return project;
    }

    @Transactional
    public void delete(Long id) {
        projectMapper.deleteById(id);
    }
}