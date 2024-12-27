use axum::{extract::Query, response::Html};
use serde::Deserialize;

use crate::views::real_estate::pages::{render_home_page, render_property_details_page, render_search_result_page};

#[derive(Deserialize, Debug)]
pub enum ListingType {
    Sale,
    NewDev,
    ShortRental,
    LongRental
}

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub page: Option<u32>,
    pub listing_type: Option<ListingType>
}

pub async fn get_real_estate_home_page() -> Html<String> {
    Html(render_home_page().into_string())
}

pub async fn get_real_estate_search_result_page(
    search_query: Query<SearchQuery>,
) -> Html<String> {
    Html(render_search_result_page(search_query.0).into_string())
}

pub async fn get_real_estate_property_page() -> Html<String> {
    Html(render_property_details_page().into_string())
}