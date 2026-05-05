package com.otacle.remote.controller;

import com.otacle.remote.service.storage.LocalStorageService;
import lombok.extern.slf4j.Slf4j;
import org.springframework.core.io.Resource;
import org.springframework.http.HttpHeaders;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.util.UriUtils;

import java.net.MalformedURLException;
import java.nio.file.Path;
import java.nio.file.Paths;

/**
 * 本地存储文件访问 Controller
 * 提供 /api/storage/files/{objectKey} 端点访问本地文件
 */
@Slf4j
@RestController
@RequestMapping("/api/storage/files")
public class StorageController {

    private final LocalStorageService localStorageService;

    public StorageController(LocalStorageService localStorageService) {
        this.localStorageService = localStorageService;
    }

    @GetMapping("/**")
    public ResponseEntity<Resource> serveFile(@RequestParam("path") String path) throws MalformedURLException {
        // 解码 URL 编码的路径
        String decodedPath = UriUtils.decode(path, "UTF-8");
        log.info("访问本地文件: {}", decodedPath);
        Resource file = localStorageService.loadFileAsResource(decodedPath);
        return ResponseEntity.ok()
                .header(HttpHeaders.CONTENT_DISPOSITION, "attachment; filename=\"" + file.getFilename() + "\"")
                .contentType(MediaType.APPLICATION_OCTET_STREAM)
                .body(file);
    }
}