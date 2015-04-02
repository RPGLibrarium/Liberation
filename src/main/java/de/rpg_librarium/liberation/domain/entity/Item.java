package de.rpg_librarium.liberation.domain.entity;


import java.io.Serializable;

import javax.persistence.Entity;
import javax.persistence.GeneratedValue;
import javax.persistence.GenerationType;
import javax.persistence.Id;
import javax.persistence.Inheritance;
import javax.persistence.InheritanceType;
import javax.persistence.ManyToOne;
import javax.persistence.OneToMany;

/**
 * An item in the libraries inventory
 */
@Entity
public class Item implements Serializable{

	/**
	 * 
	 */
	private static final long serialVersionUID = -8055426468885812056L;

	@Id
	@GeneratedValue(strategy = GenerationType.AUTO)
	private Long id;
	
	@ManyToOne
	private ItemType type;
	
	private String condition_descr;
	
	@ManyToOne
	private User owner;
	@ManyToOne
	private User holder;

	public Long getId() {
		return id;
	}

	public void setId(Long id) {
		this.id = id;
	}

	public ItemType getType() {
		return type;
	}

	public void setType(ItemType type) {
		this.type = type;
	}

	public String getConditionDescr() {
		return condition_descr;
	}

	public void setConditionDescr(String condition) {
		this.condition_descr = condition;
	}

	public User getOwner() {
		return owner;
	}

	public void setOwner(User owner) {
		this.owner = owner;
	}

	public User getHolder() {
		return holder;
	}

	public void setHolder(User holder) {
		this.holder = holder;
	}
	
	
}
