use crate::data::DbId;
use crate::{ClipError, ShortCode, Time};
use chrono::{NaiveDateTime, Utc};
use std::convert::TryFrom;
use crate::domain::clip::field;

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64
}

impl TryFrom<Clip> for crate::domain::clip::Clip {
    type Error = ClipError;

    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        use std::str::FromStr;
        Ok(
            Self {
                clip_id: field::ClipId::new(DbId::from_str(clip.clip_id.as_str())?),
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

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self {
            shortcode: shortcode.into_inner()
        }
    }
}

impl From<String> for GetClip {
    fn from(shortcode: String) -> Self {
        Self {
            shortcode
        }
    }
}

pub struct NewClip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: i64,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
}

pub struct UpdateClip {
    // can't update id and posted date
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}
