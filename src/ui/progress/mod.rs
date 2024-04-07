use log::{debug, info};

use crate::controller::{nyaodle, NyaodleState};
use crate::db::threader_configurations::ThreaderConfigurationDoc;
use crate::event::component_interaction::ComponentInteractionEvent;
use crate::threader::debug_threader::DebugThreader;
use crate::Error;

pub async fn execute_progress(
    event: &ComponentInteractionEvent<'_>,
    config: ThreaderConfigurationDoc,
) -> Result<(), Error> {
    let grubber = config.request.create_grabber(event.ctx);
    let threader = DebugThreader;
    let (tx, mut rx) = tokio::sync::mpsc::channel::<NyaodleState>(8);
    tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(state) = rx.recv() => {
                    debug!("progress state={:?}", state);
                    if state.is_completed {
                        info!("progress completed");
                        break;
                    }
                }
            }
        }
    });
    nyaodle(config._id.unwrap().to_hex(), tx, grubber, threader).await?;

    Ok(())
}
