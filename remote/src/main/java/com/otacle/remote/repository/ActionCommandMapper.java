package com.otacle.remote.repository;
import com.otacle.remote.model.ActionCommand;
import org.apache.ibatis.annotations.Mapper;
import java.util.List;

@Mapper
public interface ActionCommandMapper {
    int insert(ActionCommand command);
    ActionCommand selectById(Long id);
    List<ActionCommand> selectAll();
    List<ActionCommand> selectByProjectId(Long projectId);
    int update(ActionCommand command);
    int deleteById(Long id);
}