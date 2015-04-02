package de.rpg_librarium.liberation.domain.dao.generic;

import java.io.Serializable;

/*
 * Interface for Generic DAO
 * @param E is the Type of the Managed Entity.
 * @param K is the Type of the Primary Key.
 */
public interface GenericDao<E, K extends Serializable> {
	/*
	 * Stores the new instance into the database.
	 */
	E persist(E object);
	/*
	 * Finds an object with the given primary key, that was previously persisted to the database.
	 */
	E find(K id);
	/*
	 * Save the changes, made to an object, that was previously persisted to the database.
	 */
	void update(E object);
	/*
	 * Deletes an object from the database. 
	 */
	void delete(E object);
	
}
