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
	let router = Router::new().route("/8/weight/:pokedex_number", get(pokemon_weight));

	async fn pokemon_weight(Path(pokedex_number): Path<i64>) -> String {
		println!("{:?}", pokedex_number);

		let pokeapi: String = format!(
			"https://pokeapi.co/api/v2/pokemon/{}", // Format string doesn't need any indices, so the 0 is unecessary
			pokedex_number
		);

		let resp = reqwest::get(&pokeapi).await.expect("req couldnt finish"); // Store the response as an object here

		// pokedex_number.to_string() This just outputs the dex number to string, which does not do anything productive for us.
		// Put your new processing logic here in order to get the data out of the reqwest::Response object

		"".to_string()
	}

	Ok(router.into())
}
