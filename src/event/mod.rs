pub mod component_interaction;

use log::{info, warn};
use poise::serenity_prelude::{self as serenity};

use crate::db::race::RaceKind;
use crate::db::MongoDBExt;

use crate::race;
use crate::{Data, Error};

use self::component_interaction::ComponentInteractionEvent;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    if let serenity::FullEvent::Ready { data_about_bot, .. } = event {
        info!("Logged in as {}", data_about_bot.user.name);
    }
    if let serenity::FullEvent::InteractionCreate { interaction, .. } = event {
        if race(
            data,
            RaceKind::InteractionCreateEvent,
            interaction.id().to_string(),
        )
        .await?
        {
            return Ok(());
        }
        let mongo = data.mongo();
        match interaction {
            serenity::Interaction::Component(component_interaction) => {
                let interaction = mongo
                    .interaction_find(&component_interaction.data.custom_id, false)
                    .await?;
                match interaction {
                    Some(interaction) => {
                        ComponentInteractionEvent {
                            ctx,
                            event,
                            db_interaction: interaction,
                            interaction: component_interaction.clone(),
                            data,
                        }
                        .handle()
                        .await?;
                    }
                    None => {
                        warn!(
                            "No handler found for custom_id: {}, interaction: {:?}",
                            component_interaction.data.custom_id, component_interaction
                        );
                    }
                }
            }
            serenity::Interaction::Command(_) => {}
            _ => {
                warn!("Unhandled interaction: {:?}", interaction);
            }
        }
    }
    Ok(())
}
