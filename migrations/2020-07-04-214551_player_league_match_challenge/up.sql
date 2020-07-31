CREATE TABLE players (
  -- the Firebase user ID
  id VARCHAR PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL
);

CREATE TABLE leagues (
  id SERIAL PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL
);

CREATE TABLE player_league (
  player VARCHAR NOT NULL REFERENCES players(id),
  league SERIAL NOT NULL REFERENCES leagues(id),
  PRIMARY KEY (player, league),
  -- Jon says this could be a view
  matchmaking_rating INT NOT NULL DEFAULT 1000,
  matches_played INT NOT NULL DEFAULT 0,
  wins INT NOT NULL DEFAULT 0,
  losses INT NOT NULL DEFAULT 0
);

CREATE TABLE matches (
  id SERIAL PRIMARY KEY NOT NULL,
  league SERIAL REFERENCES leagues(id),
  player_one VARCHAR NOT NULL REFERENCES players(id),
  player_two VARCHAR NOT NULL REFERENCES players(id),
  length INT NOT NULL DEFAULT 5,
  created_at TIMESTAMP NOT NULL,
  played_at TIMESTAMP
);

CREATE TABLE games (
  id SERIAL PRIMARY KEY NOT NULL,
  table_tennis_match SERIAL NOT NULL REFERENCES matches(id),
  score_one INT NOT NULL DEFAULT 0,
  score_two INT NOT NULL DEFAULT 0
);

CREATE TABLE challenges (
  sender VARCHAR NOT NULL REFERENCES players(id),
  receiver VARCHAR REFERENCES players(id),
  PRIMARY KEY (sender, receiver),
  message TEXT,
  is_ranked BOOLEAN NOT NULL,
  created_at TIMESTAMP NOT NULL,
  expires_at TIMESTAMP NOT NULL
);