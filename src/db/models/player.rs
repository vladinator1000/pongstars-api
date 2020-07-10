use crate::db::schema::*;
use crate::db::Connection;
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
