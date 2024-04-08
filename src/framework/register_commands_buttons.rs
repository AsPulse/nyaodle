use crate::db::race::race;
use crate::db::race::RaceKind::CommandInvocation;
use crate::framework::ADMIN_GUILD_ID;
use crate::{Context, Error};
use log::warn;
use poise::command;

#[command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    if race(&ctx, CommandInvocation, ctx.id().to_string()).await? {
        return Ok(());
    };
    if ctx.guild_id() != Some(*ADMIN_GUILD_ID) {
        warn!("ignoring register command from non-admin guild.");
        return Ok(());
    }
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
