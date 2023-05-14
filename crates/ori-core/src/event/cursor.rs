use crate::StyleAttributeEnum;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Cursor {
    #[default]
    Default,
    Crosshair,
    Pointer,
    Arrow,
    Move,
    Text,
    Wait,
    Help,
    Progress,
    NotAllowed,
    ContextMenu,
    Cell,
    VerticalText,
    Alias,
    Copy,
    NoDrop,
    Grab,
    Grabbing,
    AllScroll,
    ZoomIn,
    ZoomOut,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
}

impl StyleAttributeEnum for Cursor {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "default" => Some(Self::Default),
            "crosshair" => Some(Self::Crosshair),
            "pointer" => Some(Self::Pointer),
            "arrow" => Some(Self::Arrow),
            "move" => Some(Self::Move),
            "text" => Some(Self::Text),
            "wait" => Some(Self::Wait),
            "help" => Some(Self::Help),
            "progress" => Some(Self::Progress),
            "not-allowed" => Some(Self::NotAllowed),
            "context-menu" => Some(Self::ContextMenu),
            "cell" => Some(Self::Cell),
            "vertical-text" => Some(Self::VerticalText),
            "alias" => Some(Self::Alias),
            "copy" => Some(Self::Copy),
            "no-drop" => Some(Self::NoDrop),
            "grab" => Some(Self::Grab),
            "grabbing" => Some(Self::Grabbing),
            "all-scroll" => Some(Self::AllScroll),
            "zoom-in" => Some(Self::ZoomIn),
            "zoom-out" => Some(Self::ZoomOut),
            "e-resize" => Some(Self::EResize),
            "n-resize" => Some(Self::NResize),
            "ne-resize" => Some(Self::NeResize),
            "nw-resize" => Some(Self::NwResize),
            "s-resize" => Some(Self::SResize),
            "se-resize" => Some(Self::SeResize),
            "sw-resize" => Some(Self::SwResize),
            "w-resize" => Some(Self::WResize),
            "ew-resize" => Some(Self::EwResize),
            "ns-resize" => Some(Self::NsResize),
            "nesw-resize" => Some(Self::NeswResize),
            "nwse-resize" => Some(Self::NwseResize),
            "col-resize" => Some(Self::ColResize),
            "row-resize" => Some(Self::RowResize),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Self::Default => "default",
            Self::Crosshair => "crosshair",
            Self::Pointer => "pointer",
            Self::Arrow => "arrow",
            Self::Move => "move",
            Self::Text => "text",
            Self::Wait => "wait",
            Self::Help => "help",
            Self::Progress => "progress",
            Self::NotAllowed => "not-allowed",
            Self::ContextMenu => "context-menu",
            Self::Cell => "cell",
            Self::VerticalText => "vertical-text",
            Self::Alias => "alias",
            Self::Copy => "copy",
            Self::NoDrop => "no-drop",
            Self::Grab => "grab",
            Self::Grabbing => "grabbing",
            Self::AllScroll => "all-scroll",
            Self::ZoomIn => "zoom-in",
            Self::ZoomOut => "zoom-out",
            Self::EResize => "e-resize",
            Self::NResize => "n-resize",
            Self::NeResize => "ne-resize",
            Self::NwResize => "nw-resize",
            Self::SResize => "s-resize",
            Self::SeResize => "se-resize",
            Self::SwResize => "sw-resize",
            Self::WResize => "w-resize",
            Self::EwResize => "ew-resize",
            Self::NsResize => "ns-resize",
            Self::NeswResize => "nesw-resize",
            Self::NwseResize => "nwse-resize",
            Self::ColResize => "col-resize",
            Self::RowResize => "row-resize",
        }
    }
}