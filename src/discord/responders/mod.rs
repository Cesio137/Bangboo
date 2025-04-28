mod moderate;

use crate::discord::app::creators::ResponderHandler;

pub fn responders() -> Vec<Box<dyn ResponderHandler + Send + Sync>> {
    let mut responders: Vec<Box<dyn ResponderHandler + Send + Sync>> = Vec::new();

    responders.push(Box::new(moderate::Moderate));
    
   responders
}