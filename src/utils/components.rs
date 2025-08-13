use crate::discord::*;
use crate::utils::*;
use serenity::all::{
    CacheHttp, ComponentInteraction, Context, CreateInteractionResponse,
    CreateInteractionResponseFollowup, CreateInteractionResponseMessage, EditAttachments,
    EditInteractionResponse, MessageFlags,
};

pub async fn reply_component<'a>(
    ctx: &Context,
    interaction: &ComponentInteraction,
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
        res_message = res_message.files(attachments.clone());
    }

    let result = interaction
        .create_response(ctx.http(), CreateInteractionResponse::Message(res_message))
        .await;

    if let Err(err) = result {
        let interaction_name = &interaction.data.custom_id;
        error(&format!(
            "Error trying to reply component of id \"{}\"!\n└ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}

pub async fn update_component<'a>(
    ctx: &Context,
    interaction: &ComponentInteraction,
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
        res_message = res_message.files(attachments.clone());
    }

    let result = interaction
        .create_response(
            ctx.http(),
            CreateInteractionResponse::UpdateMessage(res_message),
        )
        .await;

    if let Err(err) = result {
        let interaction_name = &interaction.data.custom_id;
        error(&format!(
            "Error trying to reply component of id \"{}\"!\n└ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}

pub async fn followup_component<'a>(
    ctx: &Context,
    interaction: &ComponentInteraction,
    flags: MessageFlags,
    payload: &'a ReplyPayload<'a>,
) -> bool {
    let mut res_followup = CreateInteractionResponseFollowup::new();
    res_followup = res_followup.flags(flags);

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
        res_followup = res_followup.files(attachments.clone());
    }

    let result = interaction.create_followup(ctx.http(), res_followup).await;

    if let Err(err) = result {
        let interaction_name = &interaction.data.custom_id;
        error(&format!(
            "Error trying to follow component of id \"{}\"!\n└ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}

pub async fn edit_component<'a>(
    ctx: &Context,
    interaction: &ComponentInteraction,
    flags: MessageFlags,
    payload: &'a ReplyPayload<'a>,
) -> bool {
    let mut edit_message = EditInteractionResponse::new();
    edit_message = edit_message.flags(flags);

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
        let interaction_name = &interaction.data.custom_id;
        error(&format!(
            "Error trying to edit component of id \"{}\"!\n└ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}
