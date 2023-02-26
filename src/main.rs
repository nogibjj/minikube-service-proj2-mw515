// ! A Rust Actix service for music recommendation with Spotify API
// ! by providing a genre.

/* 
Routes:
A. GET /
B. GET /possible-genres
C. GET /<genre>
D. GET /<other stuff>
*/

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
}

fn get_access_token(
    client_id: &str,
    client_secret: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
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
        .send()?
        .json::<AccessTokenResponse>()?;
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

fn get_recommendations(
    access_token: &str,
    genre: &str,
) -> Result<Vec<Track>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", access_token).parse().unwrap(),
    );

    let response = client
        .get("https://api.spotify.com/v1/recommendations")
        .headers(headers)
        .query(&[("seed_genres", genre)])
        .send()?
        .json::<RecommendationResponse>()?;

    Ok(response.tracks)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = get_access_token("369dbd7452cf4e5fac093191ef5e6538", "12125765ddf14ff086d8b3252969f7a6")?;
    // println!("Access token: {}", access_token);
    
    // print out the recommendations for the genre "dance"
    let recommendations = get_recommendations(&access_token, "dance")?;
    for track in recommendations {
        println!("{} by {}", track.name, track.artists[0].name);
    }
    Ok(())
}
