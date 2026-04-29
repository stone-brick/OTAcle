package com.otacle.remote.websocket;

import com.otacle.remote.dto.DecisionLogRequest;
import com.otacle.remote.service.DecisionLogService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.messaging.handler.annotation.MessageMapping;
import org.springframework.messaging.handler.annotation.SendTo;
import org.springframework.stereotype.Controller;

@Controller
public class ThinkWebSocketController {

    // 日志
    private static final Logger log = LoggerFactory.getLogger(ThinkWebSocketController.class);

    // 注入 Service
    private final DecisionLogService decisionLogService;

    // 手动构造器，彻底解决 Lombok 报错
    public ThinkWebSocketController(DecisionLogService decisionLogService) {
        this.decisionLogService = decisionLogService;
    }

    /**
     * 接收前端思考消息 -> 保存日志 -> 推送给所有订阅者
     */
    @MessageMapping("/think/log")
    @SendTo("/topic/think/logs")
    public DecisionMessage handle(DecisionMessage message) {
        try {
            // 1. 把 WebSocket 消息转换成 DTO
            DecisionLogRequest request = new DecisionLogRequest();
            request.setProjectId(message.getProjectId());
            request.setStep(message.getStep());
           // request.setCustomData(message.getCustomData());

            // 2. 保存到数据库（MyBatis 版本）
           // decisionLogService.create(request);

            log.info("思考日志已保存：projectId={}", message.getProjectId());
        } catch (Exception e) {
            log.error("保存思考日志失败", e);
        }

        // 3. 返回消息给前端
        return message;
    }
}