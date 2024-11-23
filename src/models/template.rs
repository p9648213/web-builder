use postgres_types::{FromSql, ToSql};

#[derive(Debug, ToSql, FromSql, Clone)]
pub enum TemplateType {
    Ecommerce,
    Custom,
}

pub struct Template {
    pub id: Option<i32>,
    pub template_type: Option<TemplateType>,
    pub description: Option<String>,
}
