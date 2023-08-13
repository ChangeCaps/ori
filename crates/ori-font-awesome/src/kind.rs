use ori_graphics::FontFamily;

include!(concat!(env!("OUT_DIR"), "/icons.rs"));

impl From<&str> for IconKind {
    fn from(value: &str) -> Self {
        Self::from_name(value).unwrap_or(Self::Question)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IconFont {
    /// The regular font.
    Regular,
    /// The solid font.
    Solid,
    /// The brand font.
    Brand,
}

impl IconFont {
    pub fn family(self) -> FontFamily {
        let name = match self {
            Self::Regular | Self::Solid => "Font Awesome 6 Free",
            Self::Brand => "Font Awesome 6 Brands",
        };

        FontFamily::Name(String::from(name))
    }
}
