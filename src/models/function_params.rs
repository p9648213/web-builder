use super::rso_data::SearchProperty;

pub struct RenderPropertyGrid<'a> {
    pub properties: &'a Vec<SearchProperty>,
    pub property_count: u32,
    pub properties_per_page: u32,
    pub page_no: u32,
    pub listing_type: &'a str,
    pub province: &'a str,
    pub location: &'a str,
    pub property_type: &'a str,
}
