pub mod component;
pub mod create;
pub mod interactions;

use log::info;
use poise::serenity_prelude::ComponentInteractionDataKind;

use crate::event::component_interaction::ComponentInteractionEvent;
use crate::threader::ThreaderConfiguration;
use crate::Error;

use self::component::configure_threader_component;
use self::interactions::ConfigureThreaderDocs;

pub(crate) async fn update_threader_selection(
    event: &ComponentInteractionEvent<'_>,
) -> Result<(), Error> {
    let selected_type = match &event.interaction.data.kind {
        ComponentInteractionDataKind::StringSelect { values: data } => data[0].as_str(),
        _ => {
            panic!("Unexpected data kind: {:?}", event.interaction.data.kind);
        }
    };
    let mut docs = ConfigureThreaderDocs::from_event(event).await?;

    docs.config.configuration = if selected_type == "another_channel" {
        ThreaderConfiguration::AnotherChannel { channel_id: None }
    } else if selected_type == "new_thread" {
        ThreaderConfiguration::NewThread {}
    } else if selected_type == "new_forum_post" {
        ThreaderConfiguration::NewForumPost {}
    } else {
        panic!("Unexpected selected_type: {}", selected_type);
    };

    docs.apply(event).await?;
    info!(
        "updated configure_threader UI (threader_selection) id={:?}",
        docs.config._id
    );
    Ok(())
}
