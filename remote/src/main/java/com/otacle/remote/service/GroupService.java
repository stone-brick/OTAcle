package com.otacle.remote.service;

import com.otacle.remote.model.Group;
import com.otacle.remote.model.GroupMember;
import com.otacle.remote.repository.GroupMapper;
import com.otacle.remote.repository.GroupMemberMapper;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;
import java.util.UUID;

@Slf4j
@Service
public class GroupService {

    private final GroupMapper groupMapper;
    private final GroupMemberMapper groupMemberMapper;

    public GroupService(GroupMapper groupMapper, GroupMemberMapper groupMemberMapper) {
        this.groupMapper = groupMapper;
        this.groupMemberMapper = groupMemberMapper;
    }

    /**
     * 创建分组
     */
    @Transactional
    public Group createGroup(String name, String description, Long createdBy) {
        Group group = new Group();
        group.setName(name);
        group.setDescription(description);
        group.setInviteCode(generateInviteCode());
        group.setCreatedBy(createdBy);
        
        groupMapper.insert(group);
        
        // 创建者自动成为管理员
        GroupMember member = new GroupMember();
        member.setAccountId(createdBy);
        member.setGroupId(group.getId());
        member.setRole("admin");
        groupMemberMapper.insert(member);
        
        log.info("分组创建成功: groupId={}, name={}", group.getId(), name);
        
        return group;
    }

    /**
     * 获取用户的分组列表
     */
    public List<Group> getUserGroups(Long userId) {
        return groupMapper.findByMemberId(userId);
    }

    /**
     * 获取分组详情
     */
    public Group getGroupDetail(Long groupId, Long userId) {
        validateGroupMember(groupId, userId);
        return groupMapper.findById(groupId);
    }

    /**
     * 更新分组
     */
    @Transactional
    public Group updateGroup(Long groupId, Long userId, String name, String description) {
        validateGroupAdmin(groupId, userId);
        
        Group group = groupMapper.findById(groupId);
        if (group == null) {
            throw new IllegalArgumentException("分组不存在");
        }
        
        group.setName(name);
        group.setDescription(description);
        groupMapper.update(group);
        
        return group;
    }

    /**
     * 删除分组
     */
    @Transactional
    public void deleteGroup(Long groupId, Long userId) {
        validateGroupAdmin(groupId, userId);
        groupMemberMapper.deleteByGroupId(groupId);
        groupMapper.deleteById(groupId);
    }

    /**
     * 获取分组成员列表
     */
    public List<GroupMember> getGroupMembers(Long groupId, Long userId) {
        validateGroupMember(groupId, userId);
        return groupMemberMapper.findByGroupId(groupId);
    }

    /**
     * 加入分组（通过邀请码）
     */
    @Transactional
    public void joinGroup(String inviteCode, Long userId) {
        Group group = groupMapper.findByInviteCode(inviteCode);
        if (group == null) {
            throw new IllegalArgumentException("邀请码无效");
        }
        
        // 检查是否已经是成员
        GroupMember existingMember = groupMemberMapper.findByAccountAndGroup(userId, group.getId());
        if (existingMember != null) {
            throw new IllegalArgumentException("您已经是该分组的成员");
        }
        
        // 添加为普通成员
        GroupMember member = new GroupMember();
        member.setAccountId(userId);
        member.setGroupId(group.getId());
        member.setRole("member");
        groupMemberMapper.insert(member);
        
        log.info("用户加入分组: userId={}, groupId={}", userId, group.getId());
    }

    /**
     * 离开分组
     */
    @Transactional
    public void leaveGroup(Long groupId, Long userId) {
        groupMemberMapper.deleteByAccountAndGroup(userId, groupId);
    }

    /**
     * 验证分组成员
     */
    private void validateGroupMember(Long groupId, Long userId) {
        int count = groupMemberMapper.countByAccountAndGroup(userId, groupId);
        if (count == 0) {
            throw new IllegalArgumentException("您不是该分组的成员");
        }
    }

    /**
     * 验证分组管理员
     */
    private void validateGroupAdmin(Long groupId, Long userId) {
        GroupMember member = groupMemberMapper.findByAccountAndGroup(userId, groupId);
        if (member == null || !"admin".equals(member.getRole())) {
            throw new IllegalArgumentException("您没有管理员权限");
        }
    }

    /**
     * 生成邀请码
     */
    private String generateInviteCode() {
        return UUID.randomUUID().toString().replace("-", "").substring(0, 16).toUpperCase();
    }
}
