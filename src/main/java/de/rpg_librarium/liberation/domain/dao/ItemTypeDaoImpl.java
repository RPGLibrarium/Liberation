package de.rpg_librarium.liberation.domain.dao;

import org.springframework.stereotype.Component;

import de.rpg_librarium.liberation.domain.dao.generic.GenericHibernateDaoImpl;
import de.rpg_librarium.liberation.domain.entity.ItemType;

@Component(value="itemTypeDao")
public class ItemTypeDaoImpl extends GenericHibernateDaoImpl<ItemType, Long> implements
		ItemTypeDao {

}
