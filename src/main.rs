use actix_web::{web, App, HttpServer};
extern crate actix_web;
mod resources;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(web::scope("/v1/")
            .configure(resources::recipes::config)
            .configure(resources::ingredients::config)
            .configure(resources::tags::config)
            .configure(resources::categories::config)
//            .configure(resources::search::config)
        )
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
/*
/recipes
GET
POST

/recipes/{id}
GET
PUT
PATCH
DELETE

/ingredients
GET
POST

/ingredients/{id}
GET
PUT
PATCH
DELETE

/tags
GET
POST

/tags/{id}
GET
PUT
PATCH
DELETE

/categories
GET
POST

/categories/{id}
GET
PUT
PATCH
DELETE

/search
GET
*/
