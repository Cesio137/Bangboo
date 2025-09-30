use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;
use once_cell::sync::Lazy;

const MODEL_GEMINI: &str = "gemini-2.5-flash";

pub static GEMINI: Lazy<Client> = Lazy::new(|| Client::default());

pub async fn get_text(prompt: String) -> anyhow::Result<String> {
    _ = GEMINI.resolve_service_target(MODEL_GEMINI).await?;

    let chat_req = ChatRequest::new(vec![
        //ChatMessage::system(""),
        ChatMessage::user(prompt),
    ]);

    let chat_res = GEMINI.exec_chat(MODEL_GEMINI, chat_req, None).await?;

    let texts = &chat_res.into_texts();
    Ok(texts.join("\n"))
}
