
// // #[macro_use]
// // extern crate serde_json;

// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use diesel::r2d2::{self, ConnectionManager};
// use dotenv::dotenv;
// use std::env;


// // mod routes;


// type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// pub struct Blog {
//     port: u16,
// }

// impl Blog {
//     pub fn new(port: u16) -> Self {
//         Blog { port }
//     }

//     pub fn establish_connection() -> PgConnection {
//         dotenv().ok();

//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         PgConnection::establish(&database_url)
//             .expect(&format!("Error connecting to {}", database_url))
//     }
// }
