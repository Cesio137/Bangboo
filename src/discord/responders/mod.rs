mod discloud;

use crate::discord::app::creators::ResponderHandler;

pub fn responders() -> Vec<Box<dyn ResponderHandler + Send + Sync>> {
    let responders: Vec<Box<dyn ResponderHandler + Send + Sync>> =
        vec![Box::new(discloud::Status), Box::new(discloud::Logs)];

    responders
}
