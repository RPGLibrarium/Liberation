package de.rpg_librarium.liberation.domain.dao;

import org.springframework.stereotype.Component;

import de.rpg_librarium.liberation.domain.dao.generic.GenericHibernateDaoImpl;
import de.rpg_librarium.liberation.domain.entity.User;

@Component(value="userDao")
public class UserDaoImpl extends GenericHibernateDaoImpl<User, Long> implements
		UserDao {

}
