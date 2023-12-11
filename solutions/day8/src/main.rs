use axum::{
    extract::Path,
    routing::get,
    Router,
};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/8/weight/:pokedex_number", get(pokemon_weight));

    async fn pokemon_weight(Path(pokedex_number): Path<i64>) -> String {
        println!("{:?}", pokedex_number);

        let pokeapi: String = format!(
            "https://pokeapi.co/api/v2/pokemon/{}", pokedex_number
        );

        let resp = reqwest::get(&pokeapi).await.expect("req couldnt finish");
        "".to_string()
    }

    Ok(router.into())
}
