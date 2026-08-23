use serde::{Deserialize, Serialize};

use crate::{chunk::ChunkConfig, lighting::LightingEngineConfig};

/// Which world generation preset to use.
#[derive(Deserialize, Serialize, Default, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WorldType {
    /// Vanilla Minecraft noise-based terrain (384 blocks tall).
    #[default]
    Normal,
    /// Flat superflat world.
    Flat,
    /// Chumpkin world — 2048 blocks tall, mountains up to ~1500.
    Chumpkin,
}

/// Configuration for world and level-specific settings.
///
/// Currently, it includes chunk-related options; more settings may be added later.
#[derive(Deserialize, Serialize, Default, Clone)]
pub struct LevelConfig {
    /// Configuration for chunk behaviour and management.
    pub chunk: ChunkConfig,
    /// Configuration for lighting engine propagation mode.
    #[serde(default)]
    pub lighting: LightingEngineConfig,
    /// Number of ticks between autosave checks. If 0, autosave is disabled.
    #[serde(default = "default_autosave_ticks")]
    pub autosave_ticks: u64,
    /// World generation preset.
    #[serde(default)]
    pub world_type: WorldType,
}

const fn default_autosave_ticks() -> u64 {
    6000 // Default to 5 minutes at 20 TPS
}