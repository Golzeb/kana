use std::{collections::HashMap, sync::Arc};

use serenity::{model::prelude::GuildId, prelude::*};
use songbird::tracks::TrackQueue;

pub struct KanaSongQueue;

impl TypeMapKey for KanaSongQueue {
    type Value = Arc<Mutex<HashMap<GuildId, TrackQueue>>>;
}

pub mod leave;
pub mod play;
pub mod skip;
