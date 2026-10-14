CREATE TABLE IF NOT EXISTS writeup (
    id            BIGSERIAL PRIMARY KEY,
    user_id       BIGINT        NOT NULL,
    user_account  VARCHAR(255)  NOT NULL,
    user_nickname VARCHAR(255)  NOT NULL,
    game_id       BIGINT        NOT NULL,
    game_name     VARCHAR(255)  NOT NULL,
    file_path     VARCHAR(1024) NOT NULL,
    file_name     VARCHAR(255)  NOT NULL,
    file_size     BIGINT        NOT NULL,
    file_type     VARCHAR(10)   NOT NULL,
    submitted_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_user_game UNIQUE (user_id, game_id)
);

CREATE INDEX IF NOT EXISTS idx_writeup_game_id ON writeup (game_id);
CREATE INDEX IF NOT EXISTS idx_writeup_user_id ON writeup (user_id);
CREATE INDEX IF NOT EXISTS idx_writeup_submitted_at ON writeup (submitted_at DESC);
