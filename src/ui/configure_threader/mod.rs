pub mod component;
pub mod create;
pub mod interactions;

use bson::doc;
use log::{info, warn};
use poise::serenity_prelude::{
    ComponentInteractionDataKind, CreateInteractionResponse, CreateInteractionResponseMessage,
};

use crate::db::MongoDBExt;
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

pub(crate) async fn execute_nyaodle(event: &ComponentInteractionEvent<'_>) -> Result<(), Error> {
    let docs = ConfigureThreaderDocs::from_event(event).await?;
    close_threaders_config(event).await?;
    info!(
        "executed configure_threader UI (execute_nyaodle) id={:?}",
        docs.config._id
    );
    Ok(())
}

pub(crate) async fn close_threaders_config(
    event: &ComponentInteractionEvent<'_>,
) -> Result<(), Error> {
    let docs = ConfigureThreaderDocs::from_event(event).await?;
    let mongo = event.data.mongo();
    event
        .interaction
        .create_response(
            event.ctx,
            CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::default()),
        )
        .await?;
    event.interaction.delete_response(event.ctx).await?;
    mongo
        .interactions
        .delete_many(
            doc! {
                "interaction.type": {
                    "$in": [
                        "select_threaders",
                        "execute_nyaodle",
                        "close_threaders_config",
                        "change_channel_id"
                    ]
                },
                "interaction.config_id": docs.config._id,
            },
            None,
        )
        .await?;
    mongo
        .threader_configurations
        .delete_one(doc! { "_id": docs.config._id }, None)
        .await?;
    info!("closed configure_threader UI id={:?}", docs.config._id);
    Ok(())
}

pub(crate) async fn change_channel_id(event: &ComponentInteractionEvent<'_>) -> Result<(), Error> {
    let mut docs = ConfigureThreaderDocs::from_event(event).await?;
    let channel_id = match &event.interaction.data.kind {
        ComponentInteractionDataKind::ChannelSelect { values: data } => data[0],
        _ => {
            panic!("Unexpected data kind: {:?}", event.interaction.data.kind);
        }
    };
    if !matches!(
        docs.config.configuration,
        ThreaderConfiguration::AnotherChannel { .. }
    ) {
        warn!(
            "change_channel_id called but another threader selected: {:?}",
            docs.config.configuration
        );
    }
    docs.config.configuration = ThreaderConfiguration::AnotherChannel {
        channel_id: Some(channel_id),
    };
    docs.apply(event).await?;
    info!(
        "updated configure_threader UI (change_channel_id) id={:?}",
        docs.config._id
    );
    Ok(())
}
