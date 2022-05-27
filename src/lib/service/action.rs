use crate::data::{query, DatabasePool, Transaction};
use crate::service::ask;
use crate::{Clip, ShortCode, ServiceError};
use std::convert::TryInto;
use rocket::Data;

pub async fn get_clip(req: ask::GetClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    let user_pass = req.password.clone();
    let clip: Clip = query::get_clip(req, pool).await?.try_into()?;
    if clip.password.has_password() {
        if clip.password.to_str() == user_pass.to_str() {
            Ok(clip)
        } else {
            Err(ServiceError::PermissionError("Invalid password".to_owned()))
        }
    } else {
        Ok(clip)
    }
}

pub async fn new_clip(req: ask::NewClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    Ok(query::new_clip(req, pool).await?.try_into()?)
}

pub async fn update(req: ask::UpdateClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    Ok(query::update_clip(req, pool).await?.try_into()?)
}

pub async fn increase_hit_count(
    shortcode: &ShortCode,
    hits: u32,
    pool: &DatabasePool
) -> Result<(), ServiceError> {
    Ok(query::increase_hit_count(shortcode, hits, pool).await?)
}

pub async fn begin_transaction(pool: &DatabasePool) -> Result<Transaction<'_>, ServiceError> {
    Ok(pool.begin().await?)
}


pub async fn end_transaction(transaction: Transaction<'_>) -> Result<(), ServiceError> {
    Ok(transaction.commit().await?)
}