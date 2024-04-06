use poise::serenity_prelude::{
    CreateActionRow, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
};
use poise::CreateReply;

use crate::{ApplicationContext, Error};

pub(crate) async fn configure_threader(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let select_threader = CreateActionRow::SelectMenu(CreateSelectMenu::new(
        "threader",
        CreateSelectMenuKind::String {
            options: vec![
                CreateSelectMenuOption::new("別のチャンネルに移動", "another_channel")
                    .default_selection(true),
                CreateSelectMenuOption::new("新しいスレッドを作成", "new_thread"),
                CreateSelectMenuOption::new("新しいフォーラム投稿を作成", "new_forum_post"),
            ],
        },
    ));

    ctx.send(
        CreateReply::default()
            .components(vec![select_threader])
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
