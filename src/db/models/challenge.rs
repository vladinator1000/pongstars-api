use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Challenge {
    pub sender: String,
    pub receiver: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}
