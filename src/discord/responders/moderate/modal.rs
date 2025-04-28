use std::time::Duration;
use serenity::all::{CacheHttp, ComponentInteraction, Context, CreateInteractionResponse, CreateQuickModal};

pub fn create_motive_modal(title: &str, label: &str) -> CreateQuickModal {
    let modal = CreateQuickModal::new(title)
        .paragraph_field(label)
        .timeout(Duration::from_secs(120));

    modal
}

pub async fn modal_panel(ctx: &Context, interaction: &ComponentInteraction) -> String {
    let modal = create_motive_modal("What's the reason", "Reason");

    let motive = interaction.quick_modal(ctx, modal).await;

    if let Ok(response) = motive {
        if let Some(res) = response {
            let _ = res.interaction.create_response(ctx.http(), CreateInteractionResponse::Acknowledge).await;
            return res.inputs[0].clone();
        }
    }
    "".to_string()
}