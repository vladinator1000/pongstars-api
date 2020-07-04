table! {
    challenge (sender, receiver) {
        sender -> Uuid,
        receiver -> Uuid,
        created_at -> Timestamp,
        expires_at -> Timestamp,
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
        player_one -> Uuid,
        player_two -> Uuid,
        score_one -> Int4,
        score_two -> Int4,
        length -> Int4,
        created_at -> Timestamp,
        played_at -> Nullable<Timestamp>,
    }
}

table! {
    match_league (match_, league) {
        #[sql_name = "match"]
        match_ -> Int4,
        league -> Int4,
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

joinable!(match_league -> league (league));
joinable!(match_league -> match (match));
joinable!(player_league -> league (league));
joinable!(player_league -> player (player));

allow_tables_to_appear_in_same_query!(
    challenge,
    league,
    match,
    match_league,
    player,
    player_league,
);
