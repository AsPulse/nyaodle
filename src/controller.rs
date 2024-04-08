use log::{debug, info, warn};
use tokio::sync::mpsc;

use crate::grubber::{Grubber, GrubberMessage};
use crate::threader::{MessageBulk, Threader, ThreaderMessage};
use crate::Error;

#[derive(Debug, Clone)]
pub struct NyaodleState {
    pub num_total_messages: u64,
    pub num_grubbed_messages: u64,
    pub num_threaded_messages: u64,
    pub is_grubbed: bool,
    pub is_threaded: bool,
    pub is_completed: bool,
}

pub async fn nyaodle(
    id: String,
    tx: mpsc::Sender<NyaodleState>,
    grubber: impl Grubber,
    threader: impl Threader,
) -> Result<(), Error> {
    let (tx_grubber, mut rx_grubber) = mpsc::channel::<GrubberMessage>(4);
    let (tx_threader, mut rx_threader) = mpsc::channel::<ThreaderMessage>(4);
    let (tx_threader_message, rx_threader_message) = mpsc::channel::<MessageBulk>(32);

    let mut state = NyaodleState {
        num_total_messages: 0,
        num_grubbed_messages: 0,
        num_threaded_messages: 0,
        is_grubbed: false,
        is_threaded: false,
        is_completed: false,
    };

    grubber.grub(&id, tx_grubber).await?;
    threader
        .thread(&id, tx_threader, rx_threader_message)
        .await?;

    tokio::spawn(async move {
        info!("New nyaodle controller started with id={}", id);
        loop {
            tokio::select! {
                Some(message) = rx_grubber.recv() => {
                    match message {
                        GrubberMessage::StateUpdate(state_update) => {
                            state.num_total_messages = state_update.num_total_messages;
                            state.num_grubbed_messages = state_update.num_grubbed_messages;
                            state.is_grubbed = state_update.is_completed;
                            let finished = update_state(&id, &mut state);
                            tx.send(state.clone()).await.unwrap();
                            if finished {
                                warn!("nyaodle controller finished with grubber message id={}", id);
                                break;
                            }
                        }
                        GrubberMessage::MessageTranfer(messages) => {
                            for message in messages {
                                tx_threader_message.send(message).await.unwrap();
                            }
                        }
                    }
                }
                Some(message) = rx_threader.recv() => {
                    match message {
                        ThreaderMessage::StateUpdate(state_update) => {
                            state.num_threaded_messages = state_update.num_threaded_messages;
                            state.is_threaded = state_update.is_completed;
                            let finished = update_state(&id, &mut state);
                            tx.send(state.clone()).await.unwrap();
                            if finished {
                                info!("nyaodle controller finished id={}", id);
                                break;
                            }
                        }
                    }
                }
            }
        }
        state.is_completed = true;
        tx.send(state.clone()).await.unwrap();
    });

    Ok(())
}
fn update_state(id: &str, new_state: &mut NyaodleState) -> bool {
    debug!("nyaodle state updated id={} state={:?}", id, new_state);
    new_state.is_grubbed && new_state.is_threaded
}
