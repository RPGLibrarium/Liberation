/**
 * 
 */
package de.rpg_librarium.liberation.domain.entity;

import java.io.Serializable;

import javax.persistence.Entity;
import javax.persistence.ManyToOne;

/**
 * @author librarium
 *
 */
@Entity
public class BookTitle extends ItemType implements Serializable{
	
	/**
	 * 
	 */
	private static final long serialVersionUID = 3054238170027172675L;
	
	private String titel;
	private String author;
	private String isbn;
	
	@ManyToOne
	private RuleSystem system;

	public String getTitel() {
		return titel;
	}

	public void setTitel(String titel) {
		this.titel = titel;
	}

	public String getAuthor() {
		return author;
	}

	public void setAuthor(String author) {
		this.author = author;
	}

	public String getIsbn() {
		return isbn;
	}

	public void setIsbn(String isbn) {
		this.isbn = isbn;
	}

	public RuleSystem getSystem() {
		return system;
	}

	public void setSystem(RuleSystem system) {
		this.system = system;
	}
	
	
}
