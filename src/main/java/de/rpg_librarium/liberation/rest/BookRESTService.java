package de.rpg_librarium.liberation.rest;

import javax.ws.rs.Consumes;
import javax.ws.rs.GET;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.PathParam;
import javax.ws.rs.Produces;
import javax.ws.rs.core.Context;
import javax.ws.rs.core.MediaType;
import javax.ws.rs.core.Request;

import org.springframework.stereotype.Component;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestBody;

import de.rpg_librarium.liberation.transfer.BookDTO;

@Component
@Path("/book")
public class BookRESTService {
	@Path("/{id}")
	@GET
	@Produces(MediaType.APPLICATION_JSON)
	public BookDTO show(@PathParam(value="id") int id) {
		/*
		 * TODO Fetch book by id from Database
		 */
		return new BookDTO(id, "Wege der Helden", "1234", "Peter Lustig");
	}
	
	@Path("/")
	@POST
	@Produces(MediaType.APPLICATION_JSON)
	@Consumes(MediaType.APPLICATION_JSON)
	public BookDTO create(BookDTO book) {
		/*
		 * TODO Save new book to Database
		 */
		return book;
	}
	
	/*
	 * TODO Update and delete method
	 */
}
