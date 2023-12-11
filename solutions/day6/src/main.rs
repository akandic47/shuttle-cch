use std::fmt::format;
use axum::{
    extract,
    routing::post,
    routing::get,
    Router,
    http::StatusCode,
    Json
};
use serde::Serialize;

#[derive(Serialize)]
struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_no_elf: usize,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/6", post(calculate_elf));

    async fn calculate_elf(body: String) -> Json<ElfCount>{
        let elf_split = body.split("elf");
        let elf_split_vec = elf_split.collect::<Vec<&str>>();

        let elf_on_a_shelf = body.split("elf on a shelf");
        let elf_on_a_shelf_vec = elf_on_a_shelf.collect::<Vec<&str>>();

        let shelf_split= body.split("shelf");
        let shelf_split_vec = shelf_split.collect::<Vec<&str>>();

        Json(ElfCount{
            elf: elf_split_vec.len() - 1,
            elf_on_a_shelf: elf_on_a_shelf_vec.len() - 1,
            shelf_no_elf: shelf_split_vec.len() - elf_on_a_shelf_vec.len()
        })
    }

    Ok(router.into())
}
