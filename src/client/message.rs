use crate::config::Input;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: MessageContent,
}

impl Message {
    pub fn new(input: &Input) -> Self {
        Self {
            role: MessageRole::User,
            content: input.to_message_content(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    System,
    Assistant,
    User,
}

#[allow(dead_code)]
impl MessageRole {
    pub fn is_system(&self) -> bool {
        matches!(self, MessageRole::System)
    }

    pub fn is_user(&self) -> bool {
        matches!(self, MessageRole::User)
    }

    pub fn is_assistant(&self) -> bool {
        matches!(self, MessageRole::Assistant)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Array(Vec<MessageContentPart>),
}

impl MessageContent {
    pub fn render_input(&self, resolve_url_fn: impl Fn(&str) -> String) -> String {
        match self {
            MessageContent::Text(text) => text.to_string(),
            MessageContent::Array(list) => {
                let (mut concated_text, mut files) = (String::new(), vec![]);
                for item in list {
                    match item {
                        MessageContentPart::Text { text } => {
                            concated_text = format!("{concated_text} {text}")
                        }
                        MessageContentPart::ImageUrl { image_url } => {
                            files.push(resolve_url_fn(&image_url.url))
                        }
                    }
                }
                if !concated_text.is_empty() {
                    concated_text = format!(" -- {concated_text}")
                }
                format!(".file {}{}", files.join(" "), concated_text)
            }
        }
    }

    pub fn merge_prompt(&mut self, replace_fn: impl Fn(&str) -> String) {
        match self {
            MessageContent::Text(text) => *text = replace_fn(text),
            MessageContent::Array(list) => {
                if list.is_empty() {
                    list.push(MessageContentPart::Text {
                        text: replace_fn(""),
                    })
                } else if let Some(MessageContentPart::Text { text }) = list.get_mut(0) {
                    *text = replace_fn(text)
                }
            }
        }
    }

    pub fn to_text(&self) -> String {
        match self {
            MessageContent::Text(text) => text.to_string(),
            MessageContent::Array(list) => {
                let mut parts = vec![];
                for item in list {
                    if let MessageContentPart::Text { text } = item {
                        parts.push(text.clone())
                    }
                }
                parts.join("\n\n")
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageContentPart {
    Text { text: String },
    ImageUrl { image_url: ImageUrl },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageUrl {
    pub url: String,
}

pub fn patch_system_message(messages: &mut Vec<Message>) {
    if messages[0].role.is_system() {
        let system_message = messages.remove(0);
        if let (Some(message), MessageContent::Text(system_text)) =
            (messages.get_mut(0), system_message.content)
        {
            if let MessageContent::Text(text) = message.content.clone() {
                message.content = MessageContent::Text(format!("{}\n\n{}", system_text, text))
            }
        }
    }
}

pub fn extract_sytem_message(messages: &mut Vec<Message>) -> Option<String> {
    if messages[0].role.is_system() {
        let system_message = messages.remove(0);
        return Some(system_message.content.to_text());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::InputContext;

    #[test]
    fn test_serde() {
        assert_eq!(
            serde_json::to_string(&Message::new(&Input::from_str(
                "Hello World",
                InputContext::default()
            )))
            .unwrap(),
            "{\"role\":\"user\",\"content\":\"Hello World\"}"
        );
    }
}
