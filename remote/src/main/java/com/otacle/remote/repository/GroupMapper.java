package com.otacle.remote.repository;

import com.otacle.remote.model.Group;
import org.apache.ibatis.annotations.*;
import java.util.List;

@Mapper
public interface GroupMapper {
    
    @Insert("INSERT INTO `group` (name, description, invite_code, created_by) VALUES (#{name}, #{description}, #{inviteCode}, #{createdBy})")
    @Options(useGeneratedKeys = true, keyProperty = "id")
    int insert(Group group);

    @Select("SELECT * FROM `group` WHERE id = #{id}")
    Group findById(@Param("id") Long id);

    @Select("SELECT g.* FROM `group` g INNER JOIN group_member gm ON g.id = gm.group_id WHERE gm.account_id = #{accountId}")
    List<Group> findByMemberId(@Param("accountId") Long accountId);

    @Update("UPDATE `group` SET name = #{name}, description = #{description} WHERE id = #{id}")
    int update(Group group);

    @Delete("DELETE FROM `group` WHERE id = #{id}")
    int deleteById(@Param("id") Long id);

    @Select("SELECT * FROM `group` WHERE invite_code = #{inviteCode}")
    Group findByInviteCode(@Param("inviteCode") String inviteCode);
}
