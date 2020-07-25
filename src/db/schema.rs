table! {
    challenges (sender, receiver) {
        sender -> Varchar,
        receiver -> Varchar,
        message -> Nullable<Text>,
        is_ranked -> Bool,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

table! {
    games (id) {
        id -> Int4,
        tt_match -> Int4,
        score_one -> Int4,
        score_two -> Int4,
    }
}

table! {
    leagues (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    matches (id) {
        id -> Int4,
        league -> Int4,
        player_one -> Varchar,
        player_two -> Varchar,
        length -> Int4,
        created_at -> Timestamp,
        played_at -> Nullable<Timestamp>,
    }
}

table! {
    player_league (player, league) {
        player -> Varchar,
        league -> Int4,
        matchmaking_rating -> Int4,
        matches_played -> Int4,
        wins -> Int4,
        losses -> Int4,
    }
}

table! {
    players (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

joinable!(games -> matches (tt_match));
joinable!(matches -> leagues (league));
joinable!(player_league -> leagues (league));
joinable!(player_league -> players (player));

allow_tables_to_appear_in_same_query!(
    challenges,
    games,
    leagues,
    matches,
    player_league,
    players,
);
