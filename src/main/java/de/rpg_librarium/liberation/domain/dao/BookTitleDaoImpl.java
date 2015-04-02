package de.rpg_librarium.liberation.domain.dao;

import org.springframework.stereotype.Component;

import de.rpg_librarium.liberation.domain.dao.generic.GenericHibernateDaoImpl;
import de.rpg_librarium.liberation.domain.entity.BookTitle;

@Component(value="bookTitleDao")
public class BookTitleDaoImpl extends GenericHibernateDaoImpl<BookTitle, Long> implements
		BookTitleDao {

}
