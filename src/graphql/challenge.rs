use super::league::League;
use super::{mocks::mock_player, player::Player};
pub use crate::{db::models::challenge::*, DateTimeUtc};
use chrono::Utc;

#[juniper::object]
impl Challenge {
    fn sender(&self) -> Player {
        mock_player()
    }

    fn receiver(&self) -> Option<Player> {
        Some(mock_player())
    }

    fn is_ranked(&self) -> bool {
        true
    }

    fn created_at(&self) -> DateTimeUtc {
        DateTimeUtc::from_utc(self.created_at, Utc)
    }

    fn expires_at(&self) -> DateTimeUtc {
        DateTimeUtc::from_utc(self.expires_at, Utc)
    }
}
