mod data;

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use mongodb;
use mongodb::bson::doc;

#[get("/cocktail/{name}")]
async fn cocktail(
    mongodb_client: web::Data<mongodb::Client>,
    web::Path(_name): web::Path<String>,
) -> impl Responder {
    // match &mongodb_client
    //     .database("cocktails")
    //     .collection("recipes")
    //     .find_one(doc! {}, None)
    //     .await
    // {
    //     Ok(maybe_doc) => match maybe_doc {
    //         None => HttpResponse::NotFound().finish(),
    //         Some(doc) => HttpResponse::Ok().json(doc),
    //     },
    //     Err(_) => {
    //         // Log error,
    //         HttpResponse::InternalServerError().finish()
    //     }
    // }

    let query_result = &mongodb_client
        .database("cocktails")
        .collection("recipes")
        .find_one(doc! {}, None)
        .await
        .expect("Error retrieving data");
    return match query_result {
        None => HttpResponse::NotFound().finish(),
        Some(_document) => HttpResponse::Ok().finish(),
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running.");
    let mongodb_url = std::env::var("MDB_URL").expect("Set the environment variable MDB_URL");
    let mongodb_client = mongodb::Client::with_uri_str(&mongodb_url)
        .await
        .expect("Error configuring MongoDB Client.");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(mongodb_client.clone())
            .service(cocktail)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
