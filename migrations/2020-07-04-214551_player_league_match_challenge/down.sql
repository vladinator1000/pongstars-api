DROP TABLE challenges CASCADE;

DROP TABLE games CASCADE;

DROP TABLE matches CASCADE;

DELETE FROM
  pg_type
WHERE
  typname = 'match_length';

DROP TABLE player_league CASCADE;

DROP TABLE players CASCADE;

DROP TABLE leagues CASCADE;