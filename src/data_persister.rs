use rusqlite::{params, Connection, Error};

use crate::data_parser::LocationInfo;

pub fn upsert_locations(conn: &Connection, locations: &[LocationInfo]) -> Result<(), Error> {
    let mut stmt = conn.prepare_cached(
        "INSERT OR REPLACE INTO locations (
                        city,
                        county,
                        state,
                        country_code,
                        postal_code,
                        is_open,
                        address,
                        id,
                        category,
                        name,
                        website_uri
                    )
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
    )?;
    let t = conn.unchecked_transaction()?;
    locations.iter().for_each(|li| {
        stmt.execute(params![
            li.city,
            li.county,
            li.state,
            li.country_code,
            li.postal_code,
            li.is_open,
            li.address,
            li.id,
            li.category,
            li.name,
            li.website_uri
        ])
        .expect("query to execute");
    });
    t.commit()
}
