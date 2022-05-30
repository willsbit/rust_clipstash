use super::model;
use crate::data::{DataError, DatabasePool};
use crate::ShortCode;
use sqlx::Row;
use crate::web::api::ApiKey;

type Result<T> = std::result::Result<T, DataError>;

/// Updates the database and increases the hits field. A hit is an access to a clip.
pub async fn increase_hit_count(
    shortcode: &ShortCode,
    hits: u32,
    pool: &DatabasePool
) -> Result<()> {
    let shortcode = shortcode.as_str();

    Ok(sqlx::query!(
        r#"UPDATE clips
        SET hits = hits + $1
        WHERE shortcode = $2"#,
        hits as i64,
        shortcode
    )
        .execute(pool)
        .await
        .map(|_| ())?)
}


pub async fn get_clip<M: Into<model::GetClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(sqlx::query_as!(
        model::Clip,
        r#"SELECT * FROM clips WHERE shortcode = $1"#,
        shortcode
    ).fetch_one(pool).await?)
}

pub async fn new_clip<M: Into<model::NewClip>>(
    model: M,
    pool: &DatabasePool
) -> Result<model::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"INSERT INTO clips (
            clip_id,
            shortcode,
            content,
            title,
            posted,
            expires,
            password,
            hits)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0
    ).execute(pool).await?;

    get_clip(model.shortcode, pool).await

}

pub async fn update_clip<M:Into<model::UpdateClip>>(
    model: M,
    pool: &DatabasePool
) -> Result<model::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"UPDATE clips SET
            content = $1,
            title = $2,
            expires = $3,
            password = $4
        WHERE shortcode = $5"#,
        model.content,
        model.title,
        model.expires,
        model.password,
        model.shortcode
    ).execute(pool).await?;

    get_clip(model.shortcode, pool).await
}

/// Saves an [`ApiKey`].
pub async fn save_api_key(api_key: ApiKey, pool: &DatabasePool) -> Result<ApiKey> {
    let bytes = api_key.clone().into_inner();
    let _ = sqlx::query!("INSERT INTO api_keys (api_key) VALUES ($1)", bytes)
        .execute(pool)
        .await
        .map(|_| ())?;
    Ok(api_key)
}

/// The return value from the [`revoke_api_key`] function.
pub enum RevocationStatus {
    /// The [`ApiKey`] was successfully revoked.
    Revoked,
    /// The [`ApiKey`] was not found, so no revocation occuured.
    NotFound
}

/// Revokes an [`ApiKey`].
pub async fn revoke_api_key(api_key: ApiKey, pool: &DatabasePool) -> Result<RevocationStatus> {
    let bytes = api_key.clone().into_inner();
    Ok(
        sqlx::query!("DELETE FROM api_keys WHERE api_key = $1", bytes)
            .execute(pool)
            .await
            .map(|result| match result.rows_affected() {
                0 => RevocationStatus::NotFound,
                _ => RevocationStatus::Revoked
            })?,
    )
}

/// Determines if the [`ApiKey`] is valid.
pub async fn api_key_is_valid(api_key: ApiKey, pool: &DatabasePool) -> Result<bool> {
    let bytes = api_key.clone().into_inner();
    Ok(
        sqlx::query("SELECT COUNT(api_key) FROM api_keys WHERE api_key = $1")
            .bind(bytes)
            .fetch_one(pool)
            .await
            .map(|row| {
                let count: u32 = row.get(0);
                count > 0
            })?,
    )
}

/// Deletes all expired [`Clips`](`crate::Clip`).
pub async fn delete_expired(pool: &DatabasePool) -> Result<u64> {
    Ok(
        sqlx::query!(r#"DELETE FROM clips WHERE extract(epoch from now()) > extract(epoch from expires)"#)
            .execute(pool)
            .await?
            .rows_affected()
    )
}


#[cfg(test)]
pub mod test {
    use chrono::NaiveDateTime;
    use crate::data::test::*;
    use crate::data::*;
    use crate::test::async_runtime;
    use crate::Time;

    fn model_new_clip(shortcode: &str) -> model::NewClip {
        use chrono::Utc;
        model::NewClip {
            clip_id: DbId::new().into(),
            content: format!("content for clip '{}'", shortcode),
            title: None,
            shortcode: shortcode.into(),
            posted: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
            expires: None,
            password: None
        }
    }

    #[test]
    fn clip_new_and_get() {
        let rt = async_runtime();
        let db = new_db(rt.handle());
        let pool = db.get_pool();

        let test_shortcode = "bdbd4b3cb4";

        let clip = rt.block_on(async move {
            super::new_clip(model_new_clip(test_shortcode), &pool.clone()).await
        });

        assert!(clip.is_ok());
        let clip = clip.unwrap();

        assert_eq!(clip.shortcode, test_shortcode);
        assert_eq!(clip.content, format!("content for clip '{}'", test_shortcode));

    }
}