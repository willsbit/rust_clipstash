use serde::Serialize;
use std::collections::HashMap;
use derive_more::Constructor;

pub trait PageContext {
    fn title(&self) -> &str;
    fn template_path(&self) -> &str;
    fn parent(&self) -> &str;
}
#[derive(Debug, Serialize)]
pub struct Home {}

impl Default for Home {
    fn default() -> Self {
        Self {}
    }
}

impl PageContext for Home {
    fn title(&self) -> &str {
        "Clipboard Stash"
    }
    fn template_path(&self) -> &str {
        "home"
    }
    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct ViewClip {
    pub clip: crate::Clip
}

impl PageContext for ViewClip {
    fn title(&self) -> &str {
        "View clip"
    }
    fn template_path(&self) -> &str {
        "clip"
    }
    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct PasswordRequired {
    shortcode: crate::ShortCode
}

impl PageContext for PasswordRequired {
    fn title(&self) -> &str {
        "Password required"
    }
    fn template_path(&self) -> &str {
        "clip_need_password"
    }
    fn parent(&self) -> &str {
        "base"
    }
}