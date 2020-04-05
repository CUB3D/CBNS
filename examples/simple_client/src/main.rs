use std::{env, io};

use futures_util::{future, pin_mut, StreamExt, TryFutureExt, AsyncWriteExt, SinkExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt as AWE};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use rand::Rng;
use std::thread::spawn;


#[tokio::main]
async fn main() {
    // Generate a unique id for this client
    let id: i32  = rand::thread_rng().gen_range(0, 100);

    // Generate the connection url
    let url = url::Url::parse(format!("ws://127.0.0.1:8080/poll/{}", id).as_str()).unwrap();

    // Connect to the server
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let (mut write, read) = ws_stream.split();

    let reader = read.for_each(|message| async {
        let data = message.unwrap().into_data();
        tokio::io::stdout().write_all(&data).await.unwrap();
    });

    let writer = async {
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    write.send(Message::Text(format!("{{\"action_name\": \"BROADCAST_CHANNEL\", \"target\": \"device_common\", \"notification_payload\": {{\"targetAppID\": \"\", \"dataPayload\": [{{\"key\": \"message\", \"value\": \"{}\"}}] }} }}", input.trim()))).await;
                }
                Err(error) => println!("error: {}", error),
            }
        }
    };

    pin_mut!(writer, reader);
    future::select(writer, reader).await;
}
