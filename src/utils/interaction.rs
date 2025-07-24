use crate::data::settings::EColors;
use crate::settings::logger::error;
use serenity::all::{
    CacheHttp, CommandInteraction, Context, CreateAttachment, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage
    ,
};

pub async fn reply(
    ctx: &Context,
    interaction: &CommandInteraction,
    content: Option<&str>,
    embeds: Option<Vec<CreateEmbed>>,
    attachments: Option<Vec<CreateAttachment>>,
    is_defered: bool,
) -> bool {
    if is_defered {
        let mut res_message = CreateInteractionResponseMessage::new();
        if let Some(content) = content {
            res_message = res_message.content(content);
        }
        if let Some(embeds) = embeds {
            res_message = res_message.embeds(embeds);
        }
        if let Some(attachments) = attachments {
            res_message = res_message.add_files(attachments)
        }

        let result = interaction
            .create_response(ctx.http(), CreateInteractionResponse::Message(res_message))
            .await;

        if let Err(err) = result {
            let interaction_name = &interaction.data.name;
            error(&format!(
                "Error trying to responde {} command interaction\nʟ {:?}",
                interaction_name, err
            ));
            return false;
        }
    } else {
        let mut res_followup = CreateInteractionResponseFollowup::new();
        if let Some(content) = content {
            res_followup = res_followup.content(content);
        }
        if let Some(embeds) = embeds {
            res_followup = res_followup.embeds(embeds);
        }
        if let Some(attachments) = attachments {
            res_followup = res_followup.add_files(attachments)
        }

        let result = interaction.create_followup(ctx.http(), res_followup).await;

        if let Err(err) = result {
            let interaction_name = &interaction.data.name;
            error(&format!(
                "Error trying to responde \"{}\" command interaction!\nʟ {:?}",
                interaction_name, err
            ));
            return false;
        }
    }
    true
}

pub async fn reply_with_embed(
    ctx: &Context,
    interaction: &CommandInteraction,
    color: EColors,
    content: &str,
) -> bool {
    let embed = CreateEmbed::new().color(color as u32).description(content);

    let res_message = CreateInteractionResponseMessage::new().embed(embed);

    let result = interaction
        .create_response(ctx.http(), CreateInteractionResponse::Message(res_message))
        .await;

    if let Err(err) = result {
        let interaction_name = &interaction.data.name;
        error(&format!(
            "Error trying to responde \"{}\" command interaction!\nʟ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}
