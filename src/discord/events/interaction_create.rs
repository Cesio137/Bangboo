use crate::discord::app::base::App;
use serenity::all::{Context, Interaction};

pub async fn run(app: &App, ctx: &Context, interaction: &Interaction) {
    match interaction {
        Interaction::Ping(_) => {}
        Interaction::Command(command) => {
            if let Some(callback) = app.slash_command_handlers.get(command.data.name.as_str()) {
                callback.run(app, ctx, command).await
            }
        }
        Interaction::Autocomplete(_) => {}
        Interaction::Component(component) => {
            if let Some(callback) = app.responder_handlers.get(component.data.custom_id.as_str()) {
                callback.run(ctx, component).await
            }
        }
        Interaction::Modal(_) => {}
        _ => {}
    }
}
