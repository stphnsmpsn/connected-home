use crate::api::api::make_response;
use hyper::Body;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::http::{Response, StatusCode};

#[derive(Debug, Serialize, Deserialize)]
struct Country {
    name: String,
    capital: String,
    population: u128,
}

impl Country {
    pub fn new(name: &str, capital: &str, population: u128) -> Self {
        Self {
            name: name.to_string(),
            capital: capital.to_string(),
            population,
        }
    }
}

pub async fn countries() -> Response<Body> {
    let mut countries = HashMap::<String, Country>::new();
    countries.insert(
        String::from("Canada"),
        Country::new("Canada", "Ottawa", 37000000),
    );
    countries.insert(
        String::from("Afghanistan"),
        Country::new("Afghanistan", "Kabul", 38930000),
    );
    countries.insert(
        String::from("Jamaica"),
        Country::new("Jamaica", "Kingston", 2961000),
    );
    countries.insert(
        String::from("Cypress"),
        Country::new("Cyprus", "Nicosia", 1207000),
    );
    countries.insert(
        String::from("United States"),
        Country::new("United States", "Washington, D.C", 329500000),
    );
    countries.insert(
        String::from("United Kingdom"),
        Country::new("United Kingdom", "London", 672200000),
    );
    countries.insert(
        String::from("Ukraine"),
        Country::new("Ukraine", "Kiev", 44130000),
    );

    make_response(
        StatusCode::OK,
        Some(serde_json::to_string(&countries).unwrap()),
    )
}
