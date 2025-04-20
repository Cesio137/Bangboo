use crate::discord::app::base::App;
use crate::tools::automod::DangerLevel;
use serenity::all::Message;
use serenity::client::Context;

pub async fn run(app: &App, ctx: Context, message: Message) {    
    let result = app.scam_filter.filter_message(&message.content);
    match result {
        DangerLevel::Safe => {}
        DangerLevel::High => {
            app.scam_filter.handle_spam(ctx.clone(), message.clone()).await;
            return;
        }
    }
    
    if let Some(callback) = app.prefix_commands.get(&message.content) {
        callback(ctx, message).await;
    }
}