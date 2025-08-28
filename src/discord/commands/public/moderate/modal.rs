use serenity::all::{
    CacheHttp, ComponentInteraction, Context, CreateInputText, CreateInteractionResponse,
    CreateQuickModal, InputTextStyle, QuickModal,
};
use std::time::{Duration, SystemTime};
use crate::discord::base::error;

pub async fn show_modal(
    ctx: &Context,
    interaction: &ComponentInteraction,
    timer: u64,
) -> (bool, String) {
    let field = CreateInputText::new(InputTextStyle::Short, "Reason", "mod/modal")
        .max_length(300)
        .min_length(0)
        .placeholder("Visible only in auditlogs");

    let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(err) => {
            error(&format!("Time went backwards!?\n└ {:?}", err));
            return (false, String::new());
        }
    };

    if now >= timer {
        return (false, String::new());
    }

    let timeout = timer - now;

    let modal = CreateQuickModal::new("What's the reason?")
        .field(field)
        .timeout(Duration::from_secs(timeout));

    match interaction.quick_modal(ctx, modal).await {
        Ok(response) => {
            if let Some(res) = response {
                _ = res
                    .interaction
                    .create_response(ctx.http(), CreateInteractionResponse::Acknowledge)
                    .await;

                return (true, res.inputs[0].to_string());
            }
            (false, String::new())
        }
        Err(err) => {
            error(&format!("Quick modal failed!\n└ {:?}", err));
            (false, String::new())
        }
    }
}
