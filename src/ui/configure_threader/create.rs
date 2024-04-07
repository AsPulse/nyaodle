use log::info;
use poise::CreateReply;

use crate::{ApplicationContext, Error};

use super::configure_threader_component;
use super::interactions::ConfigureThreaderDocs;

pub(crate) async fn configure_threader(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let docs = ConfigureThreaderDocs::create_and_insert(&ctx).await?;

    info!("new configure_threader UI id={:?}", docs.config._id);

    ctx.send(
        CreateReply::default()
            .components(configure_threader_component(&docs))
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
