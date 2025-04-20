use crate::discord::app::base::App;
use serenity::all::{Context, Interaction};

pub async fn run(app: &App, ctx: Context, interaction: Interaction) {
    match interaction {
        Interaction::Ping(_) => {}
        Interaction::Command(command) => {
            if let Some(callback) = app.slash_commands.get(&command.data.name) {
                callback(ctx, command).await;
            }
        }
        Interaction::Autocomplete(_) => {}
        Interaction::Component(_) => {}
        Interaction::Modal(_) => {}
        _ => {}
    }
}