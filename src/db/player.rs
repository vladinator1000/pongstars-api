use super::pool::Connection;
use diesel::prelude::*;

use super::models::Player;

pub struct PlayerDataFunctions;

impl PlayerDataFunctions {
    fn create(conn: &Connection, new_player: Player) -> Player {
        // diesel::insert_into(player::table)
        //     .values(&new_player)
        //     .get_result(conn)
        //     .expect("Error creating player")
        unimplemented!()
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::pool::get_pool;

    #[test]
    fn create_player() {
        let pool = get_pool();
        let conn = pool.get().unwrap();

        let new_player = Player {
            id: "Hello".into(),
            name: "Jon".into(),
        };

        let result = Player::create(conn, new_player);

        assert_eq!(result.name, "Jon")
    }
}
