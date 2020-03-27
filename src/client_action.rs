use crate::notification_definition::Notification;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientAction {
    pub action_name: String,
    pub target: Option<String>,
    pub notification_payload: Option<Notification>,
}
