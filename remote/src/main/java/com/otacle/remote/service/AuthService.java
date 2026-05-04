package com.otacle.remote.service;

import com.otacle.remote.dto.auth.AuthResponse;
import com.otacle.remote.dto.auth.LoginRequest;
import com.otacle.remote.dto.auth.RegisterRequest;
import com.otacle.remote.model.Account;
import com.otacle.remote.repository.AccountMapper;
import com.otacle.remote.util.JwtUtil;
import lombok.extern.slf4j.Slf4j;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

@Slf4j
@Service
public class AuthService {

    private final AccountMapper accountMapper;
    private final PasswordEncoder passwordEncoder;
    private final JwtUtil jwtUtil;

    public AuthService(AccountMapper accountMapper, PasswordEncoder passwordEncoder, JwtUtil jwtUtil) {
        this.accountMapper = accountMapper;
        this.passwordEncoder = passwordEncoder;
        this.jwtUtil = jwtUtil;
    }

    @Transactional
    public AuthResponse register(RegisterRequest request) {
        // 检查用户名是否已存在
        if (accountMapper.findByUsername(request.getUsername()) != null) {
            throw new IllegalArgumentException("用户名已存在");
        }

        // 创建账号
        Account account = new Account();
        account.setUsername(request.getUsername());
        account.setPassword(passwordEncoder.encode(request.getPassword()));
        account.setEmail(request.getEmail());
        
        accountMapper.insert(account);
        
        // 生成 JWT token
        String token = jwtUtil.generateToken(account.getId(), account.getUsername());
        
        log.info("用户注册成功: userId={}, username={}", account.getId(), account.getUsername());
        
        return new AuthResponse(token, account.getId(), account.getUsername());
    }

    public AuthResponse login(LoginRequest request) {
        Account account = accountMapper.findByUsername(request.getUsername());
        
        if (account == null || !passwordEncoder.matches(request.getPassword(), account.getPassword())) {
            log.warn("登录失败: username={}", request.getUsername());
            throw new IllegalArgumentException("用户名或密码错误");
        }
        
        String token = jwtUtil.generateToken(account.getId(), account.getUsername());
        
        log.info("用户登录成功: userId={}, username={}", account.getId(), account.getUsername());
        
        return new AuthResponse(token, account.getId(), account.getUsername());
    }

    public Account getCurrentUser(Long userId) {
        Account account = accountMapper.findById(userId);
        if (account == null) {
            throw new IllegalArgumentException("用户不存在");
        }
        return account;
    }
}
