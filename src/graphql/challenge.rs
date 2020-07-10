use super::league::{mock_league, League};
use super::player::{mock_player, Player};
pub use crate::{db::models::challenge::*, DateTime};
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
