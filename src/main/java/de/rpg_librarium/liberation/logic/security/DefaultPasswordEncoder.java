package de.rpg_librarium.liberation.logic.security;

import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
import org.springframework.stereotype.Component;

/*
 * Default Password-Ecoder BCrypt
 *	https://de.wikipedia.org/wiki/Bcrypt
 *
 *	Empty for injecting Superclass
 */
@Component
public class DefaultPasswordEncoder extends BCryptPasswordEncoder {

}
