use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;
use once_cell::sync::Lazy;

const MODEL_GEMINI: &str = "gemini-2.5-flash";

pub static GEMINI: Lazy<Client> = Lazy::new(|| Client::default());

pub async fn get_text(prompt: String) -> anyhow::Result<String> {
    _ = GEMINI.resolve_service_target(MODEL_GEMINI).await?;

    let chat_req = ChatRequest::new(vec![
        ChatMessage::system("Answer in one sentence"),
        ChatMessage::user(prompt),
    ]);

    let chat_res = GEMINI.exec_chat(MODEL_GEMINI, chat_req, None).await?;

    Ok(chat_res
        .content_text_into_string()
        .unwrap_or("NONE".to_string()))
}
