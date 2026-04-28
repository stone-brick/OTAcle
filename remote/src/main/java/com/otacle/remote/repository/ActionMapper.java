package com.otacle.remote.repository;
import com.otacle.remote.model.Action;
import org.apache.ibatis.annotations.Mapper;
import java.util.List;

@Mapper
public interface ActionMapper {
    int insert(Action action);
    Action selectById(Long id);
    List<Action> selectByProjectId(Long projectId);
    int update(Action action);
    int deleteById(Long id);
}