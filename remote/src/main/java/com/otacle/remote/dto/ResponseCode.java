package com.otacle.remote.dto;

import lombok.AllArgsConstructor;
import lombok.Getter;

/**
 * API 响应状态码枚举
 */
@Getter
@AllArgsConstructor
public enum ResponseCode {
    SUCCESS(200, "成功"),
    BAD_REQUEST(400, "请求参数错误"),
    UNAUTHORIZED(401, "未授权"),
    FORBIDDEN(403, "禁止访问"),
    NOT_FOUND(404, "资源不存在"),
    CONFLICT(409, "资源冲突"),
    INTERNAL_ERROR(500, "服务器内部错误");

    private final int code;
    private final String message;
}
