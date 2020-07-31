use crate::{DateTimeUtc, db::table_tennis_match::TableTennisMatch};
use super::{context::GraphQLContext, player::Player, league::League, mocks::{mock_player, mock_league, mock_date_time}};

#[juniper::object(Context=GraphQLContext)]
impl TableTennisMatch {
  fn id(&self) -> i32 {
    self.id
  }

  fn league(&self) -> League {
    mock_league()
  }

  fn player_one(&self) -> Player {
    mock_player()
  }

  fn player_two(&self) -> Player {
    mock_player()
  }

  fn length(&self) -> MatchLength {
    MatchLength::Five
  }

  fn created_at(&self) -> DateTimeUtc {
    mock_date_time()
  }

  fn played_at(&self) -> Option<DateTimeUtc> {
    Some(mock_date_time())
  }
}


#[derive(juniper::GraphQLEnum, Copy, Clone)]
enum MatchLength { 
  Five = 5,
  Seven = 7,
  Nine = 9 
}

