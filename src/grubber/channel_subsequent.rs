//! あるチャンネルに投稿された、このメッセージ以降のメッセージを対象にする

use crate::db::race::race;
use crate::db::race::RaceKind;
use crate::ui::configure_threader::configure_threader;
use poise::command;
use poise::serenity_prelude as serenity;

use crate::ApplicationContext;
use crate::Error;

#[command(context_menu_command = "これ以降のメッセージを移動")]
pub async fn move_channel_subsequent(
    ctx: ApplicationContext<'_>,
    _message: serenity::Message,
) -> Result<(), Error> {
    if race(
        &ctx,
        RaceKind::ReceiveInteraction,
        ctx.interaction.id.to_string(),
    )
    .await?
    {
        return Ok(());
    }
    configure_threader(ctx).await?;
    Ok(())
}
