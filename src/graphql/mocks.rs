use crate::DateTimeUtc;
use super::{player::{Stats, Player}, league::{League}, challenge::Challenge, auth::LoginResult};
use chrono::{Utc, NaiveDate};

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

pub fn mock_login_result() -> LoginResult {
  LoginResult {
      player: mock_player(),
      token: "token".to_string()
  }
}

