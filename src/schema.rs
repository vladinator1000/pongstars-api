table! {
    challenge (sender, receiver) {
        sender -> Uuid,
        receiver -> Uuid,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

table! {
    game (id) {
        id -> Int4,
        #[sql_name = "match"]
        match_ -> Int4,
        score_one -> Int4,
        score_two -> Int4,
    }
}

table! {
    league (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    match (id) {
        id -> Int4,
        league -> Int4,
        player_one -> Uuid,
        player_two -> Uuid,
        length -> Match_length,
        created_at -> Timestamp,
        played_at -> Nullable<Timestamp>,
    }
}

table! {
    player (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    player_league (player, league) {
        player -> Uuid,
        league -> Int4,
    }
}

joinable!(game -> match (match));
joinable!(match -> league (league));
joinable!(player_league -> league (league));
joinable!(player_league -> player (player));

allow_tables_to_appear_in_same_query!(
    challenge,
    game,
    league,
    match,
    player,
    player_league,
);
