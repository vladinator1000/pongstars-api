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
  PRIMARY KEY (player, league)
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
  tt_match SERIAL NOT NULL REFERENCES matches(id),
  score_one INT NOT NULL DEFAULT 0,
  score_two INT NOT NULL DEFAULT 0
);

CREATE TABLE challenges (
  sender VARCHAR NOT NULL REFERENCES players(id),
  receiver VARCHAR NOT NULL REFERENCES players(id),
  PRIMARY KEY (sender, receiver),
  created_at TIMESTAMP NOT NULL,
  expires_at TIMESTAMP NOT NULL
);