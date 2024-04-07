use poise::serenity_prelude::{
    ButtonStyle, ChannelType, CreateActionRow, CreateButton, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption,
};

use crate::threader::ThreaderConfiguration;

use super::interactions::ConfigureThreaderDocs;

pub fn configure_threader_component(docs: &ConfigureThreaderDocs) -> Vec<CreateActionRow> {
    let config = &docs.config.configuration;

    let mut components = vec![];
    components.push(CreateActionRow::SelectMenu(CreateSelectMenu::new(
        docs.select_id.to_hex(),
        CreateSelectMenuKind::String {
            options: vec![
                CreateSelectMenuOption::new("別のチャンネルに移動", "another_channel")
                    .default_selection(matches!(
                        &config,
                        ThreaderConfiguration::AnotherChannel { .. }
                    )),
                CreateSelectMenuOption::new("新しいスレッドを作成", "new_thread")
                    .default_selection(matches!(config, ThreaderConfiguration::NewThread {})),
                CreateSelectMenuOption::new("新しいフォーラム投稿を作成", "new_forum_post")
                    .default_selection(matches!(config, ThreaderConfiguration::NewForumPost {})),
            ],
        },
    )));

    match &config {
        ThreaderConfiguration::AnotherChannel { channel_id } => {
            components.push(CreateActionRow::SelectMenu(
                CreateSelectMenu::new(
                    "another_channel",
                    CreateSelectMenuKind::Channel {
                        channel_types: Some(vec![ChannelType::Text, ChannelType::Private]),
                        default_channels: channel_id.map(|id| vec![id]),
                    },
                )
                .placeholder("移動先のチャンネル"),
            ))
        }
        ThreaderConfiguration::NewThread {} => {}
        ThreaderConfiguration::NewForumPost {} => {}
    }

    match &config {
        ThreaderConfiguration::AnotherChannel { .. } => {
            components.push(CreateActionRow::Buttons(vec![
                CreateButton::new(docs.execute_id.to_hex())
                    .label("実行")
                    .style(ButtonStyle::Primary),
                CreateButton::new(docs.close_id.to_hex())
                    .label("キャンセル")
                    .style(ButtonStyle::Secondary),
            ]));
        }
        _ => {
            components.push(CreateActionRow::Buttons(vec![
                CreateButton::new(docs.execute_id.to_hex())
                    .label("(実装中)")
                    .style(ButtonStyle::Primary)
                    .disabled(true),
                CreateButton::new(docs.close_id.to_hex())
                    .label("キャンセル")
                    .style(ButtonStyle::Secondary),
            ]));
        }
    }

    components
}
