use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Match {
    pub id: i32,
    pub league: i32,
    pub player_one: String,
    pub player_two: String,
    pub length: i32,
    pub created_at: NaiveDateTime,
    pub played_at: Option<NaiveDateTime>,
}
