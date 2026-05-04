package com.otacle.remote.repository;

import com.otacle.remote.model.Account;
import org.apache.ibatis.annotations.*;

@Mapper
public interface AccountMapper {
    
    @Insert("INSERT INTO account (username, password, email) VALUES (#{username}, #{password}, #{email})")
    @Options(useGeneratedKeys = true, keyProperty = "id")
    int insert(Account account);

    @Select("SELECT * FROM account WHERE username = #{username}")
    Account findByUsername(@Param("username") String username);

    @Select("SELECT * FROM account WHERE email = #{email}")
    Account findByEmail(@Param("email") String email);

    @Select("SELECT * FROM account WHERE id = #{id}")
    Account findById(@Param("id") Long id);
}
