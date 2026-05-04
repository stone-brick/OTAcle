package com.otacle.remote.repository;

import com.otacle.remote.model.GroupMember;
import org.apache.ibatis.annotations.*;
import java.util.List;

@Mapper
public interface GroupMemberMapper {
    
    @Insert("INSERT INTO group_member (account_id, group_id, role) VALUES (#{accountId}, #{groupId}, #{role})")
    int insert(GroupMember member);

    @Select("SELECT * FROM group_member WHERE account_id = #{accountId} AND group_id = #{groupId}")
    GroupMember findByAccountAndGroup(@Param("accountId") Long accountId, @Param("groupId") Long groupId);

    @Select("SELECT * FROM group_member WHERE group_id = #{groupId}")
    List<GroupMember> findByGroupId(@Param("groupId") Long groupId);

    @Delete("DELETE FROM group_member WHERE account_id = #{accountId} AND group_id = #{groupId}")
    int deleteByAccountAndGroup(@Param("accountId") Long accountId, @Param("groupId") Long groupId);

    @Delete("DELETE FROM group_member WHERE group_id = #{groupId}")
    int deleteByGroupId(@Param("groupId") Long groupId);

    @Select("SELECT COUNT(*) FROM group_member WHERE group_id = #{groupId} AND account_id = #{accountId}")
    int countByAccountAndGroup(@Param("accountId") Long accountId, @Param("groupId") Long groupId);
}
