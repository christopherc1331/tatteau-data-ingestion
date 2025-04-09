use data_fetcher::fetch_data;
use data_parser::{parse_data, ParsedLocationData};
use data_persister::upsert_locations;
use dotenv::dotenv;
use geographical_location::USCounties;
use rusqlite::Connection;
use serde_json::Value;

pub mod data_fetcher;
pub mod data_parser;
pub mod data_persister;
pub mod geographical_location;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let location = USCounties::IDKootenaiCounty;
    let limit_results_to: i8 = 20;
    let mut current_token: Option<String> = None;
    let mut max_iter: i8 = 5;
    let conn: Connection = Connection::open("tatteau.db").expect("Database should load");

    while max_iter > 0 {
        max_iter -= 1;

        let res: Value = fetch_data(&location, limit_results_to, &current_token).await;
        println!("BEFORE: {:#?}", res);
        let parsed_data_opt: Option<ParsedLocationData> = parse_data(&res);
        if let Some(parsed_data) = parsed_data_opt {
            let ParsedLocationData {
                next_token,
                location_info,
            } = parsed_data;
            println!("AFTER: {:#?}", location_info);
            println!(
                "Found {} results out of {}",
                location_info.len(),
                limit_results_to
            );

            current_token = next_token.map(|s| s.to_string());
            upsert_locations(&conn, &location_info);
        }

        if current_token.is_none() {
            break;
        }
    }
    Ok(())
}
