use crate::DateTimeUtc;
use super::{player::{Stats, Player, TableTennisMatch}, league::{League}, challenge::Challenge, auth::LoginResult};
use chrono::{Utc, NaiveDate, NaiveDateTime};

pub fn mock_naive_date() -> NaiveDateTime {
  NaiveDate::from_ymd(2021, 1, 8).and_hms(10, 0, 0) as NaiveDateTime
}

pub fn mock_date_time() -> DateTimeUtc {
  let date_time = NaiveDate::from_ymd(2021, 1, 8).and_hms(10, 0, 0);
  DateTimeUtc::from_utc(date_time, Utc)
}

pub fn mock_league() -> League {
  League {
      id: 1,
      name: "GLA".into(),
  }
}

pub fn mock_player() -> Player {
  Player {
      id: "1".into(),
      name: "Jon".into(),
  }
}

pub fn mock_table_tennis_match() -> TableTennisMatch {
  TableTennisMatch {
    id: 1,
    league: 1,
    player_one: "Jon".to_string(),
    player_two: "Vlady".to_string(),
    length: 5,
    created_at: mock_naive_date(),
    played_at: None
  }
}

pub fn mock_stats() -> Stats {
  Stats {
    matchmaking_rating: 1000,
    matches_played: 5,
    wins: 3,
    losses: 2,
  }
}

pub fn mock_challenge() -> Challenge {
  let date_time = NaiveDate::from_ymd(2021, 1, 8).and_hms(10, 0, 0);

  Challenge {
      sender: "Vladinator".into(),
      receiver: Some("Jon".into()),
      message: Some("Looking for a game before lunch.".into()),
      created_at: date_time,
      expires_at: date_time,
      is_ranked: false,
  }
}