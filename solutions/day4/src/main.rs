use std::fmt::format;
use axum::{
    extract,
    routing::post,
    routing::get,
    Router,
    http::StatusCode,
    Json
};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
struct ReindeerStrength {
    name: String,
    strength: i64,
}

#[derive(Deserialize)]
struct ReindeerContestant {
    name: String,
    strength: i64,
    speed: f64,
    height: i64,
    antler_width: i64,
    snow_magic_power: i64,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i64,
}

#[derive(Deserialize)]
struct Contest {
    name: String,
    winning_value: i64,
    winning_text: String,
}

struct ContestFloat {
    name: String,
    winning_value: f64,
    winning_text: String,
}

#[derive(Serialize)]
struct ContestResults {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/4/strength", post(calculate_strength))
        .route("/4/contest", post(calculate_contest));

    async fn calculate_strength(extract::Json(payload): extract::Json<Vec<ReindeerStrength>>) -> Result<String, StatusCode> {
        let mut strength_sum = 0;
        for data in payload.iter() {
            strength_sum += data.strength;
        }
        Ok(strength_sum.to_string())
    }

    async fn calculate_contest(extract::Json(payload): extract::Json<Vec<ReindeerContestant>>) -> Json<ContestResults> {
        let mut fastest_contest = ContestFloat {
            name: String::from(""),
            winning_value: 0.0,
            winning_text: String::from(""),
        };

        let mut tallest_contest = Contest {
            name: String::from(""),
            winning_value: 0,
            winning_text: String::from(""),
        };

        let mut magician_contest = Contest {
            name: String::from(""),
            winning_value: 0,
            winning_text: String::from(""),
        };

        let mut consumer_contest = Contest {
            name: String::from(""),
            winning_value: 0,
            winning_text: String::from(""),
        };

        for reindeer in payload.iter() {
            if reindeer.speed > fastest_contest.winning_value {
                fastest_contest.winning_value = reindeer.speed;
                fastest_contest.winning_text = format!(
                    "Speeding past the finish line with a strength of {0} is {1}", reindeer.strength, reindeer.name
                )
            }
            if reindeer.height > tallest_contest.winning_value {
                tallest_contest.winning_value = reindeer.height;
                tallest_contest.winning_text = format!(
                    "{0} is standing tall with his {1} cm wide antlers", reindeer.name, reindeer.antler_width
                )
            }
            if reindeer.snow_magic_power > magician_contest.winning_value {
                magician_contest.winning_value = reindeer.snow_magic_power;
                magician_contest.winning_text = format!(
                    "{0} could blast you away with a snow magic power of {1}", reindeer.name, reindeer.snow_magic_power
                )
            }
            if reindeer.candies_eaten_yesterday > consumer_contest.winning_value {
                consumer_contest.winning_value = reindeer.candies_eaten_yesterday;
                consumer_contest.winning_text = format!(
                    "{0} ate lots of candies, but also some grass", reindeer.name
                )
            }
        }

        let contest_results = ContestResults {
            fastest: fastest_contest.winning_text,
            tallest: tallest_contest.winning_text,
            magician: magician_contest.winning_text,
            consumer: consumer_contest.winning_text,
        };

        Json(contest_results)
    }

    Ok(router.into())
}
