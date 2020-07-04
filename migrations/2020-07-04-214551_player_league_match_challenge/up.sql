CREATE TABLE player (
  -- the Firebase user ID
  id UUID PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL
);

CREATE TABLE league (
  id SERIAL PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL
);

CREATE TABLE player_league (
  player UUID NOT NULL REFERENCES player(id),
  league SERIAL NOT NULL REFERENCES league(id),
  PRIMARY KEY (player, league)
);

CREATE TYPE match_length AS ENUM('5', '7', '9');

CREATE TABLE match (
  id SERIAL PRIMARY KEY NOT NULL,
  league SERIAL REFERENCES league(id),
  player_one UUID NOT NULL REFERENCES player(id),
  player_two UUID NOT NULL REFERENCES player(id),
  length match_length NOT NULL DEFAULT '5',
  created_at TIMESTAMP NOT NULL,
  played_at TIMESTAMP
);

CREATE TABLE game (
  id SERIAL PRIMARY KEY NOT NULL,
  match SERIAL NOT NULL REFERENCES match(id),
  score_one INT NOT NULL DEFAULT 0,
  score_two INT NOT NULL DEFAULT 0
);

CREATE TABLE challenge (
  sender UUID NOT NULL REFERENCES player(id),
  receiver UUID NOT NULL REFERENCES player(id),
  PRIMARY KEY (sender, receiver),
  created_at TIMESTAMP NOT NULL,
  expires_at TIMESTAMP NOT NULL
);