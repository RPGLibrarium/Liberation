package de.rpg_librarium.liberation.transfer;

import java.io.Serializable;

import javax.xml.bind.annotation.XmlRootElement;

@XmlRootElement
public class BookDTO implements Serializable{
	private static final long serialVersionUID = 1L;
	public int id;
	public String name;
	public String isbn;
	public String author;
	
	public BookDTO(int id, String name, String isbn, String author) {
		this.id = id;
		this.name = name;
		this.isbn = isbn;
		this.author = author;
	}
	
	public BookDTO() {
	}
}
