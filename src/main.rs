// ! A Rust Actix service for music recommendation with Spotify API
// ! by providing a genre.

/* 
Routes:
A. GET /
B. GET /possible_genres
C. GET /<genre>
D. GET /<other>
*/

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use musicreco::get_recommendations_based_on_genre;
use musicreco::get_possible_genres;
use musicreco::get_access_token;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/possible_genres")]
async fn possible_genres(access_token_data: web::Data<String>) -> impl Responder {
    let genres = get_possible_genres(access_token_data.get_ref().clone()).await;
    match genres {
        Ok(genres) => {
            println!("Genres: {}", genres);
            // Return an HTTP response
            HttpResponse::Ok().body(format!("Possible genres: \n\n{}", genres))
        }
        Err(e) => {
            eprintln!("Error getting genres: {}", e);
            HttpResponse::InternalServerError().body("Error getting genres")
        }
    }
}

// write a method to request recommendations based on a genre
// return a list of recommendations formatted as a string
// e.g. "Track 1 by Artist 1, Track 2 by Artist 2, ..."
#[get("/{genre}")]
async fn recommend(access_token_data: web::Data<String>, genre: web::Path<String>) -> impl Responder {
    match get_recommendations_based_on_genre(access_token_data.get_ref().clone(), &genre).await {
        Ok(recommendations) => {
            println!("Recommendations: {}", recommendations);
            // Return an HTTP response
            HttpResponse::Ok().body(format!("Recommendations for {} genre \n\n{}", genre, recommendations))
        }
        Err(e) => {
            eprintln!("Error getting recommendations: {}", e);
            HttpResponse::InternalServerError().body("Error getting recommendations")
        }
    }}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO
    let access_token = get_access_token("CLIENT ID", "CLIENT SECRET").await.unwrap();
    let access_token_data = web::Data::new(access_token.clone());
     HttpServer::new(move || {
        App::new()
            // Register the access_token_data as a global state for the application
            .app_data(access_token_data.clone())
            .service(index)
            .service(possible_genres)
            .service(recommend)
            //.service(other)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
