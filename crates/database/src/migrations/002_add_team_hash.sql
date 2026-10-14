ALTER TABLE writeup ADD COLUMN IF NOT EXISTS team_id BIGINT;
ALTER TABLE writeup ADD COLUMN IF NOT EXISTS team_name VARCHAR(255);
ALTER TABLE writeup ADD COLUMN IF NOT EXISTS file_hash VARCHAR(64);

-- Add unique constraint for competition (team_id, game_id)
ALTER TABLE writeup DROP CONSTRAINT IF EXISTS uq_team_game;
ALTER TABLE writeup ADD CONSTRAINT uq_team_game UNIQUE (team_id, game_id);

CREATE INDEX IF NOT EXISTS idx_writeup_file_hash ON writeup (file_hash);
CREATE INDEX IF NOT EXISTS idx_writeup_team_id ON writeup (team_id);
