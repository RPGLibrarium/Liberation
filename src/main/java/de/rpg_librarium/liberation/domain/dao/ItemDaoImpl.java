package de.rpg_librarium.liberation.domain.dao;

import org.springframework.stereotype.Component;

import de.rpg_librarium.liberation.domain.dao.generic.GenericHibernateDaoImpl;
import de.rpg_librarium.liberation.domain.entity.Item;

@Component(value="itemDao")
public class ItemDaoImpl extends GenericHibernateDaoImpl<Item, Long> implements
		ItemDao {

}
