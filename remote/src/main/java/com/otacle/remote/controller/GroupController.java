package com.otacle.remote.controller;

import com.otacle.remote.dto.ApiResponse;
import com.otacle.remote.model.Group;
import com.otacle.remote.model.GroupMember;
import com.otacle.remote.service.GroupService;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.tags.Tag;
import jakarta.validation.Valid;
import lombok.Data;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@Slf4j
@RestController
@RequestMapping("/api/groups")
@CrossOrigin(origins = "*")
@Tag(name = "分组管理", description = "分组的创建、查询、更新、删除和成员管理")
public class GroupController {

    private final GroupService groupService;

    public GroupController(GroupService groupService) {
        this.groupService = groupService;
    }

    /**
     * 获取我的分组列表
     */
    @GetMapping
    @Operation(summary = "获取我的分组列表", description = "返回当前用户加入的所有分组列表")
    public ApiResponse<List<Group>> getMyGroups(@Parameter(hidden = true) @RequestAttribute("userId") Long userId) {
        try {
            List<Group> groups = groupService.getUserGroups(userId);
            return ApiResponse.success(groups);
        } catch (Exception e) {
            log.error("获取分组列表失败", e);
            return ApiResponse.error(500, "获取分组列表失败");
        }
    }

    /**
     * 创建分组
     */
    @PostMapping
    @Operation(summary = "创建分组", description = "创建新分组，创建者自动成为管理员，系统自动生成邀请码")
    public ApiResponse<Group> createGroup(@Parameter(hidden = true) @RequestAttribute("userId") Long userId, @Valid @RequestBody CreateGroupRequest request) {
        try {
            Group group = groupService.createGroup(request.getName(), request.getDescription(), userId);
            return ApiResponse.success(group);
        } catch (Exception e) {
            log.error("创建分组失败", e);
            return ApiResponse.error(500, "创建分组失败: " + e.getMessage());
        }
    }

    /**
     * 获取分组详情
     */
    @GetMapping("/{id}")
    @Operation(summary = "获取分组详情", description = "根据分组 ID 获取分组的详细信息")
    public ApiResponse<Group> getGroupDetail(@PathVariable Long id, @Parameter(hidden = true) @RequestAttribute("userId") Long userId) {
        try {
            Group group = groupService.getGroupDetail(id, userId);
            return ApiResponse.success(group);
        } catch (IllegalArgumentException e) {
            return ApiResponse.error(403, e.getMessage());
        } catch (Exception e) {
            log.error("获取分组详情失败", e);
            return ApiResponse.error(500, "获取分组详情失败");
        }
    }

    /**
     * 更新分组
     */
    @PutMapping("/{id}")
    @Operation(summary = "更新分组", description = "更新分组名称和描述（仅管理员可操作）")
    public ApiResponse<Group> updateGroup(@PathVariable Long id, @Parameter(hidden = true) @RequestAttribute("userId") Long userId, @Valid @RequestBody UpdateGroupRequest request) {
        try {
            Group group = groupService.updateGroup(id, userId, request.getName(), request.getDescription());
            return ApiResponse.success(group);
        } catch (IllegalArgumentException e) {
            return ApiResponse.error(403, e.getMessage());
        } catch (Exception e) {
            log.error("更新分组失败", e);
            return ApiResponse.error(500, "更新分组失败");
        }
    }

    /**
     * 删除分组
     */
    @DeleteMapping("/{id}")
    @Operation(summary = "删除分组", description = "删除分组及其所有成员关系（仅管理员可操作）")
    public ApiResponse<Void> deleteGroup(@PathVariable Long id, @Parameter(hidden = true) @RequestAttribute("userId") Long userId) {
        try {
            groupService.deleteGroup(id, userId);
            return ApiResponse.success(null);
        } catch (IllegalArgumentException e) {
            return ApiResponse.error(403, e.getMessage());
        } catch (Exception e) {
            log.error("删除分组失败", e);
            return ApiResponse.error(500, "删除分组失败");
        }
    }

    /**
     * 获取分组成员
     */
    @GetMapping("/{id}/members")
    @Operation(summary = "获取分组成员", description = "返回指定分组的所有成员列表")
    public ApiResponse<List<GroupMember>> getGroupMembers(@PathVariable Long id, @Parameter(hidden = true) @RequestAttribute("userId") Long userId) {
        try {
            List<GroupMember> members = groupService.getGroupMembers(id, userId);
            return ApiResponse.success(members);
        } catch (IllegalArgumentException e) {
            return ApiResponse.error(403, e.getMessage());
        } catch (Exception e) {
            log.error("获取分组成员失败", e);
            return ApiResponse.error(500, "获取分组成员失败");
        }
    }

    /**
     * 加入分组
     */
    @PostMapping("/{id}/join")
    @Operation(summary = "加入分组", description = "使用邀请码加入指定分组")
    public ApiResponse<Void> joinGroup(@PathVariable Long id, @Parameter(hidden = true) @RequestAttribute("userId") Long userId, @Valid @RequestBody JoinGroupRequest request) {
        try {
            groupService.joinGroup(request.getInviteCode(), userId);
            return ApiResponse.success(null);
        } catch (IllegalArgumentException e) {
            return ApiResponse.error(400, e.getMessage());
        } catch (Exception e) {
            log.error("加入分组失败", e);
            return ApiResponse.error(500, "加入分组失败");
        }
    }

    /**
     * 离开分组
     */
    @DeleteMapping("/{id}/leave")
    @Operation(summary = "离开分组", description = "退出指定分组")
    public ApiResponse<Void> leaveGroup(@PathVariable Long id, @Parameter(hidden = true) @RequestAttribute("userId") Long userId) {
        try {
            groupService.leaveGroup(id, userId);
            return ApiResponse.success(null);
        } catch (Exception e) {
            log.error("离开分组失败", e);
            return ApiResponse.error(500, "离开分组失败");
        }
    }

    @Data
    static class CreateGroupRequest {
        @jakarta.validation.constraints.NotBlank(message = "分组名称不能为空")
        private String name;
        private String description;
    }

    @Data
    static class UpdateGroupRequest {
        private String name;
        private String description;
    }

    @Data
    static class JoinGroupRequest {
        private String inviteCode;
    }
}
