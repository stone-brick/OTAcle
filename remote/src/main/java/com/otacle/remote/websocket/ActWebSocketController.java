package com.otacle.remote.websocket;

import com.otacle.remote.dto.ActionCommandRequest;
import com.otacle.remote.service.ActionCommandService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.messaging.handler.annotation.MessageMapping;
import org.springframework.messaging.handler.annotation.SendTo;
import org.springframework.stereotype.Controller;

/**
 * Act Module WebSocket Controller
 * 用于接收动作命令并存储
 */
@Controller
@RequiredArgsConstructor
@Slf4j
public class ActWebSocketController {

    private final ActionCommandService actionCommandService;

    /**
     * 接收动作命令并存储，然后广播给所有订阅者（如桌面应用）
     *
     * @param message 动作命令消息
     */
    @MessageMapping("/act/command")
    @SendTo("/topic/act/commands")
    public CommandMessage receiveCommand(CommandMessage message) {
        log.info("Received command: execute={}, params={}, projectId={}",
                message.getExecute(), message.getParams(), message.getProjectId());

        // 存储到数据库
        try {
            ActionCommandRequest request = new ActionCommandRequest();
           // request.setExecute(message.getExecute());
            //request.setParams(message.getParams());
            request.setProjectId(message.getProjectId());

            actionCommandService.createCommand(request);
        } catch (Exception e) {
            log.error("Failed to save action command", e);
        }

        // 广播给所有订阅者（桌面应用可以订阅此主题来执行命令）
        return message;
    }
}