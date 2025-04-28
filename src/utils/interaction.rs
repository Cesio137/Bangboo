use anyhow::Result;
use serenity::all::{CacheHttp, CommandInteraction, Context, CreateAttachment, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, EditInteractionResponse};

pub async fn reply_with_embed(ctx: &Context, interaction: &CommandInteraction, embed: CreateEmbed, is_defered: bool) -> Result<()> {
    if is_defered {
        let interaction_message = CreateInteractionResponseMessage::new()
            .embed(embed);
        interaction.create_response(
            ctx.http(),
            CreateInteractionResponse::Message(interaction_message)
        ).await?;
    } else {
        let followup_message = CreateInteractionResponseFollowup::new()
            .embed(embed);
        interaction.create_followup(
            ctx.http(),
            followup_message
        ).await?;
    }

    Ok(())
}

pub async fn reply_with_attachment(ctx: &Context, interaction: &CommandInteraction, attachment: CreateAttachment, is_defered: bool) -> Result<()> {
    if is_defered {
        let interaction_message = EditInteractionResponse::new()
            .new_attachment(attachment);
        interaction.edit_response(
            ctx.http(),
            interaction_message
        ).await?;
    } else { 
        let interaction_message = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .add_file(attachment)
        );
        interaction.create_response(
            ctx.http(),
            interaction_message
        ).await?;
    }

    Ok(())
}