package com.otacle.remote.repository;
import com.otacle.remote.model.Project;
import org.apache.ibatis.annotations.Mapper;
import java.util.List;

@Mapper
public interface ProjectMapper {
    int insert(Project project);
    Project selectById(Long id);
    List<Project> selectAll();
    int update(Project project);
    int deleteById(Long id);
    Project selectByName(String name);
}