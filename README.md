This tutorial was based on https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8, credits to Sean Wragg

## Rust language

"Rust is a multi-paradigm programming language focused on performance and safety"

- Rust is a statically and strongly typed
- Rust is designed to be memory safe
- Rust is comparable to the performance of idiomatic C++


## Preparing the environment

Install rust:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Should shows:

```
$ rustc --version
rustc 1.45.1 (c367798cf 2020-07-26)
```

If you have it already installed you can update it with:

```
$ rustup update
```

## Starting a new project

```
$ cargo new hero-api --bin && cd hero-api
```

After this command we'll have this on the directory:

```
$ ls
Cargo.toml  src
```

The src folder is where the source code of our project will remain.
The Cargo.toml will act similar to Node’s package.json

if we run 'cargo run' now we'll ahve the following output:

```
$ cargo run
   Compiling hero v0.1.0 (/home/marlon/git/hero)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/hero`
Hello, world!
```

## Adding a new dependency

Edit Cargo.toml as it follows:
```
[dependencies]
rocket = "0.4.5"
rocket_codegen = "0.4.5"
```

Rocket definition from its [website](https://rocket.rs): 

    Rocket is a web framework for Rust that makes it simple to write fast, secure web applications without sacrificing flexibility, usability, or type safety.

So, that is the web framework we'll be utilizing to build our API.

The rocket framework requires the nightly build of rust so we'll set for this project the nightly version of rust:

```
$ rustup override set nightly
```

Let's test if everything is working, getting the started code from rocker.rs website.
Edit the file src/main.rs as follows:

```
#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes};

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}

```

After this you can run `cargo run` again and then it'll compile and start the webserver:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/hero-api`
🔧 Configured for development.
    => address: localhost
    => port: 8000
    => log: normal
    => workers: 16
    => secret key: generated
    => limits: forms = 32KiB
    => keep-alive: 5s
    => tls: disabled
🛰  Mounting /:
    => GET /hello/<name>/<age> (hello)
🚀 Rocket has launched from http://localhost:8000
```

You can try it:

```
$ curl http://localhost:8000/hello/Marlon/28
Hello, 28 year old named Marlon!%                                   

```

## Serializing and deserializing

For this we'll need another library to do the job, so add those lines in you denpendencies section of your Cargo.toml:

```
rocket_contrib = "0.4.5"
serde = { version = "1.0", features = ["derive"] }
```

And then let's create a type, create a file named src/hero.rs:

```
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}

```

and returns it into a get endpoint, on main.rs add:

```
#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes};

mod hero;

use hero::Hero;
use rocket_contrib::json::Json;

#[get("/<id>")]
fn fetch(id: i32) -> Json<Hero> {
    let hero = Hero {
        id: Some(id),
        name: String::from("Batman"),
        identity: String::from("Bruce Wayne"),
        hometown: String::from("Gotham City"),
        age: 30,
    };
    Json(hero)
}

fn main() {
    rocket::ignite().mount("/hero/", routes![fetch]).launch();
}
```

We can test the endpoint with:

```
$ curl http://localhost:8000/hero/12
```

Ok so now, we can add the other REST methods for the type:

```
#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[get("/")]
fn list() -> Json<Vec<Hero>> {
    Json(vec![
        Hero {
            id: Some(123),
            name: String::from("Batman"),
            identity: String::from("Bruce Wayne"),
            hometown: String::from("Gotham City"),
            age: 30,
        },
        Hero {
            id: Some(456),
            name: String::from("Superman"),
            identity: String::from("Clark Kent"),
            hometown: String::from("Metropolis"),
            age: 32,
        },
    ])
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>) -> Json<Hero> {
    let mut h = hero.0;
    h.id = Some(id);
    Json(h)
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<Hero> {
    let hero = Hero {
        id: Some(id),
        name: String::from("Batman"),
        identity: String::from("Bruce Wayne"),
        hometown: String::from("Gotham City"),
        age: 30,
    };
    Json(hero)
}
```

Add the new imports:

```
use rocket::{delete, get, post, put, routes};
```
And modify the main method:
```
fn main() {
    rocket::ignite()
        .mount("/hero/", routes![fetch, create, update, delete, list])
        .launch();
}
```


We can test all endpoint now:

```
$ curl http://localhost:8000/hero/12

$ curl http://localhost:8000/hero

$ curl http://localhost:8000/hero -H "Accept: application/json" -X POST --data '{"name":"Spider-Man","identity":"Peter Park","hometown":"New York","age":15}'

$ curl http://localhost:8000/hero/12 -H "Accept: application/json" -X PUT --data '{"name":"Spider-Man","identity":"Peter Park","hometown":"New York","age":16}'


```

## Persistence

As you probably noticed those values are all hard coded. So let's add some form of persisntence on this to get closer to a real world application.

For this we'll be using `diesel`:

    Diesel is a Safe, Extensible ORM and Query Builder for Rust

Add the dependency on your Cargo.toml, additionally we'll add another dependenci to read a .env file

```
diesel = { version = "1.4.4", features = ["postgres"] }
dotenv = "0.15.0"
```

Let's configure our database as well, for this I've used a docker compose file `docker-compose.yml`:

```
version: "3"

services:
  db:
    image: "postgres:10-alpine"
    volumes:
      -  postgresql_data:/var/lib/postgresql/data
    hostname: postgres
    environment:
      - POSTGRES_DB=hero
      - POSTGRES_USER=postgres
      - POSTGRES_HOST_AUTH_METHOD=trust
    ports:
      - "5432:5432"

volumes:
  postgresql_data:

```

And the .env file:

```
DATABASE_URL=postgres://postgres@localhost/hero
```

Let's up the database:
```
$ docker-compose up
```

And Run the diesel CLI setup:

```
$ diesel setup
```

This will create our database (if it didn't already exist), and create an empty migrations directory that we can use to manage our schema (more on that later).


Let's create our first migrations:

```
$ diesel migration generate create_heroes
```

This will generate two files inside the migrations folder, one for update the version of the database and one to downgrade. Diesel doesn't have a tool to auto generate the SQL based on the 'model'. So we have to do it manually:

`migrations/2020-08-07-202843_create_heroes/up.sql`:
```
DROP TABLE IF EXISTS heroes;

CREATE TABLE heroes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    identity VARCHAR NOT NULL,
    hometown VARCHAR NOT NULL,
    age INT NOT NULL
)
```

`migrations/2020-08-07-202843_create_heroes/down.sql`:
```
DROP TABLE heroes;
```

Now we can play with our migrations:

To apply:
```
$ diesel migration run
```

If you want to test if your migration is working correctly in both direction you can use:
```
diesel migration redo 
```

To dowgrade:
```
$ diesel migration revert
```
Let's write the code to stablish the database connection:

`src/db.rs`:
```
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

```

Now we can write some database operations for our model Hero.
Modify the `hero.rs` file as follows:

```
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

```

And modify our `main.rs` to use the database instead mock objects:

```
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

```

Now we should be able to test all methods:

```
$ curl http://localhost:8000/hero -H "Accept: application/json" -X POST --data '{"name":"Spider-Man","identity":"Peter Park","hometown":"New York","age":15}'

$ curl http://localhost:8000/hero

$ curl http://localhost:8000/hero/12

$ curl http://localhost:8000/hero/12 -H "Accept: application/json" -X PUT --data '{"name":"Spider-Man","identity":"Peter Park","hometown":"New York","age":16}'

$ curl http://localhost:8000/hero/12 -X DELETE

```