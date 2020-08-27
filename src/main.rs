#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

use rocket::{delete, get, post, put, routes};

mod db;
mod hero;
mod schema;

use rocket_contrib::json::Json;

use db::establish_connection;

use diesel::pg::PgConnection;
use hero::{Hero, NewHero};

use std::cell::RefCell;

thread_local! {
    static CONNECTION: RefCell<PgConnection> = RefCell::new(establish_connection())
}

#[get("/<id>")]
fn fetch(id: i32) -> Json<Hero> {
    Json(CONNECTION.with(|connection| {
        return Hero::fetch(id, &*connection.borrow());
    }))
}

#[post("/", data = "<hero>")]
fn create(hero: Json<NewHero>) -> Json<NewHero> {
    Json(CONNECTION.with(|connection| {
        return Hero::create(hero.0, &*connection.borrow());
    }))
}

#[get("/")]
fn list() -> Json<Vec<Hero>> {
    return Json(CONNECTION.with(|connection| {
        return Hero::read(&*connection.borrow());
    }));
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<NewHero>) {
    CONNECTION.with(|connection| {
        return Hero::update(id, hero.0, &*connection.borrow());
    });
}

#[delete("/<id>")]
fn delete(id: i32) {
    CONNECTION.with(|connection| {
        return Hero::delete(id, &*connection.borrow());
    });
}

fn main() {
    rocket::ignite()
        .mount("/hero/", routes![fetch, create, update, delete, list])
        .launch();
}
