use axum::{routing::get, Router, extract::Path, http::StatusCode};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/1_1/:num1/:num2", get(recalibrate_packets))
        .route("/1_2/*path", get(recalibrate_sleds))
        .route("/test", get(|| async { "Hello, World!" }));

    async fn recalibrate_packets(Path((num1, num2)): Path<(i64, i64)>) -> String {
        (num1 ^ num2).pow(3).to_string()
    }

    async fn recalibrate_sleds(Path(path): Path<String>) -> Result<String, StatusCode> {
        let path_split = path.split("/");
        let path_count = path_split.collect::<Vec<&str>>();
        if path_count.len() < 1 || path_count.len() > 20 {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let mut result = 0;
        for path in path_count {
            let path_number: i64 = path.parse().unwrap();
            result = result ^ path_number;
        }
        result = result.pow(3);
        Ok(result.to_string())
    }

    Ok(router.into())
}
