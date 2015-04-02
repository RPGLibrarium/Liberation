package de.rpg_librarium.liberation.domain.dao;

import org.springframework.stereotype.Component;

import de.rpg_librarium.liberation.domain.dao.generic.GenericHibernateDaoImpl;
import de.rpg_librarium.liberation.domain.entity.RuleSystem;

@Component(value="ruleSystemDao")
public class RuleSystemDaoImpl extends GenericHibernateDaoImpl<RuleSystem, Long> implements
		RuleSystemDao {

}
