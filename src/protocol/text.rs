use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextComponentScoreboard {
    pub name: String,
    pub objective: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextComponentNbtSource {
    Block,
    Entity,
    Storage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TextComponentKind {
    Text {
        text: String,
    },
    Translatable {
        translate: String,
        fallback: String,
        with: Vec<TextComponent>,
    },
    #[serde(rename = "score")]
    Scoreboard {
        score: TextComponentScoreboard,
    },
    Selector {
        selector: String,
        separator: Box<TextComponent>,
    },
    Keybind {
        keybind: String,
    },
    NBT {
        source: TextComponentNbtSource,
        nbt: String,
        interpret: bool,
        separator: Box<TextComponent>,
        block: String,
        entity: String,
        storage: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamedColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Named(NamedColor),
    Hex(String), // TODO: proper HEX validation
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextComponentClientEventAction {
    OpenUrl,
    OpenFile,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
    ShowDialog,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextComponentClickEvent {
    pub action: TextComponentClientEventAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextComponentHoverEventAction {
    ShowText,
    ShowItem,
    ShowEntity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextComponentHoverEvent {
    pub action: TextComponentHoverEventAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<TextComponent>>, // TODO: support for other types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    // TODO: implement
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub components: Option<Box<TextComponent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<TextComponent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>, // TODO: UUID
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextComponent {
    #[serde(flatten)]
    pub kind: TextComponentKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<TextComponent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>, // TODO: namespaces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_color: Option<u32>, // TODO: proper typing / utilities set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_event: Option<TextComponentClickEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<TextComponentHoverEvent>,
}

#[cfg(test)]
mod tests {
    use crate::protocol::text::TextComponent;

    #[test]
    fn test_component() {
        let text_component = TextComponent {
            kind: super::TextComponentKind::Text {
                text: "Hello, World!".to_owned(),
            },
            extra: Some(vec![TextComponent {
                kind: super::TextComponentKind::Text {
                    text: "This is Kasumi".to_owned(),
                },
                extra: None,
                color: Some(super::Color::Named(super::NamedColor::Gold)),
                font: None,
                bold: None,
                italic: Some(true),
                underlined: Some(true),
                strikethrough: None,
                obfuscated: None,
                shadow_color: None,
                insertion: None,
                click_event: None,
                hover_event: None,
            }]),
            color: Some(super::Color::Named(super::NamedColor::DarkGreen)),
            font: None,
            bold: Some(true),
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            shadow_color: None,
            insertion: None,
            click_event: None,
            hover_event: None,
        };
        println!(
            "{:#?}",
            serde_json::to_string_pretty(&text_component).unwrap()
        )
    }
}
