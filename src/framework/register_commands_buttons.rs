use crate::db::race::race;
use crate::db::race::RaceKind::CommandInvocation;
use crate::{Context, Error};
use poise::command;

#[command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    if race(&ctx, CommandInvocation, ctx.id().to_string()).await? {
        return Ok(());
    };
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
