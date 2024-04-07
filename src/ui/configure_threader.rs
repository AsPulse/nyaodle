use poise::serenity_prelude::{
    ButtonStyle, ChannelType, ComponentInteractionDataKind, CreateActionRow, CreateButton,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption,
};
use poise::CreateReply;

use crate::event::component_interaction::ComponentInteractionEvent;
use crate::{ApplicationContext, Error};

use super::configure_threader_interactions::ConfigureThreaderDocs;

pub(crate) async fn configure_threader(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let docs = ConfigureThreaderDocs::create_and_insert(&ctx).await?;

    ctx.send(
        CreateReply::default()
            .components(configure_threader_component(&docs, None))
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

pub(crate) async fn update_configure_threader(
    event: &ComponentInteractionEvent<'_>,
) -> Result<(), Error> {
    let selected_type = match &event.interaction.data.kind {
        ComponentInteractionDataKind::StringSelect { values: data } => data[0].as_str(),
        _ => {
            panic!("Unexpected data kind: {:?}", event.interaction.data.kind);
        }
    };
    event
        .interaction
        .create_response(
            event.ctx,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new().components(configure_threader_component(
                    todo!(),
                    Some(selected_type.to_string()),
                )),
            ),
        )
        .await?;
    Ok(())
}

pub fn configure_threader_component(
    docs: &ConfigureThreaderDocs,
    default: Option<String>,
) -> Vec<CreateActionRow> {
    let default = default.unwrap_or("another_channel".to_string());

    let mut components = vec![];
    components.push(CreateActionRow::SelectMenu(CreateSelectMenu::new(
        docs.select_id.to_hex(),
        CreateSelectMenuKind::String {
            options: vec![
                CreateSelectMenuOption::new("別のチャンネルに移動", "another_channel")
                    .default_selection("another_channel" == default),
                CreateSelectMenuOption::new("新しいスレッドを作成", "new_thread")
                    .default_selection("new_thread" == default),
                CreateSelectMenuOption::new("新しいフォーラム投稿を作成", "new_forum_post")
                    .default_selection("new_forum_post" == default),
            ],
        },
    )));

    if default == "another_channel" {
        components.push(CreateActionRow::SelectMenu(
            CreateSelectMenu::new(
                "another_channel",
                CreateSelectMenuKind::Channel {
                    channel_types: Some(vec![ChannelType::Text, ChannelType::Private]),
                    default_channels: None,
                },
            )
            .placeholder("移動先のチャンネル"),
        ))
    }

    components.push(if default == "another_channel" {
        CreateActionRow::Buttons(vec![
            CreateButton::new(docs.execute_id.to_hex())
                .label("実行")
                .style(ButtonStyle::Primary),
            CreateButton::new(docs.close_id.to_hex())
                .label("キャンセル")
                .style(ButtonStyle::Secondary),
        ])
    } else {
        CreateActionRow::Buttons(vec![
            CreateButton::new(docs.execute_id.to_hex())
                .label("(実装中)")
                .style(ButtonStyle::Primary)
                .disabled(true),
            CreateButton::new(docs.close_id.to_hex())
                .label("キャンセル")
                .style(ButtonStyle::Secondary),
        ])
    });

    components
}
