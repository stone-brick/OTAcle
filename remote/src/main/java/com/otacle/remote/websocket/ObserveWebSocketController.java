package com.otacle.remote.websocket;

import com.otacle.remote.dto.ImageFrameRequest;
import com.otacle.remote.service.ImageFrameService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.messaging.handler.annotation.MessageMapping;
import org.springframework.messaging.handler.annotation.SendTo;
import org.springframework.stereotype.Controller;

@Controller
public class ObserveWebSocketController {
    private static final Logger log = LoggerFactory.getLogger(ObserveWebSocketController.class);

    private final ImageFrameService imageFrameService;

    public ObserveWebSocketController(ImageFrameService imageFrameService) {
        this.imageFrameService = imageFrameService;
    }

    @MessageMapping("/observe/frame")
    @SendTo("/topic/observe/frames")
    public FrameMessage handle(FrameMessage message) {
        try {
            ImageFrameRequest request = new ImageFrameRequest();
            request.setProjectId(message.getProjectId());
            request.setFrameId(message.getFrameId());
            request.setWidth(message.getWidth());
            request.setHeight(message.getHeight());
            request.setTimestamp(message.getTimestamp());
            // 【修复1】去掉不存在的 setCropBlocks 调用
            // request.setCropBlocks(message.getData().toString());

            // 【修复2】使用 Service 里正确的方法名 create
            imageFrameService.create(request);
        } catch (Exception e) {
            log.error("Failed to save image frame", e);
        }
        return message;
    }
}