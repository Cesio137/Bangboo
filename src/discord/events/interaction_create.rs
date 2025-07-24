use crate::discord::app::base::App;
use serenity::all::{Context, Interaction};

pub async fn run(app: &App, ctx: Context, interaction: Interaction) {
    match interaction {
        Interaction::Ping(_) => {}
        Interaction::Command(command) => {
            if let Some(callback) = app.slash_command_handlers.get(&command.data.name) {
                let command_name = command.data.name.clone();
                callback.run(app, ctx, command).await
            }
        }
        Interaction::Autocomplete(_) => {}
        Interaction::Component(_) => {}
        Interaction::Modal(_) => {}
        _ => {}
    }
}
