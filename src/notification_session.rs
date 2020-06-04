use crate::client_action::ClientAction;

use crate::messages::{
    ChannelNotificationMsg, ConnectMsg, DeviceNotificationMsg, DeviceSubscribeMsg,
    DeviceUnsubscribeMsg, DisconnectMsg, PushedMsg,
};
use crate::notification_server::NotificationServer;

use actix::*;
use actix_web_actors::ws;

use actix_web_actors::ws::Message::Text;
use std::time::Duration;

pub struct WSNotificationSession {
    pub server_address: Addr<NotificationServer>,
    // The id of this client
    pub uid: usize,
    pub token: String,
}

//TODO: const DEVICE_STATUS_UPDATE_INTERVAL: Duration = Duration::from_secs(8 * 60);
//TODO: test different durations for this
// Set the delay between keep-alive pings
const DEVICE_PING_INTERVAL: Duration = Duration::from_secs(10);

use futures::executor::block_on;

impl Actor for WSNotificationSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Start the device notifier
        //        ctx.run_interval(DEVICE_STATUS_UPDATE_INTERVAL, |act, ctx | {
        //            ctx.text("device_update_status");
        //        });

        ctx.run_interval(DEVICE_PING_INTERVAL, |_act, ctx| {
            debug!("Sending ping");
            //            ctx.text("");
            ctx.ping(b"");
        });

        let addr = ctx.address();
        // unimplemented!();

        let x = block_on(
            self.server_address.send(ConnectMsg {
                addr: addr.recipient(),
                token: self.token.clone(),
            }), // .into_actor(self)
                // .then(|res, act, ctx| {
                //     match res {
                //         Ok(uid) => act.uid = uid,
                //         _ => ctx.stop(),
                //     }
                //     fut::ok(())
                // })
        );

        match x {
            Ok(uid) => self.uid = uid,
            _ => ctx.stop(),
        }
        // .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        println!("Websocket DC");

        //TODO: error handling
        block_on(self.server_address.send(DisconnectMsg { uid: self.uid })).unwrap();

        Running::Stop
    }
}

impl Handler<PushedMsg> for WSNotificationSession {
    type Result = ();

    fn handle(&mut self, msg: PushedMsg, ctx: &mut Self::Context) {
        ctx.text(msg.message);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WSNotificationSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        if let Ok(item) = item {
            if let Text(msg) = item {
                println!("Got text message from client '{}': {}", self.token, &msg);

                let client_message = serde_json::from_str::<ClientAction>(msg.as_str());

                if let Ok(action) = client_message {
                    match action.action_name.as_str() {
                        "BROADCAST_DEVICE" => {
                            let target = action.target.expect("No target given");

                            println!("Sending broadcast to '{}'", target);
                            self.server_address.do_send(DeviceNotificationMsg {
                                device_token: target,
                                message: serde_json::to_string(
                                    &action
                                        .notification_payload
                                        .expect("No notification payload given"),
                                )
                                .unwrap(),
                            })
                        }
                        "BROADCAST_CHANNEL" => {
                            let target = action.target.expect("No target given");

                            println!("Sending broadcast to channel '{}'", target);
                            self.server_address.do_send(ChannelNotificationMsg {
                                channel: target,
                                message: serde_json::to_string(
                                    &action
                                        .notification_payload
                                        .expect("No notification payload given"),
                                )
                                .unwrap(),
                            })
                        }
                        "SUBSCRIBE" => {
                            let target = action.target.expect("No target given");

                            println!(
                                "Subscribing client '{}' to channel '{}'",
                                self.token, target
                            );

                            self.server_address.do_send(DeviceSubscribeMsg {
                                device_token: self.token.clone(),
                                channel: target,
                            })
                        }
                        "UNSUBSCRIBE" => {
                            let target = action.target.expect("No target given");

                            println!(
                                "Unsubscribing client '{}' from channel '{}'",
                                self.token, target
                            );

                            self.server_address.do_send(DeviceUnsubscribeMsg {
                                device_token: self.token.clone(),
                                channel: target,
                            })
                        }

                        _ => {
                            eprintln!("Got unknown action: {}", action.action_name);
                        }
                    }
                } else {
                    let e = client_message.err().unwrap();
                    eprintln!("Unable to parse message: {}", e);
                }
            } else {
                println!(
                    "Got unknown message from client '{}': '{:?}'",
                    self.token, &item
                );
            }
        }
    }
}
