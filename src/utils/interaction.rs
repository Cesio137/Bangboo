use crate::data::settings::EColors;
use crate::settings::logger::error;
use serenity::all::{
    CacheHttp, CommandInteraction, Context, CreateAttachment, CreateComponent, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage,
    EditAttachments, EditInteractionResponse, MessageFlags,
};

#[derive(Debug, Clone)]
pub struct ReplyPayload<'a> {
    pub content: Option<&'a str>,
    pub embeds: Option<Vec<CreateEmbed<'a>>>,
    pub components: Option<Vec<CreateComponent<'a>>>,
    pub attachments: Option<Vec<CreateAttachment<'a>>>,
}

impl<'a> Default for ReplyPayload<'a> {
    fn default() -> Self {
        ReplyPayload {
            content: None,
            embeds: None,
            components: None,
            attachments: None,
        }
    }
}

pub async fn reply<'a>(
    ctx: &Context,
    interaction: &CommandInteraction,
    flags: MessageFlags,
    payload: &'a ReplyPayload<'a>,
) -> bool {
    let mut res_message = CreateInteractionResponseMessage::new();
    res_message = res_message.flags(flags);

    if let Some(content) = payload.content {
        res_message = res_message.content(content);
    }
    if let Some(embeds) = &payload.embeds {
        res_message = res_message.embeds(embeds);
    }
    if let Some(components) = &payload.components {
        res_message = res_message.components(components);
    }
    if let Some(attachments) = &payload.attachments {
        res_message = res_message.add_files(attachments.to_vec())
    }

    let result = interaction
        .create_response(ctx.http(), CreateInteractionResponse::Message(res_message))
        .await;

    if let Err(err) = result {
        let interaction_name = &interaction.data.name;
        error(&format!(
            "Error trying to responde {} command interaction\n└ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}

pub async fn update<'a>(
    ctx: &Context,
    interaction: &CommandInteraction,
    flags: MessageFlags,
    payload: &'a ReplyPayload<'a>,
) -> bool {
    let mut res_message = CreateInteractionResponseMessage::new();
    res_message = res_message.flags(flags);

    if let Some(content) = payload.content {
        res_message = res_message.content(content);
    }
    if let Some(embeds) = &payload.embeds {
        res_message = res_message.embeds(embeds);
    }
    if let Some(components) = &payload.components {
        res_message = res_message.components(components);
    }
    if let Some(attachments) = &payload.attachments {
        res_message = res_message.add_files(attachments.to_vec())
    }

    let result = interaction
        .create_response(
            ctx.http(),
            CreateInteractionResponse::UpdateMessage(res_message),
        )
        .await;

    if let Err(err) = result {
        let interaction_name = &interaction.data.name;
        error(&format!(
            "Error trying to responde {} command interaction\n└ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}

pub async fn followup<'a>(
    ctx: &Context,
    interaction: &CommandInteraction,
    flags: MessageFlags,
    payload: &'a ReplyPayload<'a>,
) -> bool {
    let mut res_followup = CreateInteractionResponseFollowup::new();
    res_followup = res_followup.flags(flags);

    let payload = payload;
    if let Some(content) = payload.content {
        res_followup = res_followup.content(content);
    }
    if let Some(embeds) = &payload.embeds {
        res_followup = res_followup.embeds(embeds);
    }
    if let Some(components) = &payload.components {
        res_followup = res_followup.components(components);
    }
    if let Some(attachments) = &payload.attachments {
        res_followup = res_followup.add_files(attachments.to_vec())
    }

    let result = interaction.create_followup(ctx.http(), res_followup).await;

    if let Err(err) = result {
        let interaction_name = &interaction.data.name;
        error(&format!(
            "Error trying to followup {} command interaction\n└ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}

pub async fn edit<'a>(
    ctx: &Context,
    interaction: &CommandInteraction,
    flags: MessageFlags,
    payload: &'a ReplyPayload<'a>,
) -> bool {
    let mut edit_message = EditInteractionResponse::new();
    edit_message = edit_message.flags(flags);

    let payload = payload;
    if let Some(content) = payload.content {
        edit_message = edit_message.content(content);
    }
    if let Some(embeds) = &payload.embeds {
        edit_message = edit_message.embeds(embeds);
    }
    if let Some(components) = &payload.components {
        edit_message = edit_message.components(components);
    }
    if let Some(attachments) = &payload.attachments {
        let mut edit_attachments = EditAttachments::new();
        for attachment in attachments {
            edit_attachments = edit_attachments.add(attachment.clone());
        }
        edit_message = edit_message.attachments(edit_attachments)
    }

    let result = interaction.edit_response(ctx.http(), edit_message).await;

    if let Err(err) = result {
        let interaction_name = &interaction.data.name;
        error(&format!(
            "Error trying to followup {} command interaction\n└ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}

pub async fn reply_with_embed(
    ctx: &Context,
    interaction: &CommandInteraction,
    flags: MessageFlags,
    color: EColors,
    content: &str,
) -> bool {
    let embed = CreateEmbed::new().color(color as u32).description(content);

    let res_message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .flags(flags);
    let result = interaction
        .create_response(ctx.http(), CreateInteractionResponse::Message(res_message))
        .await;

    if let Err(err) = result {
        let interaction_name = &interaction.data.name;
        error(&format!(
            "Error trying to responde \"{}\" command interaction!\n└ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}

pub async fn update_with_embed(
    ctx: &Context,
    interaction: &CommandInteraction,
    flags: MessageFlags,
    color: EColors,
    content: &str,
) -> bool {
    let embed = CreateEmbed::new().color(color as u32).description(content);

    let res_message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .flags(flags);

    let result = interaction
        .create_response(
            ctx.http(),
            CreateInteractionResponse::UpdateMessage(res_message),
        )
        .await;

    if let Err(err) = result {
        let interaction_name = &interaction.data.name;
        error(&format!(
            "Error trying to responde \"{}\" command interaction!\n└ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}

pub async fn followup_with_embed(
    ctx: &Context,
    interaction: &CommandInteraction,
    flags: MessageFlags,
    color: EColors,
    content: &str,
) -> bool {
    let embed = CreateEmbed::new().color(color as u32).description(content);

    let res_followup = CreateInteractionResponseFollowup::new()
        .embed(embed)
        .flags(flags);

    let result = interaction.create_followup(ctx.http(), res_followup).await;

    if let Err(err) = result {
        let interaction_name = &interaction.data.name;
        error(&format!(
            "Error trying to followup \"{}\" command interaction!\n└ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}
