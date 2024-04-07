use log::warn;
use poise::serenity_prelude as serenity;

use crate::framework::interactions::PendingInteraction;
use crate::ui::configure_threader::update_configure_threader;
use crate::Error;

#[derive(Debug, Clone)]
pub struct ComponentInteractionEvent<'a> {
    pub ctx: &'a serenity::Context,
    pub event: &'a serenity::FullEvent,
    pub db_interaction: PendingInteraction,
    pub interaction: serenity::ComponentInteraction,
}

impl ComponentInteractionEvent<'_> {
    pub async fn handle(&self) -> Result<(), Error> {
        match &self.db_interaction {
            PendingInteraction::SelectThreaders { config_id } => {
                update_configure_threader(self).await?;
            }
            _ => {
                warn!(
                    "No handler found for custom_id: {}, interaction: {:?}",
                    self.interaction.data.custom_id, self.interaction
                );
            }
        }
        Ok(())
    }
}
