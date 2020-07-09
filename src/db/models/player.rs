use crate::db::Connection;
use crate::db::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Debug, Identifiable, Insertable)]
pub struct Player {
    pub id: String,
    pub name: String,
}

pub fn create(connection: &Connection, new_player: Player) -> Player {
    diesel::insert_into(players::table)
        .values(&new_player)
        .get_result(connection)
        .expect("Error creating player")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::pool::get_pool;

    #[test]
    fn create_player() {
        let pool = get_pool();
        let connection = pool.get().unwrap();

        let new_player = Player {
            id: "Hello".into(),
            name: "Jon".into(),
        };

        let result = create(&connection, new_player);

        assert_eq!(result.name, "Jon")
    }
}
