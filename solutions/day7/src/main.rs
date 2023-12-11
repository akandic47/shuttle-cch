use std::collections::HashMap;
use std::fmt::format;
use std::iter::Map;
use axum::{
    extract,
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

#[derive(Serialize, Deserialize)]
struct Recipe {
    flour: i64,
    sugar: i64,
    butter: i64,
    #[serde(rename = "baking powder")]
    baking_powder: i64,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: i64,
}

#[derive(Serialize, Deserialize)]
struct Pantry {
    flour: i64,
    sugar: i64,
    butter: i64,
    #[serde(rename = "baking powder")]
    baking_powder: i64,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: i64,
}

#[derive(Serialize, Deserialize)]
struct CookieData {
    recipe: Recipe,
    pantry: Pantry,
}

#[derive(Serialize, Deserialize)]
struct CookieUnknownData {
    recipe: HashMap<String, Value>,
    pantry: HashMap<String, Value>,
}

#[derive(Serialize)]
struct CookiesResponse {
    cookies: i64,
    pantry: Pantry,
}

#[derive(Serialize)]
struct CookiesUnknownResponse {
    cookies: i64,
    pantry: HashMap<String, Value>,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/7/decode", get(decode_recipe))
        .route("/7/bake", get(bake_recipe))
        .route("/7/bake_unknown", get(bake_unknown_recipe));

    async fn decode_recipe(headers: HeaderMap) -> String {
        let cookie = headers.get("cookie").unwrap().to_str().unwrap().to_string();
        let cookie_split: Vec<&str> = cookie.split("recipe=").collect();
        let bytes = general_purpose::STANDARD.decode(cookie_split[1]).unwrap();

        println!("{:?}", bytes);
        let response = String::from_utf8(bytes).unwrap();
        response
    }

    async fn bake_recipe(headers: HeaderMap) -> String {
        let cookie = headers.get("cookie").unwrap().to_str().unwrap().to_string();
        let cookie_split: Vec<&str> = cookie.split("recipe=").collect();
        let bytes = general_purpose::STANDARD.decode(cookie_split[1]).unwrap();
        let response = String::from_utf8(bytes).unwrap();

        let cookie_data: CookieData = serde_json::from_str(&response).unwrap();

        let available_cookies = cookie_data.pantry.flour / cookie_data.recipe.flour;

        serde_json::to_string(
            &CookiesResponse{
                cookies: available_cookies,
                pantry: Pantry {
                    flour: cookie_data.pantry.flour - available_cookies * cookie_data.recipe.flour,
                    sugar: cookie_data.pantry.sugar - available_cookies * cookie_data.recipe.sugar,
                    butter: cookie_data.pantry.butter - available_cookies * cookie_data.recipe.butter,
                    baking_powder: cookie_data.pantry.baking_powder - available_cookies * cookie_data.recipe.baking_powder,
                    chocolate_chips: cookie_data.pantry.chocolate_chips - available_cookies * cookie_data.recipe.chocolate_chips,
                }
            }
        ).unwrap()
    }

    async fn bake_unknown_recipe(headers: HeaderMap) -> String {
        let cookie = headers.get("cookie").unwrap().to_str().unwrap().to_string();
        let cookie_split: Vec<&str> = cookie.split("recipe=").collect();
        let bytes = general_purpose::STANDARD.decode(cookie_split[1]).unwrap();
        let response = String::from_utf8(bytes).unwrap();

        let cookie_data: CookieUnknownData = serde_json::from_str(&response).unwrap();

        let mut cookies_n: i64 = 0;
        for (key, val) in &cookie_data.recipe {
           if cookie_data.pantry.contains_key(key) {
               let pantries_value = cookie_data.pantry.get(key).unwrap();
               let recipe_value = cookie_data.recipe.get(key).unwrap();

               if recipe_value.as_i64() > pantries_value.as_i64() {
                   cookies_n = 0;
                   break;
               } else {
                   let possible_cookies: i64 = pantries_value.as_i64().unwrap() / recipe_value.as_i64().unwrap();
                   if possible_cookies < cookies_n {
                       cookies_n = possible_cookies;
                   }
               }
           } else {
               cookies_n = 0;
               break;
           }
        }

        let mut pantry_response: HashMap<String, Value> = HashMap::new();
        for (key, val) in &cookie_data.pantry {
            let mut pantry_value: i64 = val.as_i64().unwrap();

            if cookie_data.recipe.contains_key(key) && cookies_n > 0 {
                pantry_value = val.as_i64().unwrap() - cookies_n * cookie_data.recipe.get(key).unwrap().as_i64().unwrap();
            }
            pantry_response.insert(key.to_string(), serde_json::Value::from(pantry_value));
        }

        serde_json::to_string(
            &CookiesUnknownResponse{
                cookies: cookies_n,
                pantry: pantry_response
            }
        ).unwrap()

    }


    Ok(router.into())
}
