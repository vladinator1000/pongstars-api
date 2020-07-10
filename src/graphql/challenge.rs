use super::league::{League, mock_league};
use super::player::{Player, mock_player};
pub use crate::{DateTime, db::models::challenge::*};
use chrono::{NaiveDate, Utc};

#[juniper::object]
impl Challenge {
    fn sender(&self) -> Player {
      mock_player()
    }
    
    fn receiver(&self) -> Player {
      mock_player()
    }

    fn created_at(&self) -> DateTime {
      DateTime::from_utc(self.created_at, Utc)
    }

    fn expires_at(&self) -> DateTime {
      DateTime::from_utc(self.expires_at, Utc)
    }
}

pub fn mock_challenge() -> Challenge {
    let date_time = NaiveDate::from_ymd(2021, 1, 8).and_hms(10, 0, 0);

    Challenge {
        sender: "Vladinator".into(),
        receiver: "Jon".into(),
        created_at: date_time,
        expires_at: date_time,
    }
}
