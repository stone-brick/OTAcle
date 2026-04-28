package com.otacle.remote.websocket;

import lombok.Data;

@Data
public class CommandMessage {
    private String execute;
    private String params;
    private Long projectId;
}