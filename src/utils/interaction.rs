use anyhow::Result;
use serenity::all::{CommandInteraction, Context, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::builder::CreateAttachment;

pub async fn defer_reply(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    ctx.http.create_interaction_response(
        interaction.id,
        &interaction.token,
        &CreateInteractionResponse::Defer(
            CreateInteractionResponseMessage::new()
        ),
        vec![]
    ).await?;

    Ok(())
}

pub async fn reply_with_embed(ctx: &Context, interaction: &CommandInteraction, embed: CreateEmbed, is_defered: bool) -> Result<()> {
    if is_defered {
        let interaction_message = CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new().embed(embed)
        );
        ctx.http.edit_original_interaction_response(
            &interaction.token,
            &interaction_message,
            vec![]
        ).await?;
    } else {
        let interaction_message = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().embed(embed)
        );
        ctx.http.create_interaction_response(
            interaction.id,
            &interaction.token,
            &interaction_message,
            vec![]
        ).await?;
    }

    Ok(())
}

pub async fn reply_with_attachment(ctx: &Context, interaction: &CommandInteraction, attachment: CreateAttachment, is_defered: bool) -> Result<()> {
    if is_defered {
        let interaction_message = CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new()
        );
        ctx.http.edit_original_interaction_response(
            &interaction.token,
            &interaction_message,
            vec![attachment]
        ).await?;
    } else { 
        let interaction_message = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
        );
        ctx.http.create_interaction_response(
            interaction.id,
            &interaction.token,
            &interaction_message,
            vec![attachment]
        ).await?;
    }

    Ok(())
}