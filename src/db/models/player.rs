use crate::db::schema::*;
use crate::db::{DbQueryRunner, Connection};
use diesel::prelude::*;

#[derive(Queryable, Debug, Identifiable, Insertable)]
pub struct Player {
    pub id: String,
    pub name: String,
}

impl<'a> DbQueryRunner<'a> {
    fn create(&self, new_player: Player) -> Player {
        diesel::insert_into(players::table)
            .values(&new_player)
            .get_result(self.connection)
            .expect("Error creating player")
    }
}