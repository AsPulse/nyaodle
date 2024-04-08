pub mod custom_id;
pub mod interactions;
pub mod register_commands_buttons;
use std::env;

use once_cell::sync::Lazy;
use poise::serenity_prelude::GuildId;

pub static ADMIN_GUILD_ID: Lazy<GuildId> = Lazy::new(|| {
    let str = env::var("ADMIN_GUILD_ID").expect("Expected ADMIN_GUILD_ID in the environment");
    let id = str
        .parse::<u64>()
        .expect("ADMIN_GUILD_ID must be a valid u64");
    GuildId::new(id)
});
