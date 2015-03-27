package de.rpg_librarium.liberation.rest;


import javax.persistence.EntityManager;
import javax.persistence.PersistenceContext;
import javax.transaction.Transactional;
import javax.ws.rs.GET;
import javax.ws.rs.Path;

import org.springframework.security.access.annotation.Secured;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.stereotype.Component;
import org.springframework.web.bind.annotation.PathVariable;

import de.rpg_librarium.liberation.domain.entity.Item;

@Component
@Path("/")
public class SimpleREST {
	
	@PersistenceContext
	private EntityManager em;
	
	@GET
	@Path("/{name}")
	@Secured({"USER"})
	@Transactional
	public String getGreeting(@PathVariable String name) {
        Item item = new Item();
        item.setProduct("name");
        em.persist(item);
		return item.toString();
	}
}