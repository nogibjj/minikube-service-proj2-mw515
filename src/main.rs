// ! A Rust Actix service for music recommendation with Spotify API
// ! by providing a genre.

/* 
Routes:
A. GET /
B. GET /<genre>
*/

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
}

async fn get_access_token(
    client_id: &str,
    client_secret: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = "grant_type=client_credentials";
    let basic_auth = general_purpose::STANDARD.encode(format!("{}:{}", client_id, client_secret));

    let response = client
    .post("https://accounts.spotify.com/api/token")
    .header(
        reqwest::header::AUTHORIZATION,
        format!("Basic {}", basic_auth),
    )
    .header(
        reqwest::header::CONTENT_TYPE,
        "application/x-www-form-urlencoded",
    )
    .body(body)
    .send()
    .await?
    .json::<AccessTokenResponse>()
    .await?;

    Ok(response.access_token)
}

#[derive(Debug, Deserialize, Serialize)]
struct RecommendationResponse {
    tracks: Vec<Track>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Track {
    name: String,
    artists: Vec<Artist>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Artist {
    name: String,
}

async fn get_recommendations(
    access_token: &str,
    genre: &str,
) -> Result<Vec<Track>, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", access_token).parse().unwrap(),
    );

    let response = client
        .get("https://api.spotify.com/v1/recommendations")
        .headers(headers)
        .query(&[("seed_genres", genre)])
        .send()
        .await?
        .json::<RecommendationResponse>()
        .await?;

    Ok(response.tracks)
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// write a method to request recommendations based on a genre
// return a list of recommendations formatted as a string
// e.g. "Track 1 by Artist 1, Track 2 by Artist 2, ..."
async fn get_recommendations_based_on_genre(access_token_data: String) -> Result<String, reqwest::Error> {
    let access_token = &access_token_data;
    // print out the recommendations for the genre "dance"
    // TODO: make this a parameter
    let recommendations = get_recommendations(access_token, "dance").await?;
    // return a list of recommendations formatted as a string
    // e.g. "Track 1 by Artist 1, Track 2 by Artist 2, ..."
    let mut recommendations_string = String::new();
    for track in recommendations {
        let mut line = format!("{} by {}, ", track.name, track.artists[0].name);
        line += "\n";
        recommendations_string.push_str(&line);
        // recommendations_string.push_str(&format!("{} by {}, ", track.name, track.artists[0].name));
    }
    Ok(recommendations_string)
}

#[get("/{genre}")]
async fn recommend(access_token_data: web::Data<String>, genre: web::Path<String>) -> impl Responder {
    match get_recommendations_based_on_genre(access_token_data.get_ref().clone()).await {
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

#[get("/{other}")]
async fn other() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let access_token = get_access_token("369dbd7452cf4e5fac093191ef5e6538", "12125765ddf14ff086d8b3252969f7a6").await.unwrap();
    let access_token_data = web::Data::new(access_token.clone());
     HttpServer::new(move || {
        App::new()
            // Register the access_token_data as a global state for the application
            .app_data(access_token_data.clone())
            .service(index)
            //.service(possible_genres)
            .service(recommend)
            .service(other)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
