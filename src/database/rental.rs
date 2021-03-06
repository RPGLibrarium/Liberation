use super::*;
use crate::serde_formats;
use serde::Serialize;
use mysql::params;

/// Id type for Rental
pub type RentalId = Id;

/// Stores all information of a rental process
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Rental {
    /// Id
    pub id: Option<RentalId>,
    /// Date of rental
    #[serde(with = "serde_formats::naive_date")]
    pub from: Date,
    /// Date of end of rental
    #[serde(with = "serde_formats::naive_date")]
    pub to: Date,
    /// Id of rented Book
    pub book: BookId,
    /// Type of rentee
    pub rentee_type: EntityType,
    /// Id of rentee
    pub rentee: EntityId,
}

impl Rental {
    /// Construct a new Rental object with given parameters
    pub fn new(
        id: Option<RentalId>,
        from: Date,
        to: Date,
        book: BookId,
        rentee: EntityId,
        rentee_type: EntityType,
    ) -> Rental {
        return Rental {
            id: id,
            from: from,
            to: to,
            book: book,
            rentee: rentee,
            rentee_type: rentee_type,
        };
    }

    /// Construct a new Rental object with given parameters with manual input of rentee type
    pub fn from_db(
        id: RentalId,
        from: Date,
        to: Date,
        book: BookId,
        rentee_member: Option<MemberId>,
        rentee_guild: Option<GuildId>,
        rentee_type: String,
    ) -> Result<Rental, String> {
        let rentee_type = match EntityType::from_str(rentee_type.as_str()) {
            Ok(x) => x,
            Err(_) => return Err(String::from("Bad rentee_type")),
        };

        let rentee: EntityId = match match rentee_type {
            EntityType::Member => rentee_member,
            EntityType::Guild => rentee_guild,
        } {
            Some(x) => x,
            None => return Err(String::from(
                "Field 'rentee_member' or 'rentee_guild' is not set according to 'rentee_type'.",
            )),
        };

        Ok(Rental::new(Some(id), from, to, book, rentee, rentee_type))
    }
}

impl DMO for Rental {
    type Id = RentalId;

    fn get(db: &Database, rental_id: RentalId) -> Result<Option<Rental>, Error> {
        let mut results = db.pool
        .prep_exec(
            "select rental_id, from_date, to_date, book_by_id, rentee_member_by_id, rentee_guild_by_id, rentee_type from rentals where rental_id=:rental_id;",
            params!{
                "rental_id" => rental_id,
            },
        )
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, from, to, book, rentee_member, rentee_guild, rentee_type) = mysql::from_row(row);
            let from: NaiveDate = from;
            let to: NaiveDate = to;
            Rental::from_db(id, from, to, book, rentee_member, rentee_guild, rentee_type).unwrap()
        }).collect::<Vec<Rental>>()
    })?;
        return Ok(results.pop());
    }

    fn get_all(db: &Database) -> Result<Vec<Rental>, Error> {
        Ok(db.pool.prep_exec("select rental_id, from_date, to_date, book_by_id, rentee_member_by_id, rentee_guild_by_id, rentee_type from rentals;",())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, from, to, book, rentee_member, rentee_guild, rentee_type) = mysql::from_row(row);
            //FIXME: @FutureMe: You should have handled the error directly!!!! You stupid prick.
            let from: NaiveDate = from;
            let to: NaiveDate = to;
            Rental::from_db(id, from, to, book, rentee_member, rentee_guild, rentee_type).unwrap()
        }).collect()
    })?)
    }

    fn insert(db: &Database, inp: &Rental) -> Result<RentalId, Error> {
        check_date!(inp.from, inp.to);
        Ok(db.pool.prep_exec("insert into rentals (from_date, to_date, book_by_id, rentee_member_by_id, rentee_guild_by_id) values (:from, :to, :book, :rentee_member, :rentee_guild)",
        params!{
            "from" => inp.from,
            "to" => inp.to,
            "book" => inp.book,
            "rentee_member" => match inp.rentee_type {
                EntityType::Member => Some(inp.rentee),
                EntityType::Guild => None,
            },
            "rentee_guild" => match inp.rentee_type {
                EntityType::Member => None,
                EntityType::Guild => Some(inp.rentee),
            },
        }).map(|result| result.last_insert_id()
        )?)
    }

    fn update(db: &Database, rental: &Rental) -> Result<(), Error> {
        check_date!(rental.from, rental.to);
        Ok(db.pool.prep_exec("update rentals set from_date=:from, to_date=:to, book_by_id=:book, rentee_member_by_id=:rentee_member, rentee_guild_by_id=:rentee_guild where rental_id=:id;",
        params!{
            //"from" => rental.from.format(SQL_DATEFORMAT).to_string(),
            //"to" => rental.to.format(SQL_DATEFORMAT).to_string(),
            "from" => rental.from,
            "to" => rental.to,
            "book" => rental.book,
            "rentee_member" => match rental.rentee_type {
                EntityType::Member => Some(rental.rentee),
                EntityType::Guild => None,
            },
            "rentee_guild" => match rental.rentee_type {
                EntityType::Member => None,
                EntityType::Guild => Some(rental.rentee),
            },
            "id" => rental.id,
        }).and(Ok(()))?)
    }

    fn delete(db: &Database, id: Id) -> Result<bool, Error> {
        Ok(db
            .pool
            .prep_exec(
                "delete from rentals where rental_id=:id",
                params! {
                    "id" => id,
                },
            )
            .map_err(|err| Error::DatabaseError(err))
            .and_then(|result| match result.affected_rows() {
                1 => Ok(true),
                0 => Ok(false),
                _ => Err(Error::IllegalState),
            })?)
    }
}
#[cfg(test)]
mod tests {
    use database::test_util::*;
    use database::*;

    #[test]
    fn insert_rental_correct() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let mut member_in = Member::new(None, _s("some-external-id"));
        let result = db
            .insert(&mut member_in)
            // .and_then(|member|
            //     db.insert_guild(_s("Yordle Academy of Science and Progress"), _s("Piltover"), member.id)
            //         .and_then(|guild| Ok((member, guild)))
            // )
            .and_then(|member_id| {
                insert_book_default(&db).and_then(|(book_id, _)| Ok((book_id, member_id)))
            })
            .and_then(|(book_id, member_id)| {
                let mut rental_in = Rental::new(
                    None,
                    _d(2018, 2, 4),
                    _d(2018, 4, 16),
                    book_id,
                    member_id,
                    EntityType::Member,
                );
                db.insert(&mut rental_in).and_then(|id| Ok((id, rental_in)))
            })
            .and_then(|(id, rental_in)| {
                db.get::<Rental>(id).and_then(|rental_out| {
                    Ok((
                        Rental {
                            id: Some(id),
                            ..rental_in
                        },
                        rental_out,
                    ))
                })
            })
            .and_then(|(rental_in, rental_out)| {
                Ok(rental_out.map_or(false, |rec_rental| rental_in == rec_rental))
            });
        teardown(settings);
        match result {
            Ok(true) => (),
            Ok(false) => panic!("Inserted rental is not in DB :("),
            _ => {
                result.unwrap();
                ()
            }
        }
    }

    #[test]
    fn insert_rental_invalid_book() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db).and_then(|(_book_id, book)| {
            db.insert(&mut Rental::new(
                None,
                _d(2014, 8, 16),
                _d(3264, 12, 08),
                1235415123,
                book.owner,
                book.owner_type,
            ))
        });
        teardown(settings);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn insert_rental_invalid_owner_id() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db).and_then(|(book_id, book)| {
            db.insert(&mut Rental::new(
                None,
                _d(2014, 8, 16),
                _d(3264, 12, 08),
                book_id,
                12342433,
                book.owner_type,
            ))
        });
        teardown(settings);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn insert_rental_wrong_owner_type() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db).and_then(|(_book_id, book)| {
            db.insert(&mut Rental::new(
                None,
                _d(2014, 8, 16),
                _d(3264, 12, 08),
                012481632,
                book.owner,
                match book.owner_type {
                    EntityType::Member => EntityType::Guild,
                    EntityType::Guild => EntityType::Member,
                },
            ))
        });
        teardown(settings);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn update_rental_correct() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db)
            .and_then(|(book_id, book)| {
                let mut orig_rental = Rental::new(
                    None,
                    _d(2012, 3, 4),
                    _d(2056, 7, 8),
                    book_id,
                    book.owner,
                    book.owner_type,
                );
                db.insert(&mut orig_rental)
                    .and_then(|rental_id| Ok((rental_id, orig_rental)))
            })
            .and_then(|(rental_id, orig_rental)| {
                db.insert(&mut Member::new(None, _s("rincewind")))
                    .and_then(|member_id| Ok((rental_id, orig_rental, member_id)))
            })
            .and_then(|(rental_id, orig_rental, member_id)| {
                db.insert(&mut Guild::new(
                    None,
                    _s("Yordle Academy of Science and Progress"),
                    _s("Piltover"),
                    member_id,
                ))
                .and_then(|guild_id| Ok((rental_id, orig_rental, guild_id)))
            })
            .and_then(|(rental_id, orig_rental, guild_id)| {
                db.insert(&mut RpgSystem::new(None, _s("Discworld"), None))
                    .and_then(|system_id| Ok((rental_id, orig_rental, guild_id, system_id)))
            })
            .and_then(|(rental_id, orig_rental, guild_id, system_id)| {
                db.insert(&mut Title::new(
                    None,
                    _s("Unseen University Adventures"),
                    system_id,
                    _s("en"),
                    _s("Twoflower Publishing"),
                    2048,
                    None,
                ))
                .and_then(|title_id| Ok((rental_id, orig_rental, guild_id, title_id)))
            })
            .and_then(|(rental_id, orig_rental, guild_id, title_id)| {
                db.insert(&mut Book::new(
                    None,
                    title_id,
                    guild_id,
                    EntityType::Guild,
                    _s("impressive"),
                    21,
                ))
                .and_then(|book_id| Ok((rental_id, orig_rental, book_id, guild_id)))
            })
            .and_then(|(rental_id, orig_rental, book_id, guild_id)| {
                let rental_update = Rental {
                    id: Some(rental_id),
                    from: _d(2090, 10, 11),
                    to: _d(2112, 1, 3),
                    book: book_id,
                    rentee: guild_id,
                    rentee_type: EntityType::Guild,
                    ..orig_rental
                };
                db.update(&rental_update)
                    .and_then(|_| Ok((rental_id, rental_update)))
            })
            .and_then(|(rental_id, rental_update)| {
                db.get(rental_id)
                    .and_then(|rec_rental| Ok((rental_update, rec_rental)))
            })
            .and_then(|(rental_update, rec_rental)| {
                Ok(rec_rental.map_or(false, |fetched_rental| rental_update == fetched_rental))
            });
        teardown(settings);
        match result {
            Ok(true) => (),
            Ok(false) => panic!("Expected updated guild to be corretly stored in DB"),
            _ => {
                result.unwrap();
                ()
            }
        }
    }

    #[test]
    fn update_rental_invalid_from() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db)
            .and_then(|(book_id, book)| {
                let mut rental = Rental::new(
                    None,
                    _d(2012, 3, 4),
                    _d(2056, 7, 8),
                    book_id,
                    book.owner,
                    book.owner_type,
                );
                db.insert(&mut rental).and_then(|_| Ok(rental))
            })
            .and_then(|mut orig_rental| {
                orig_rental.from = _d(-99, 10, 11);
                db.update(&orig_rental)
            });
        teardown(settings);
        match result {
            Err(Error::IllegalValueForType(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::IllegalValueForType)"),
        }
    }

    #[test]
    fn update_rental_invalid_to() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db)
            .and_then(|(book_id, book)| {
                let mut rental = Rental::new(
                    None,
                    _d(2012, 3, 4),
                    _d(2056, 7, 8),
                    book_id,
                    book.owner,
                    book.owner_type,
                );
                db.insert(&mut rental).and_then(|_| Ok(rental))
            })
            .and_then(|mut orig_rental| {
                orig_rental.to = _d(-99, 11, 12);
                db.update(&orig_rental)
            });
        teardown(settings);
        match result {
            Err(Error::IllegalValueForType(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::IllegalValueForType)"),
        }
    }

    #[test]
    fn update_rental_invalid_book() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db)
            .and_then(|(book_id, book)| {
                let mut rental = Rental::new(
                    None,
                    _d(2012, 3, 4),
                    _d(2056, 7, 8),
                    book_id,
                    book.owner,
                    book.owner_type,
                );
                db.insert(&mut rental).and_then(|id| {
                    Ok(Rental {
                        id: Some(id),
                        ..rental
                    })
                })
            })
            .and_then(|mut rental| {
                rental.book = 012481632;
                db.update(&rental)
            });
        teardown(settings);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::IllegalValueForType)"),
        }
    }

    #[test]
    fn update_rental_invalid_rentee_id() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db)
            .and_then(|(book_id, book)| {
                let mut rental = Rental::new(
                    None,
                    _d(2012, 3, 4),
                    _d(2056, 7, 8),
                    book_id,
                    book.owner,
                    book.owner_type,
                );
                db.insert(&mut rental).and_then(|id| {
                    Ok(Rental {
                        id: Some(id),
                        ..rental
                    })
                })
            })
            .and_then(|mut rental| {
                rental.rentee = 012481632;
                db.update(&rental)
            });
        teardown(settings);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::IllegalValueForType)"),
        }
    }

    #[test]
    fn update_rental_wrong_rentee_type() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db)
            .and_then(|(book_id, book)| {
                let mut rental = Rental::new(
                    None,
                    _d(2012, 3, 4),
                    _d(2056, 7, 8),
                    book_id,
                    book.owner,
                    book.owner_type,
                );
                db.insert(&mut rental).and_then(|id| {
                    Ok(Rental {
                        id: Some(id),
                        ..rental
                    })
                })
            })
            .and_then(|mut rental| {
                rental.rentee_type = match rental.rentee_type {
                    EntityType::Member => EntityType::Guild,
                    EntityType::Guild => EntityType::Member,
                };
                db.update(&rental)
            });
        teardown(settings);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::IllegalValueForType)"),
        }
    }
}
