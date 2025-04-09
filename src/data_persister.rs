use rusqlite::{Connection, Error};

use crate::data_parser::LocationInfo;

pub fn upsert_locations(conn: &Connection, locations: &Vec<LocationInfo>) -> usize {
    let mut sql: String = "INSERT OR REPLACE INTO locations (
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
                    VALUES ?"
        .to_string();
    locations.iter().for_each(|li| {
        let inserted_cols = format!(
            "
                (
                    {},
                    {},
                    {},
                    {},
                    {},
                    {},
                    {},
                    {},
                    {},
                    {},
                    {}
                ),
            ",
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
        );
        sql += &inserted_cols;
    });

    conn.execute(stringify!(sql), ())
        .expect("Data to write to db")
}
