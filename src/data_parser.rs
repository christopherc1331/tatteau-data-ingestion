use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct LocationInfo {
    pub city: String,
    pub county: String,
    pub state: String,
    pub country_code: String,
    pub postal_code: String,
    pub is_open: bool,
    pub address: String,
    pub id: String,
    pub category: String,
    pub name: String,
    pub website_uri: String,
}

#[derive(Debug)]
pub struct ParsedLocationData<'a> {
    pub location_info: Vec<LocationInfo>,
    pub next_token: Option<&'a str>,
}

fn convert_val_to_string(v: &Value) -> String {
    v.as_str().unwrap_or("").to_string()
}

fn convert_val_obj_to_location_info(val: &Value) -> LocationInfo {
    let postal_code: String = val["addressComponents"][7]["shortText"]
        .as_str()
        .or_else(|| val["addressComponents"][7]["longText"].as_str())
        .unwrap_or("")
        .to_string();

    LocationInfo {
        city: convert_val_to_string(&val["addressComponents"][3]["longText"]),
        county: convert_val_to_string(&val["addressComponents"][4]["longText"]),
        state: convert_val_to_string(&val["addressComponents"][5]["longText"]),
        country_code: convert_val_to_string(&val["addressComponents"][6]["longText"]),
        postal_code,
        is_open: convert_val_to_string(&val["businessStatus"]) == "OPERATIONAL",
        address: convert_val_to_string(&val["formattedAddress"]),
        id: convert_val_to_string(&val["id"]),
        category: convert_val_to_string(&val["primaryType"]),
        name: convert_val_to_string(&val["displayName"]["text"]),
        website_uri: convert_val_to_string(&val["websiteUri"]),
    }
}

pub fn parse_data(value: &Value) -> Option<ParsedLocationData> {
    println!("{:#?}", value);
    let location_info: Option<Vec<LocationInfo>> = match &value["places"] {
        Value::Array(v) => Some(v.iter().map(convert_val_obj_to_location_info).collect()),
        _ => None,
    };

    location_info.map(|li| ParsedLocationData {
        location_info: li,
        next_token: value["nextPageToken"].as_str(),
    })
}
