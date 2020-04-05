use serde::{Deserialize, Serialize};

//TODO: rename this to k and v
#[derive(Serialize, Deserialize, Debug)]
pub struct PushMessagePayloadEntry {
    pub key: String,
    pub value: String,
}

//TODO: remove targetappid and rename to dp
#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub targetAppID: String,
    //    pub priority: NotificationPrioritory,
    //    pub requiresAcknoqledge: bool,
    //    pub message: PushMessagePayload,
    pub dataPayload: Option<Vec<PushMessagePayloadEntry>>,
}
