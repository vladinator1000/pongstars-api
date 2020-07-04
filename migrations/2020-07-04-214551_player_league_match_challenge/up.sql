BEGIN;

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

CREATE TABLE match (
  id SERIAL PRIMARY KEY NOT NULL,
  player_one UUID NOT NULL REFERENCES player(id),
  player_two UUID NOT NULL REFERENCES player(id),
  score_one INT NOT NULL DEFAULT 0,
  score_two INT NOT NULL DEFAULT 0,
  length INT NOT NULL DEFAULT 3,
  created_at TIMESTAMP NOT NULL,
  played_at TIMESTAMP
);

CREATE TABLE match_league (
  match SERIAL NOT NULL REFERENCES match(id),
  league SERIAL NOT NULL REFERENCES league(id),
  PRIMARY KEY (match, league)
);

CREATE TABLE challenge (
  sender UUID NOT NULL REFERENCES player(id),
  receiver UUID NOT NULL REFERENCES player(id),
  PRIMARY KEY (sender, receiver),
  created_at TIMESTAMP NOT NULL,
  expires_at TIMESTAMP NOT NULL
);

COMMIT;