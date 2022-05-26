use crate::data::DbId;
use crate::{ClipError, ShortCode, Time};
use chrono::{NaiveDateTime, Utc};
use std::convert::TryFrom;
use crate::domain::clip::field;

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub clip_id: String,
    pub shortcode: String,
    pub content: String,
    pub title: Option<String>,
    pub posted: NaiveDateTime,
    pub expires: Option<NaiveDateTime>,
    pub password: Option<String>,
    pub hits: i64
}

impl TryFrom<Clip> for crate::domain::Clip {
    type Error = ClipError;

    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        use std::str::FromStr;
        Ok(
            Self {
                clip_id: field::ClipId::new(DbId::try_from(clip.clip_id.as_str())?),
                shortcode: field::ShortCode::from(clip.shortcode.as_str()),
                content: field::Content::new(clip.content.as_str())?,
                title: field::Title::new(clip.title),
                posted: field::Posted::new(Time::from_naive_utc(clip.posted)),
                expires: field::Expires::new(clip.expires.map(Time::from_naive_utc)),
                password: field::Password::new(clip.password.unwrap_or_default())?,
                hits: field::Hits::new(u64::try_from(clip.hits)?)
            }
        )
    }
}

pub struct GetClip {
    pub(in crate::data) shortcode: String
}