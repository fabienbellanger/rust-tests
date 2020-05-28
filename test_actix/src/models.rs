use serde::Serialize;

#[derive(Serialize)]
pub struct Status {
    pub status: &'static str,
    pub message: String,
}

#[derive(Serialize)]
pub struct Task {
    pub id: u32,
    pub name: &'static str,
    pub message: String,
}
