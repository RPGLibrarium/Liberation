use crate::api::*;
use crate::auth::{Claims};
use crate::database::*;
use crate::error::Error;
use std::collections::HashMap;

/// Get all RPG systems from database
pub fn get_rpgsystems(db: &Database) -> Result<GetRpgSystems, Error> {
    match db.get_all::<RpgSystem>() {
        Ok(rpgsystems) => Ok(GetRpgSystems { rpgsystems }),
        Err(e) => Err(e),
    }
}

/// Get an RPG system with given id from database
/// Fills the stock and availability infos when user is logged in.
pub fn get_rpgsystem(
    db: &Database,
    claims: Option<Claims>,
    system_id: RpgSystemId,
) -> Result<GetRpgSystem, Error> {
    let titles = db.get_titles_by_rpg_system(system_id)?;

    let include_stock = match claims {
        Some(_) => true,
        _ => false,
    };

    match db.get::<RpgSystem>(system_id) {
        Ok(Some(rpgsystem)) => Ok(GetRpgSystem::new(rpgsystem, titles, include_stock)),
        _ => Err(Error::ItemNotFound),
    }
}

/// Insert a RPG system into database
pub fn post_rpgsystem(
    db: &Database,
    _claims: Option<Claims>,
    system: PutPostRpgSystem,
) -> Result<RpgSystemId, Error> {
    //TODO: Error handling
    //TODO: Assert Id is unset
    Ok(db.insert::<RpgSystem>(&system.rpgsystem)?)
}

/// Update a specific system in database
pub fn put_rpgsystem(
    db: &Database,
    _claims: Option<Claims>,
    system: &PutPostRpgSystem,
) -> Result<(), Error> {
    //TODO: Error handling
    Ok(db.update::<RpgSystem>(&system.rpgsystem)?)
}

/// Delete the RPG system with given id from database
pub fn delete_rpgsystem(
    db: &Database,
    _claims: Option<Claims>,
    systemid: RpgSystemId,
) -> Result<(), Error> {
    match db.delete::<RpgSystem>(systemid) {
        Ok(true) => Ok(()),
        Ok(false) => Err(Error::ItemNotFound),
        Err(e) => Err(e),
    }
}

/// Get all titles from database
pub fn get_titles(db: &Database) -> Result<GetTitles, Error> {
    //TODO: authentication

    //TODO Error mapping
    let tuples = db.get_titles_with_details()?;

    Ok(GetTitles {
        titles: tuples
            .into_iter()
            .map(|(title, system, stock, available)| {
                TitleWithSystem::new(title, system, stock, available)
            })
            .collect(),
    })
}

/// Get a title with given id from database
pub fn get_title(
    db: &Database,
    claims: Option<Claims>,
    title_id: TitleId,
) -> Result<GetTitle, Error> {
    let (title, system, stock, available) = db.get_title_with_details(title_id)?.unwrap();
    let books = get_books_by_title_id(db, title_id, claims)?;
    //TODO: Error handling
    //Map Errors to API Errors
    //404 Not found

    Ok(GetTitle::new(title, system, stock, available, books))
}

/// Insert a title into database
pub fn post_title(
    db: &Database,
    _claims: Option<Claims>,
    title: PutPostTitle,
) -> Result<TitleId, Error> {
    //TODO: Error handling
    Ok(db.insert::<Title>(&title.title)?)
}

/// Update a specific title in database
pub fn put_title(db: &Database, _claims: Option<Claims>, title: PutPostTitle) -> Result<(), Error> {
    //TODO: Error handling
    Ok(db.update::<Title>(&title.title)?)
}

/// Delete the title with given id from database
pub fn delete_title(db: &Database, _claims: Option<Claims>, id: TitleId) -> Result<(), Error> {
    //TODO: Errorhandling
    db.delete::<Title>(id)?;
    Ok(())
}

/// Get all books of a title including rental information
fn get_books_by_title_id(
    _db: &Database,
    _id: TitleId,
    _claims: Option<Claims>,
) -> Result<Vec<BookWithOwnerWithRental>, Error> {
    //TODO: Stub
    return Ok(vec![]);
}

/// Get all books from database
pub fn get_books(db: &Database, _claims: Option<Claims>) -> Result<GetBooks, Error> {
    //TODO: authentication

    //TODO Error mapping
    let books = db.get_books_with_details()?;
    let systems_vec = RpgSystem::get_all(db)?;
    let titles_vec = Title::get_all(db)?;
    let guilds_vec = Guild::get_all(db)?;

    let mut systems_map: HashMap<RpgSystemId, RpgSystem> = HashMap::new();
    for system in systems_vec {
        match system.id {
            Some(id) => {
                systems_map.insert(id, system);
                ()
            }
            None => (),
        }
    }
    let mut titles_map: HashMap<TitleId, TitleWithSystem> = HashMap::new();
    for title in titles_vec {
        match title.id {
            Some(id) => {
                match systems_map.get(&title.system).cloned() {
                    Some(system) => {
                        titles_map.insert(id, TitleWithSystem::new(title, system, 0, 0));
                        ()
                    }
                    None => (),
                }
                ()
            }
            None => (),
        }
    }
    let mut guilds_map: HashMap<GuildId, Guild> = HashMap::new();
    for guild in guilds_vec {
        match guild.id {
            Some(id) => {
                guilds_map.insert(id, guild);
                ()
            }
            None => (),
        }
    }
    let titles_map = titles_map;
    let guilds_map = guilds_map;

    Ok(GetBooks {
        books: books
            .into_iter()
            .map(move |(book, rental, available)| {
                BookWithTitleWithOwnerWithRental {
                    id: book.id.expect("book id shall not be empty"),
                    quality: book.quality,
                    available,
                    external_inventory_id: book.external_inventory_id,
                    rental: match rental {
                        None => None,
                        Some(r) => Some(RentalWithRentee {
                            from: r.from,
                            to: r.to,
                            rentee: Entity {
                                entity_type: r.rentee_type.clone(),
                                id: r.rentee,
                                name: match r.rentee_type {
                                    EntityType::Guild => guilds_map
                                        .get(&r.rentee)
                                        .expect("invalid guild id")
                                        .name
                                        .clone(),
                                    EntityType::Member => String::from("NO DATA"), // TODO use keycloak
                                },
                            },
                        }),
                    },
                    title: titles_map
                        .get(&book.title)
                        .cloned()
                        .expect("invalid book title"),
                    owner: Entity {
                        entity_type: book.owner_type.clone(),
                        id: book.owner,
                        name: match book.owner_type {
                            EntityType::Guild => guilds_map
                                .get(&book.owner)
                                .expect("invalid guild id")
                                .name
                                .clone(),
                            EntityType::Member => String::from("NO DATA"), // TODO use keycloak
                        },
                    },
                }
            })
            .collect(),
    })
}

pub fn get_book(_db: &Database, _claims: Option<Claims>, _id: BookId) -> Result<(), Error> {
    //TODO:: Stub
    //TODO: authentication
    Ok(())
}

pub fn post_book(
    _db: &Database,
    _claims: Option<Claims>,
    _book: PutPostBook,
) -> Result<BookId, Error> {
    //TODO:: Stub
    //TODO: authentication
    Ok(1234)
}

pub fn put_book(_db: &Database, _claims: Option<Claims>, _book: PutPostBook) -> Result<(), Error> {
    //TODO:: Stub
    //TODO: authentication
    Ok(())
}

pub fn delete_book(db: &Database, _claims: Option<Claims>, id: BookId) -> Result<(), Error> {
    //TODO:: Stub
    //TODO: Errorhandling
    db.delete::<Book>(id)?;
    Ok(())
}

pub fn get_members(_db: &Database, _claims: Option<Claims>) -> Result<GetMembers, Error> {
    //TODO: Stub
    //TODO: Get Members from Database
    //TODO: Complete Infos from Keycloak
    Ok(GetMembers { members: vec![] })
}

pub fn get_member(_db: &Database, _claims: Option<Claims>, _id: MemberId) -> Result<(), Error> {
    //TODO: Stub
    //TODO: Get Members from Database
    //TODO: Complete Infos from Keycloak
    Ok(())
}

pub fn get_guilds(_db: &Database, _claims: Option<Claims>) -> Result<GetGuilds, Error> {
    //TODO: Stub
    Ok(GetGuilds { guilds: vec![] })
}

pub fn get_guild(_db: &Database, _claims: Option<Claims>, _id: GuildId) -> Result<(), Error> {
    //TODO: Stub
    Ok(())
}

pub fn post_guild(
    _db: &Database,
    _claims: Option<Claims>,
    _guild: PutPostGuild,
) -> Result<GuildId, Error> {
    //TODO: Stub
    Ok(1234)
}

pub fn put_guild(_db: &Database, _claims: Option<Claims>, _guild: PutPostGuild) -> Result<(), Error> {
    //TODO: Stub
    Ok(())
}

pub fn delete_guild(_db: &Database, _claims: Option<Claims>, _guild: GuildId) -> Result<(), Error> {
    //TODO: Stub
    Ok(())
}
