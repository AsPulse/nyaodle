use bson::doc;
use bson::oid::ObjectId;
use poise::serenity_prelude::{
    ButtonStyle, ChannelType, ComponentInteractionDataKind, CreateActionRow, CreateButton,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption,
};
use poise::CreateReply;

use crate::db::MongoDBExt;
use crate::event::component_interaction::ComponentInteractionEvent;
use crate::threader::ThreaderConfiguration;
use crate::{ApplicationContext, Error};

use super::configure_threader_interactions::ConfigureThreaderDocs;

pub(crate) async fn configure_threader(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let docs = ConfigureThreaderDocs::create_and_insert(&ctx).await?;

    ctx.send(
        CreateReply::default()
            .components(configure_threader_component(&docs))
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
    let mut docs = ConfigureThreaderDocs::from_interaction_ids(
        event.data,
        ObjectId::parse_str(event.interaction.data.custom_id.clone())?,
    )
    .await?;

    docs.config.configuration = if selected_type == "another_channel" {
        ThreaderConfiguration::AnotherChannel { channel_id: None }
    } else if selected_type == "new_thread" {
        ThreaderConfiguration::NewThread {}
    } else if selected_type == "new_forum_post" {
        ThreaderConfiguration::NewForumPost {}
    } else {
        panic!("Unexpected selected_type: {}", selected_type);
    };

    event
        .data
        .mongo()
        .threader_configurations
        .update_one(
            doc! {
                "_id": docs.config._id,
            },
            doc! {
                "$set": {
                    "configuration": bson::to_bson(&docs.config.configuration)?
                }
            },
            None,
        )
        .await?;

    event
        .interaction
        .create_response(
            event.ctx,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .components(configure_threader_component(&docs)),
            ),
        )
        .await?;
    Ok(())
}

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
