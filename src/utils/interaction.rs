use crate::data::settings::EColors;
use crate::settings::logger::error;
use serenity::all::{CacheHttp, CommandInteraction, ComponentInteraction, Context, CreateAttachment, CreateComponent, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, EditInteractionResponse};

pub async fn reply<'a>(
    ctx: &Context,
    interaction: &CommandInteraction,
    ephemeral: bool,
    is_defered: bool,
    content: Option<&'a str>,
    embeds: Option<Vec<CreateEmbed<'a>>>,
    components: Option<Vec<CreateComponent<'a>>>,
    attachments: Option<Vec<CreateAttachment<'a>>>
) -> bool {
    if !is_defered {
        let mut res_message = CreateInteractionResponseMessage::new()
            .ephemeral(ephemeral);
        if let Some(content) = content {
            res_message = res_message.content(content);
        }
        if let Some(embeds) = embeds {
            res_message = res_message.embeds(embeds);
        }
        if let Some(components) = components {
            res_message = res_message.components(components);
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
                "Error trying to responde {} command interaction\n└ {:?}",
                interaction_name, err
            ));
            return false;
        }
    } else {
        let mut res_followup = CreateInteractionResponseFollowup::new()
            .ephemeral(!ephemeral);
        if let Some(content) = content {
            res_followup = res_followup.content(content);
        }
        if let Some(embeds) = embeds {
            res_followup = res_followup.embeds(embeds);
        }
        if let Some(components) = components {
            res_followup = res_followup.components(components);
        }
        if let Some(attachments) = attachments {
            res_followup = res_followup.add_files(attachments)
        }

        let result = interaction.create_followup(ctx.http(), res_followup).await;

        if let Err(err) = result {
            let interaction_name = &interaction.data.name;
            error(&format!(
                "Error trying to responde \"{}\" command interaction!\n└ {:?}",
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
    ephemeral: bool,
    color: EColors,
    content: &str,
) -> bool {
    let embed = CreateEmbed::new().color(color as u32).description(content);

    let res_message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(ephemeral);
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

pub async fn edit_reply<'a>(
    ctx: &Context,
    interaction: &CommandInteraction,
    content: Option<&'a str>,
    embeds: Option<Vec<CreateEmbed<'a>>>,
    components: Option<Vec<CreateComponent<'a>>>,
) -> bool {
    let mut edit_interaction = EditInteractionResponse::new();
    if let Some(content) = content {
        edit_interaction = edit_interaction.content(content);
    }
    if let Some(embeds) = embeds {
        edit_interaction = edit_interaction.embeds(embeds);
    }
    if let Some(components) = components {
        edit_interaction = edit_interaction.components(components);
    }

    let result = interaction
        .edit_response(ctx.http(), edit_interaction)
        .await;

    if let Err(err) = result {
        let interaction_name = &interaction.data.name;
        error(&format!(
            "Error trying to edit \"{}\" command interaction!\n└ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}

pub async fn reply_component<'a>(
    ctx: &Context,
    interaction: &ComponentInteraction,
    update: bool,
    content: Option<&'a str>,
    embeds: Option<Vec<CreateEmbed<'a>>>,
    components: Option<Vec<CreateComponent<'a>>>
) -> bool {
    let mut res_interaction = CreateInteractionResponseMessage::new();
    if let Some(content) = content {
        res_interaction = res_interaction.content(content);
    }
    if let Some(embeds) = embeds {
        res_interaction = res_interaction.embeds(embeds);
    }
    if let Some(components) = components {
        res_interaction = res_interaction.components(components);
    }
    
    
    let result = if update {
        interaction
            .create_response(ctx.http(), CreateInteractionResponse::UpdateMessage(res_interaction))
            .await
    } else {
        interaction
            .create_response(ctx.http(), CreateInteractionResponse::Message(res_interaction))
            .await
    };

    if let Err(err) = result {
        let interaction_name = &interaction.data.custom_id;
        error(&format!(
            "Error trying to edit response triggered by \"{}\"!\n└ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}

pub async fn edit_component_reply<'a>(
    ctx: &Context,
    interaction: &ComponentInteraction,
    content: Option<&'a str>,
    embeds: Option<Vec<CreateEmbed<'a>>>,
    components: Option<Vec<CreateComponent<'a>>>,
) -> bool {
    let mut edit_interaction = EditInteractionResponse::new();
    if let Some(content) = content {
        edit_interaction = edit_interaction.content(content);
    }
    if let Some(embeds) = embeds {
        edit_interaction = edit_interaction.embeds(embeds);
    }
    if let Some(components) = components {
        edit_interaction = edit_interaction.components(components);
    }

    let result = interaction
        .edit_response(ctx.http(), edit_interaction)
        .await;

    if let Err(err) = result {
        let interaction_name = &interaction.data.custom_id;
        error(&format!(
            "Error trying to edit response triggered by \"{}\"!\n└ {:?}",
            interaction_name, err
        ));
        return false;
    }

    true
}
