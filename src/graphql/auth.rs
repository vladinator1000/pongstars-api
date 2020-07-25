use super::{mocks::mock_player, player::{Player}};

#[derive(juniper::GraphQLInputObject)]
pub struct SignupInput {
  pub first_name: String,
  pub surname: String,
  pub email: String,
  pub password: String
}

#[derive(juniper::GraphQLInputObject)]
pub struct LoginInput {
  pub email: String,
  pub password: String
}

pub struct LoginResult {
  pub player: Player,
  pub token: String
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
