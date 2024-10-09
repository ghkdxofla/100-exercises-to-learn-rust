use tokio::sync::mpsc as tokio_mpsc;


pub struct Message {
    payload: String,
    response_channel: tokio_mpsc::Sender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: tokio_mpsc::Receiver<Message>) {
    loop {
        if let Ok(msg) = receiver.recv().await.ok_or("") {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = tokio_mpsc::channel(10);
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                }).await.unwrap();
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    use tokio::sync::mpsc as tokio_mpsc;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = tokio_mpsc::channel(10);
        let (response_sender, mut response_receiver) = tokio_mpsc::channel(10);
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .await.unwrap();

        tokio::spawn(pong(receiver));

        let answer = response_receiver.recv().await.unwrap().payload;
        assert_eq!(answer, "pong");
    }
}
