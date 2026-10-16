-- Personal submissions are unique per user; team submissions are unique per team.
-- Keep former teams' submissions when a user joins a different team.
ALTER TABLE writeup DROP CONSTRAINT uq_user_game;

CREATE UNIQUE INDEX uq_writeup_personal_user_game
    ON writeup (user_id, game_id) WHERE team_id IS NULL;
