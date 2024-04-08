use log::info;
use tokio::sync::mpsc;

use crate::Error;

use super::{MessageBulk, Threader, ThreaderMessage, ThreaderState};
use poise::serenity_prelude as serenity;

pub struct DebugThreader<'a> {
    pub ctx: &'a serenity::Context,
}

impl Threader for DebugThreader<'_> {
    async fn thread(
        &self,
        id: &str,
        tx: mpsc::Sender<ThreaderMessage>,
        mut rx: mpsc::Receiver<MessageBulk>,
    ) -> Result<(), Error> {
        let id = id.to_string();
        tokio::spawn(async move {
            let mut state = ThreaderState {
                num_threaded_messages: 0,
                is_completed: false,
            };
            loop {
                tokio::select! {
                    Some(message) = rx.recv() => {
                        match message {
                            MessageBulk::Continue(message) => {
                                info!("debug_threader received message id={} content={}", id, message.content);
                                state.num_threaded_messages += 1;
                                tx.send(ThreaderMessage::StateUpdate(state.clone()))
                                    .await
                                    .unwrap();
                            }
                            MessageBulk::End => {
                                state.is_completed = true;
                                tx.send(ThreaderMessage::StateUpdate(state.clone()))
                                    .await
                                    .unwrap();
                                break;
                            }
                        }
                    }
                }
            }
        });
        Ok(())
    }
}
