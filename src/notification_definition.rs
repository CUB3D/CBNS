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
    #[serde(rename = "targetAppID")]
    pub target_app_id: String,
    //    pub priority: NotificationPrioritory,
    //    pub requiresAcknoqledge: bool,
    //    pub message: PushMessagePayload,
    #[serde(rename = "dataPayload")]
    pub data_payload: Option<Vec<PushMessagePayloadEntry>>,
}
