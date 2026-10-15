use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Writeup {
    pub id: i64,
    pub user_id: i64,
    pub user_account: String,
    pub user_nickname: String,
    pub team_id: Option<i64>,
    pub team_name: Option<String>,
    pub game_id: i64,
    pub game_name: String,
    pub file_path: String,
    pub file_name: String,   // display name: "{game}-{team/user}.{ext}"
    pub file_size: i64,
    pub file_type: String,
    pub file_hash: Option<String>,
    pub submitted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct UpsertWriteup<'a> {
    pub user_id: i64,
    pub user_account: &'a str,
    pub user_nickname: &'a str,
    pub team_id: Option<i64>,
    pub team_name: Option<&'a str>,
    pub game_id: i64,
    pub game_name: &'a str,
    pub file_path: &'a str,
    pub file_name: &'a str,
    pub file_size: i64,
    pub file_type: &'a str,
    pub file_hash: &'a str,
}

/// For competitions: upsert by (team_id, game_id). For training: upsert by (user_id, game_id).
pub async fn upsert(pool: &PgPool, w: UpsertWriteup<'_>) -> Result<Writeup> {
    let row = if let Some(team_id) = w.team_id {
        // Competition: shared per team
        sqlx::query_as::<_, Writeup>(
            r#"
            INSERT INTO writeup
                (user_id, user_account, user_nickname, team_id, team_name, game_id, game_name,
                 file_path, file_name, file_size, file_type, file_hash, submitted_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW(), NOW())
            ON CONFLICT (team_id, game_id) DO UPDATE SET
                user_id       = EXCLUDED.user_id,
                user_account  = EXCLUDED.user_account,
                user_nickname = EXCLUDED.user_nickname,
                team_name     = EXCLUDED.team_name,
                game_name     = EXCLUDED.game_name,
                file_path     = EXCLUDED.file_path,
                file_name     = EXCLUDED.file_name,
                file_size     = EXCLUDED.file_size,
                file_type     = EXCLUDED.file_type,
                file_hash     = EXCLUDED.file_hash,
                updated_at    = NOW()
            RETURNING *
            "#,
        )
        .bind(w.user_id)
        .bind(w.user_account)
        .bind(w.user_nickname)
        .bind(team_id)
        .bind(w.team_name)
        .bind(w.game_id)
        .bind(w.game_name)
        .bind(w.file_path)
        .bind(w.file_name)
        .bind(w.file_size)
        .bind(w.file_type)
        .bind(w.file_hash)
        .fetch_one(pool)
        .await?
    } else {
        // Training: per user
        sqlx::query_as::<_, Writeup>(
            r#"
            INSERT INTO writeup
                (user_id, user_account, user_nickname, game_id, game_name,
                 file_path, file_name, file_size, file_type, file_hash, submitted_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW())
            ON CONFLICT (user_id, game_id) WHERE team_id IS NULL DO UPDATE SET
                user_account  = EXCLUDED.user_account,
                user_nickname = EXCLUDED.user_nickname,
                game_name     = EXCLUDED.game_name,
                file_path     = EXCLUDED.file_path,
                file_name     = EXCLUDED.file_name,
                file_size     = EXCLUDED.file_size,
                file_type     = EXCLUDED.file_type,
                file_hash     = EXCLUDED.file_hash,
                updated_at    = NOW()
            RETURNING *
            "#,
        )
        .bind(w.user_id)
        .bind(w.user_account)
        .bind(w.user_nickname)
        .bind(w.game_id)
        .bind(w.game_name)
        .bind(w.file_path)
        .bind(w.file_name)
        .bind(w.file_size)
        .bind(w.file_type)
        .bind(w.file_hash)
        .fetch_one(pool)
        .await?
    };
    Ok(row)
}

pub async fn get_old_file_path(
    pool: &PgPool,
    user_id: i64,
    game_id: i64,
    team_id: Option<i64>,
) -> Result<Option<String>> {
    let row: Option<String> = if let Some(tid) = team_id {
        sqlx::query_scalar(
            "SELECT file_path FROM writeup WHERE team_id = $1 AND game_id = $2",
        )
        .bind(tid)
        .bind(game_id)
        .fetch_optional(pool)
        .await?
    } else {
        sqlx::query_scalar(
            "SELECT file_path FROM writeup WHERE user_id = $1 AND game_id = $2 AND team_id IS NULL",
        )
        .bind(user_id)
        .bind(game_id)
        .fetch_optional(pool)
        .await?
    };
    Ok(row)
}

/// For competitions: find by team. For training: find by user.
pub async fn find_by_user_and_game(
    pool: &PgPool,
    user_id: i64,
    game_id: i64,
    team_id: Option<i64>,
) -> Result<Option<Writeup>> {
    let row = if let Some(tid) = team_id {
        sqlx::query_as::<_, Writeup>(
            "SELECT * FROM writeup WHERE team_id = $1 AND game_id = $2",
        )
        .bind(tid)
        .bind(game_id)
        .fetch_optional(pool)
        .await?
    } else {
        sqlx::query_as::<_, Writeup>(
            "SELECT * FROM writeup WHERE user_id = $1 AND game_id = $2 AND team_id IS NULL",
        )
        .bind(user_id)
        .bind(game_id)
        .fetch_optional(pool)
        .await?
    };
    Ok(row)
}

/// Admin: find by writeup ID and game ID.
pub async fn find_by_id_and_game(
    pool: &PgPool,
    writeup_id: i64,
    game_id: i64,
) -> Result<Option<Writeup>> {
    let row = sqlx::query_as::<_, Writeup>(
        "SELECT * FROM writeup WHERE id = $1 AND game_id = $2",
    )
    .bind(writeup_id)
    .bind(game_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn find_by_user_or_teams(
    pool: &PgPool,
    user_id: i64,
    team_ids: &[i64],
    page: i64,
    page_size: i64,
) -> Result<(Vec<Writeup>, i64)> {
    let offset = (page - 1) * page_size;
    let (rows, total) = if team_ids.is_empty() {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM writeup WHERE user_id = $1 AND team_id IS NULL")
            .bind(user_id)
            .fetch_one(pool)
            .await?;
        let rows = sqlx::query_as::<_, Writeup>(
            "SELECT * FROM writeup WHERE user_id = $1 AND team_id IS NULL ORDER BY updated_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        (rows, total)
    } else {
        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM writeup WHERE (user_id = $1 AND team_id IS NULL) OR team_id = ANY($2)",
        )
        .bind(user_id)
        .bind(team_ids)
        .fetch_one(pool)
        .await?;
        let rows = sqlx::query_as::<_, Writeup>(
            "SELECT * FROM writeup WHERE (user_id = $1 AND team_id IS NULL) OR team_id = ANY($2) ORDER BY updated_at DESC LIMIT $3 OFFSET $4",
        )
        .bind(user_id)
        .bind(team_ids)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        (rows, total)
    };
    Ok((rows, total))
}

pub async fn find_by_user(
    pool: &PgPool,
    user_id: i64,
    page: i64,
    page_size: i64,
) -> Result<(Vec<Writeup>, i64)> {
    find_by_user_or_teams(pool, user_id, &[], page, page_size).await
}

pub async fn find_by_game(
    pool: &PgPool,
    game_id: i64,
    page: i64,
    page_size: i64,
) -> Result<(Vec<Writeup>, i64)> {
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM writeup WHERE game_id = $1")
        .bind(game_id)
        .fetch_one(pool)
        .await?;

    let rows = sqlx::query_as::<_, Writeup>(
        "SELECT * FROM writeup WHERE game_id = $1 ORDER BY submitted_at DESC LIMIT $2 OFFSET $3",
    )
    .bind(game_id)
    .bind(page_size)
    .bind((page - 1) * page_size)
    .fetch_all(pool)
    .await?;

    Ok((rows, total))
}

pub async fn find_all_by_game(pool: &PgPool, game_id: i64) -> Result<Vec<Writeup>> {
    let rows = sqlx::query_as::<_, Writeup>(
        "SELECT * FROM writeup WHERE game_id = $1 ORDER BY submitted_at ASC",
    )
    .bind(game_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn delete_by_id(pool: &PgPool, id: i64) -> Result<()> {
    sqlx::query("DELETE FROM writeup WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Check if the same file hash was already submitted by another team or user for this game.
pub async fn find_duplicate_hash(
    pool: &PgPool,
    game_id: i64,
    user_id: i64,
    team_id: Option<i64>,
    file_hash: &str,
) -> Result<bool> {
    let exists: bool = if let Some(tid) = team_id {
        sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM writeup WHERE game_id = $1 AND team_id IS DISTINCT FROM $2 AND file_hash = $3)",
        )
        .bind(game_id)
        .bind(tid)
        .bind(file_hash)
        .fetch_one(pool)
        .await?
    } else {
        sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM writeup WHERE game_id = $1 AND (user_id != $2 OR team_id IS NOT NULL) AND file_hash = $3)",
        )
        .bind(game_id)
        .bind(user_id)
        .bind(file_hash)
        .fetch_one(pool)
        .await?
    };
    Ok(exists)
}
