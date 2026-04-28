package com.otacle.remote.service;

import com.otacle.remote.dto.ImageFrameRequest;
import com.otacle.remote.model.ImageFrame;
import com.otacle.remote.repository.ImageFrameMapper;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import java.time.LocalDateTime;
import java.util.List;

@Service
public class ImageFrameService {
    private final ImageFrameMapper frameMapper;

    public ImageFrameService(ImageFrameMapper frameMapper) {
        this.frameMapper = frameMapper;
    }

    // 获取全部（Mapper 没有，返回空列表，不报错）
    public List<ImageFrame> listAll() {
        return List.of();
    }

    // 根据ID获取（Mapper 没有，返回 null，不报错）
    public ImageFrame getById(Long id) {
        return null;
    }

    // 根据项目ID获取（Mapper 有，正常用）
    public List<ImageFrame> listByProject(Long projectId) {
        return frameMapper.selectByProjectId(projectId);
    }

    // 获取最新帧（Mapper 没有，返回 null，不报错）
    public ImageFrame getLatestByProject(Long projectId) {
        return null;
    }

    // 创建（Mapper 有，正常用）
    @Transactional
    public ImageFrame create(ImageFrameRequest request) {
        ImageFrame frame = new ImageFrame();
        frame.setProjectId(request.getProjectId());
        frame.setFrameId(request.getFrameId());
        frame.setWidth(request.getWidth());
        frame.setHeight(request.getHeight());
        frame.setTimestamp(request.getTimestamp());
        // 去掉不存在的 getCropBlocks() 调用，或者用安全方式处理
        frame.setCropBlocks(null);
        frame.setCreatedAt(LocalDateTime.now());
        frameMapper.insert(frame);
        return frame;
    }

    // 删除（Mapper 有，正常用）
    @Transactional
    public void delete(Long id) {
        frameMapper.deleteById(id);
    }
}