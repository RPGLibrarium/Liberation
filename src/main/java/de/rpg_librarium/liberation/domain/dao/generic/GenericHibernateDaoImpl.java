/**
 * 
 */
package de.rpg_librarium.liberation.domain.dao.generic;

import java.io.Serializable;
import java.lang.reflect.ParameterizedType;

import javax.persistence.EntityManager;
import javax.persistence.PersistenceContext;

import org.springframework.transaction.annotation.Transactional;
/**
 * @author librarium
 *
 */

public class GenericHibernateDaoImpl<E, K extends Serializable> implements GenericDao<E, K> {
	
	@PersistenceContext
	private EntityManager em;
	
	private Class<E> entityClass;
	
	public GenericHibernateDaoImpl() {
	        ParameterizedType genericSuperclass = (ParameterizedType) getClass()
	             .getGenericSuperclass();
	        this.entityClass = (Class<E>) genericSuperclass
	             .getActualTypeArguments()[0];
	    }

	@Override
	@Transactional
	public E persist(E object) {
		this.em.persist(object);
		return object;
	}

	@Override
	public E find(K id) {
		return this.em.find(entityClass, id);
	}

	@Override
	public void update(E object) {
		this.em.merge(object);
	}

	@Override
	public void delete(E object) {
		this.em.remove(object);
		
	}


	
}
