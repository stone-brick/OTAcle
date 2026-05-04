package com.otacle.remote.model;

import lombok.Data;
import java.time.LocalDateTime;

@Data
public class Account {
    private Long id;
    private String username;
    private String password;
    private String email;
    private LocalDateTime createdAt;
    private LocalDateTime updatedAt;
}
