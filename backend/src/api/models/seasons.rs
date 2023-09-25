use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SeasonOut {
    id: i32,
    name: String,
    label: String,
}

impl From<crate::models::Season> for SeasonOut {
    fn from(s: crate::models::Season) -> Self {
        Self {
            id: s.id,
            name: s.name,
            label: s.label,
        }
    }
}
