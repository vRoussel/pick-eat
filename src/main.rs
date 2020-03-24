extern crate actix_web;
extern crate tokio_postgres;

use actix_web::{web, App, HttpServer};
use tokio_postgres::{NoTls, Error};
use tokio;
mod resources;


async fn start_web_server(db_conn: tokio_postgres::Client) -> std::io::Result<()> {
    let mydata = web::Data::new(db_conn);
    HttpServer::new(move || App::new()
        .app_data(mydata.clone())
        .service(web::scope("/v1/")
            .configure(resources::recipes::config)
            .configure(resources::ingredients::config)
            .configure(resources::tags::config)
            .configure(resources::categories::config)
//            .configure(resources::search::config)
        )
    )
    .bind("127.0.0.1:8080")?.run().await
}

async fn get_db_conn() -> Result<tokio_postgres::Client, tokio_postgres::Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=valentin dbname=pick_eat", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let db_conn = get_db_conn().await?;
    start_web_server(db_conn).await?;
    Ok(())
}
