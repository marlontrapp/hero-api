use super::schema::heroes;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use diesel;
use diesel::PgConnection;

#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "heroes"]
pub struct NewHero {
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "heroes"]
pub struct Hero {
    pub id: i32,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}

impl Hero {
    pub fn fetch(id: i32, connection: &PgConnection) -> Hero {
        heroes::table.find(id).first::<Hero>(connection).unwrap()
    }

    pub fn create(hero: NewHero, connection: &PgConnection) -> NewHero {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .execute(connection)
            .expect("Error creating new hero");
        hero
    }

    pub fn read(connection: &PgConnection) -> Vec<Hero> {
        heroes::table
            .order(heroes::id.asc())
            .load::<Hero>(connection)
            .unwrap()
    }

    pub fn update(id: i32, hero: NewHero, connection: &PgConnection) -> bool {
        diesel::update(heroes::table.find(id))
            .set(&hero)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(heroes::table.find(id))
            .execute(connection)
            .is_ok()
    }
}
