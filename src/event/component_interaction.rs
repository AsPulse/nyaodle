use log::warn;
use poise::serenity_prelude as serenity;

use crate::framework::interactions::PendingInteraction;
use crate::ui::configure_threader::{
    close_threaders_config, execute_nyaodle, update_threader_selection,
};
use crate::{Data, Error};

#[derive(Clone)]
pub struct ComponentInteractionEvent<'a> {
    pub ctx: &'a serenity::Context,
    pub event: &'a serenity::FullEvent,
    pub db_interaction: PendingInteraction,
    pub interaction: serenity::ComponentInteraction,
    pub data: &'a Data,
}

impl ComponentInteractionEvent<'_> {
    pub async fn handle(&self) -> Result<(), Error> {
        match &self.db_interaction {
            PendingInteraction::SelectThreaders { .. } => {
                update_threader_selection(self).await?;
            }
            PendingInteraction::ExecuteNyaodle { .. } => {
                execute_nyaodle(self).await?;
            }
            PendingInteraction::CloseThreadersConfig { .. } => {
                close_threaders_config(self).await?;
            }
            #[allow(unreachable_patterns)]
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
