package com.otacle.remote.model;

import lombok.Data;
import java.time.LocalDateTime;

@Data
public class GroupMember {
    private Long id;
    private Long accountId;
    private Long groupId;
    private String role; // admin, 成员
    private LocalDateTime joinedAt;
}
