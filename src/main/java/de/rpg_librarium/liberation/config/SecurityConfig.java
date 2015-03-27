package de.rpg_librarium.liberation.config;

import javax.sql.DataSource;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.annotation.authentication.builders.AuthenticationManagerBuilder;
import org.springframework.security.config.annotation.method.configuration.EnableGlobalMethodSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.config.annotation.web.configuration.WebSecurityConfigurerAdapter;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;

import de.rpg_librarium.liberation.logic.security.DefaultPasswordEncoder;
import de.rpg_librarium.liberation.logic.security.JDBCUserDetailsService;

@Configuration
@EnableWebSecurity
@EnableGlobalMethodSecurity(prePostEnabled = true)
public class SecurityConfig extends WebSecurityConfigurerAdapter{
	@Autowired
	private DataSource dataSource;
	
	@Autowired 
	JDBCUserDetailsService userDetailsService;
	
	@Autowired
	DefaultPasswordEncoder passwordEncoder;

	@Autowired
	public void configureGlobal(AuthenticationManagerBuilder auth)
			throws Exception {
		auth.inMemoryAuthentication()
		.withUser("thibaud").password("1234").roles("USER");
		//auth.userDetailsService(userDetailsService).passwordEncoder(passwordEncoder);
		//TODO Change to own Database Schema by writing a new UserDetailsService.
	}
}