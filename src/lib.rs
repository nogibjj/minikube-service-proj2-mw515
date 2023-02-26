use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct RecommendationResponse {
    tracks: Vec<Track>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    name: String,
    artists: Vec<Artist>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Artist {
    name: String,
}

#[derive(Deserialize)]
pub struct GenresResponse {
    genres: Vec<String>,
}

pub async fn get_access_token(
    client_id: &str,
    client_secret: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().danger_accept_invalid_certs(true).build()?; //reqwest::Client::new();
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

pub async fn get_recommendations(
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


// write a method to request recommendations based on a genre
// return a list of recommendations formatted as a string
// e.g. "Track 1 by Artist 1, Track 2 by Artist 2, ..."
pub async fn get_recommendations_based_on_genre(access_token_data: String, genre: &str) -> Result<String, reqwest::Error> {
    let access_token = &access_token_data;
    // print out the recommendations for the genre "dance"
    // TODO: make this a parameter
    let recommendations = get_recommendations(access_token, genre).await?;
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


// write an async method to request all possible genres with spotify api
// return a list of genres formatted as a string
// e.g. "genre 1, genre 2, ..."
pub async fn get_possible_genres(access_token_data: String) -> Result<String, reqwest::Error>{
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", access_token_data).parse().unwrap(),
    );

    let response = client
        .get("https://api.spotify.com/v1/recommendations/available-genre-seeds")
        .headers(headers)
        .send()
        .await?
        .json::<GenresResponse>()
        .await?;

    Ok(response.genres.join(", "))
}




