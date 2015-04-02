package de.rpg_librarium.liberation.domain.entity;

import java.io.Serializable;

import javax.persistence.Entity;
import javax.persistence.GeneratedValue;
import javax.persistence.GenerationType;
import javax.persistence.Id;
import javax.persistence.Inheritance;
import javax.persistence.InheritanceType;

/**
 * An item in the libraries inventory
 */
@Entity
@Inheritance(strategy=InheritanceType.JOINED)
public abstract class ItemType implements Serializable{
	/**
	 * 
	 */
	private static final long serialVersionUID = -2069069779097627854L;
	
	@Id
	@GeneratedValue(strategy = GenerationType.AUTO)
	private Long id;
	
	private String product_number;

	public Long getId() {
		return id;
	}

	public void setId(Long id) {
		this.id = id;
	}

	public String getProduct_number() {
		return product_number;
	}

	public void setProduct_number(String product_number) {
		this.product_number = product_number;
	}
	
	
}
