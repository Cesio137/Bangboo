use async_trait::async_trait;
use crate::discord::app::creators::SlashCommandHandler;
use crate::settings::global::EColor;
use crate::utils::{embeds::res, interaction::reply_with_embed};
use serenity::all::{CommandDataOptionValue, CommandInteraction, CommandOptionType, CommandType, Context, CreateCommand, InteractionContext};
use serenity::builder::CreateCommandOption;
use crate::discord::app::base::App;

pub struct Age;

#[async_trait]
impl SlashCommandHandler for Age {
    fn command(&self) -> CreateCommand {
        let option = CreateCommandOption::new(CommandOptionType::User, "user", "Select an user.")
            .required(false);
        CreateCommand::new("age")
            .description("Displays your or another user's account creation date.")
            .kind(CommandType::ChatInput)
            .add_option(option)
            .add_context(InteractionContext::Guild)
    }

    async fn run(&self, app: &App, ctx: Context, interaction: CommandInteraction) {
        let options = &interaction.data.options;
        let user_id = if let Some(option) = options.get(0) {
            if let CommandDataOptionValue::User(ref user_id) = option.value {
                user_id.clone()
            } else { interaction.user.id }
        } else { interaction.user.id };

        let user = match ctx.http.get_user(user_id).await {
            Ok(user) => user,
            Err(_) => {
                let embed = res(EColor::Danger, "An error occured while fetching the user.");
                let _ = reply_with_embed(&ctx, &interaction, embed, false).await;
                return
            }
        };

        let user_name = user.global_name.as_ref().unwrap_or(&user.name);
        let formatted_date = user.created_at().format("%a, %Hh%Mmin, %d/%b/%Y").to_string();

        let content = format!(
            "**{}**'s account was created at {}.",
            user_name,
            formatted_date,
        );

        let embed = res(EColor::Green, &content);
        let _ = reply_with_embed(&ctx, &interaction, embed, false).await;
    }
}