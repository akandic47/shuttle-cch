use std::collections::HashMap;
use std::fmt::format;
use std::iter::Map;
use axum::{
    extract::Path,
    routing::get,
    http::header::HeaderMap,
    Router,
    http::StatusCode,
    Json
};
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};

use serde::Deserialize;
use serde::Serialize;
use serde_json::{from_str};
use serde_json::value::Value;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/8/weight/:pokedex_number", get(pokemon_weight));

    async fn pokemon_weight(Path(pokedex_number): Path<i64>) -> String {
        println!("{:?}", pokedex_number);

        let pokeapi: String = format!(
            "https://pokeapi.co/api/v2/pokemon/{0}", pokedex_number
        );

        reqwest::get(&pokeapi).await.expect("req couldnt finish");
        pokedex_number.to_string()
    }

    Ok(router.into())
}
