use super::player::{mock_player, Player};

#[derive(juniper::GraphQLInputObject)]
pub struct SignupInput {
  first_name: String,
  surname: String,
  email: String,
  password: String
}

#[derive(juniper::GraphQLInputObject)]
pub struct LoginInput {
  email: String,
  password: String
}

pub struct LoginResult {
  player: Player,
  token: String
}

#[juniper::object]
impl LoginResult {
  fn player(&self) -> Player {
      mock_player()
  }

  fn token(&self) -> &str {
      &self.token
  }
}

pub fn mock_login_result() -> LoginResult {
  LoginResult {
      player: mock_player(),
      token: "token".to_string()
  }
}

