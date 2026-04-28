package com.otacle.remote.repository;
import com.otacle.remote.model.ImageFrame;
import org.apache.ibatis.annotations.Mapper;
import java.util.List;

@Mapper
public interface ImageFrameMapper {
    int insert(ImageFrame frame);
    List<ImageFrame> selectByProjectId(Long projectId);
    int deleteById(Long id);
}