package com.otacle.remote.repository;
import com.otacle.remote.model.DecisionLog;
import org.apache.ibatis.annotations.Mapper;
import java.util.List;

@Mapper
public interface DecisionLogMapper {
    int insert(DecisionLog log);
    List<DecisionLog> selectByProjectId(Long projectId);
    int deleteById(Long id);
}