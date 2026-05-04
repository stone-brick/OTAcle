package com.otacle.remote.util;

import io.jsonwebtoken.Claims;
import io.jsonwebtoken.Jwts;
import io.jsonwebtoken.security.Keys;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

import javax.crypto.SecretKey;
import java.nio.charset.StandardCharsets;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;

/**
 * JWT 工具类
 */
@Slf4j
@Component
public class JwtUtil {

    private static final int MIN_SECRET_LENGTH = 32;

    @Value("${jwt.secret}")
    private String jwtSecret;

    @Value("${jwt.expiration}")
    private Long jwtExpiration;

    /**
     * 检查 JWT Secret 是否满足最小长度要求
     */
    public boolean isSecretValid() {
        if (jwtSecret == null || jwtSecret.isEmpty()) {
            return false;
        }
        byte[] secretBytes = jwtSecret.getBytes(StandardCharsets.UTF_8);
        return secretBytes.length >= MIN_SECRET_LENGTH;
    }

    /**
     * 生成 JWT Token
     */
    public String generateToken(Long userId, String username) {
        if (!isSecretValid()) {
            throw new IllegalStateException("JWT Secret 长度不足，至少需要 " + MIN_SECRET_LENGTH + " 字节");
        }
        SecretKey key = Keys.hmacShaKeyFor(jwtSecret.getBytes(StandardCharsets.UTF_8));
        
        Map<String, Object> claims = new HashMap<>();
        claims.put("userId", userId);
        claims.put("username", username);
        
        Date now = new Date();
        Date expiryDate = new Date(now.getTime() + jwtExpiration);
        
        return Jwts.builder()
                .claims(claims)
                .subject(username)
                .issuedAt(now)
                .expiration(expiryDate)
                .signWith(key)
                .compact();
    }

    /**
     * 从 Token 中获取用户 ID
     */
    public Long getUserIdFromToken(String token) {
        Claims claims = getClaimsFromToken(token);
        return claims != null ? claims.get("userId", Long.class) : null;
    }

    /**
     * 从 Token 中获取用户名
     */
    public String getUsernameFromToken(String token) {
        Claims claims = getClaimsFromToken(token);
        return claims != null ? claims.getSubject() : null;
    }

    /**
     * 验证 Token 是否有效
     */
    public boolean validateToken(String token) {
        try {
            Claims claims = getClaimsFromToken(token);
            return claims != null && !isTokenExpired(claims);
        } catch (Exception e) {
            log.warn("JWT Token 验证失败: {}", e.getMessage());
            return false;
        }
    }

    /**
     * 从 Token 中解析 Claims
     */
    private Claims getClaimsFromToken(String token) {
        try {
            if (!isSecretValid()) {
                throw new IllegalStateException("JWT Secret 长度不足，至少需要 " + MIN_SECRET_LENGTH + " 字节");
            }
            SecretKey key = Keys.hmacShaKeyFor(jwtSecret.getBytes(StandardCharsets.UTF_8));
            return Jwts.parser()
                    .verifyWith(key)
                    .build()
                    .parseSignedClaims(token)
                    .getPayload();
        } catch (Exception e) {
            log.warn("JWT Token 解析失败: {}", e.getMessage());
            return null;
        }
    }

    /**
     * 检查 Token 是否过期
     */
    private boolean isTokenExpired(Claims claims) {
        Date expiration = claims.getExpiration();
        return expiration.before(new Date());
    }
}
