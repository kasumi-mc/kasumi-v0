use std::collections::HashMap;

use bytes::BytesMut;
use serde::{Deserialize, Serialize};

use crate::protocol::{PrefixedArray, Readable, Writeable, text::Color};

#[derive(Debug)]
pub struct RegistryData {
    pub registry_id: String,
    pub entries: PrefixedArray<RegistryDataEntry>,
}

#[derive(Debug)]
pub struct RegistryDataEntry {
    pub entry_id: String,
    pub data: Option<Box<[u8]>>,
}

impl RegistryDataEntry {
    pub fn from_nbt(name: &str, nbt: &impl Serialize) -> Result<Self, pumpkin_nbt::Error> {
        let mut nbt_data_buffer = Vec::new();
        pumpkin_nbt::serializer::to_bytes_unnamed(nbt, &mut nbt_data_buffer)?;
        Ok(Self {
            entry_id: name.to_string(),
            data: Some(nbt_data_buffer.into_boxed_slice()),
        })
    }
}

impl Readable for RegistryDataEntry {
    fn read(buffer: &[u8]) -> Result<(Self, usize), crate::protocol::ReadError> {
        todo!()
    }
}

impl Writeable for RegistryDataEntry {
    fn write(&self) -> Result<bytes::Bytes, crate::protocol::WriteError> {
        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(&self.entry_id.write()?);

        buffer.extend_from_slice(&self.data.is_some().write()?);
        if let Some(data) = &self.data {
            buffer.extend_from_slice(&data);
        }
        Ok(buffer.freeze())
    }
}

impl Writeable for RegistryData {
    fn write(&self) -> Result<bytes::Bytes, crate::protocol::WriteError> {
        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(&self.registry_id.write()?);
        buffer.extend_from_slice(&self.entries.write()?);
        Ok(buffer.freeze())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryBiomeEffectsParticleOptions {
    #[serde(rename = "type")]
    pub option_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryBiomeEffectsParticle {
    pub options: RegistryBiomeEffectsParticleOptions,
    pub probability: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryBiomeEffectsAmbientSound {
    pub sound_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegistryBiomeEffectsAmbientSoundType {
    SoundtrackId(String),
    AmbientSound(RegistryBiomeEffectsAmbientSound),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryBiomeEffectsMoodSound {
    pub sound: String,
    pub tick_delay: i32,
    pub block_search_extent: i32,
    pub offset: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryBiomeEffectsAdditionsSound {
    pub sound: String,
    pub tick_chance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryBiomeEffectsMusicData {
    pub sound: String,
    pub min_delay: i32,
    pub max_delay: i32,
    pub replace_current_music: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryBiomeEffectsMusic {
    pub data: RegistryBiomeEffectsMusicData,
    pub weight: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryBiomeEffects {
    pub fog_color: i32,
    pub water_color: i32,
    pub water_fog_color: i32,
    pub sky_color: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foliage_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grass_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grass_color_modifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle: Option<RegistryBiomeEffectsParticle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient_sound: Option<RegistryBiomeEffectsAmbientSoundType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood_sound: Option<RegistryBiomeEffectsMoodSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions_sound: Option<RegistryBiomeEffectsAdditionsSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music: Option<Vec<RegistryBiomeEffectsMusic>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegistryBiomeCarvers {
    String(String),
    Vec(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryBiome {
    pub has_precipitation: bool,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_modifier: Option<String>,
    pub downfall: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creature_spawn_probability: Option<f32>,
    pub carvers: RegistryBiomeCarvers,
    pub features: Vec<Vec<String>>,
    pub effects: RegistryBiomeEffects,
    // TODO: spawners and spawn_costs
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryMobVariant {
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    // TODO: spawn conditions
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryWolfVariantAssets {
    pub angry: String,
    pub tame: String,
    pub wild: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryWolfVariant {
    pub assets: RegistryWolfVariantAssets,
    // TODO: spawn conditions
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryWolfSoundVariant {
    pub ambient_sound: String,
    pub death_sound: String,
    pub growl_sound: String,
    pub hurt_sound: String,
    pub pant_sound: String,
    pub whine_sound: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegistryPaintingVariantText {
    String(String),
    // TODO: fix
    TextComponent { color: Color, translate: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryPaintingVariant {
    pub asset_id: String,
    pub height: i32,
    pub width: i32,
    pub title: RegistryPaintingVariantText,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<RegistryPaintingVariantText>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryDimensionTypeMonsterSpawnLightLevel {
    #[serde(rename = "type")]
    pub light_level_type: String,
    pub max_inclusive: i32,
    pub min_inclusive: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegistryDimensionTypeMonsterSpawnLightLevelType {
    Int(i32),
    RegistryDimensionTypeMonsterSpawnLightLevel(RegistryDimensionTypeMonsterSpawnLightLevel),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryDimensionType {
    pub ambient_light: f32,
    pub bed_works: bool,
    pub coordinate_scale: f32,
    pub effects: String,
    pub has_ceiling: bool,
    pub has_raids: bool,
    pub has_skylight: bool,
    pub height: i32,
    pub infiniburn: String,
    pub logical_height: i32,
    pub min_y: i32,
    pub monster_spawn_block_light_limit: i32,
    pub monster_spawn_light_level: RegistryDimensionTypeMonsterSpawnLightLevelType,
    pub natural: bool,
    pub piglin_safe: bool,
    pub respawn_anchor_works: bool,
    pub ultrawarm: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryDamageType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub death_message_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<String>,
    pub exhaustion: f32,
    pub message_id: String,
    pub scaling: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    #[serde(rename = "minecraft:worldgen/biome")]
    pub biome: HashMap<String, RegistryBiome>,
    #[serde(rename = "minecraft:cat_variant")]
    pub cat_variant: HashMap<String, RegistryMobVariant>,
    #[serde(rename = "minecraft:chicken_variant")]
    pub chicken_variant: HashMap<String, RegistryMobVariant>,
    #[serde(rename = "minecraft:cow_variant")]
    pub cow_variant: HashMap<String, RegistryMobVariant>,
    #[serde(rename = "minecraft:frog_variant")]
    pub frog_variant: HashMap<String, RegistryMobVariant>,
    #[serde(rename = "minecraft:pig_variant")]
    pub pig_variant: HashMap<String, RegistryMobVariant>,
    #[serde(rename = "minecraft:wolf_variant")]
    pub wolf_variant: HashMap<String, RegistryWolfVariant>,
    #[serde(rename = "minecraft:wolf_sound_variant")]
    pub wolf_sound_variant: HashMap<String, RegistryWolfSoundVariant>,
    #[serde(rename = "minecraft:painting_variant")]
    pub painting_variant: HashMap<String, RegistryPaintingVariant>,
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension_type: HashMap<String, RegistryDimensionType>,
    #[serde(rename = "minecraft:damage_type")]
    pub damage_type: HashMap<String, RegistryDamageType>,
}

#[macro_export]
macro_rules! generate_registry_builder {
    ($function_name:ident, $field_name: ident) => {
        pub fn $function_name(registry: &Registry) -> RegistryData {
            let entries: Vec<RegistryDataEntry> = registry
                .$field_name
                .iter()
                .map(|(name, nbt)| RegistryDataEntry::from_nbt(name, nbt).unwrap())
                .collect();
            RegistryData {
                registry_id: format!("minecraft:{}", stringify!($field_name)),
                entries: PrefixedArray(entries),
            }
        }
    };
}

pub fn build_biome(registry: &Registry) -> RegistryData {
    let entries: Vec<RegistryDataEntry> = registry
        .biome
        .iter()
        .map(|(name, nbt)| RegistryDataEntry::from_nbt(name, nbt).unwrap())
        .collect();
    RegistryData {
        registry_id: String::from("minecraft:worldgen/biome"),
        entries: PrefixedArray(entries),
    }
}

// generate_registry_builder!(build_biome, biome);
generate_registry_builder!(build_cat_variant, cat_variant);
generate_registry_builder!(build_chicken_variant, chicken_variant);
generate_registry_builder!(build_cow_variant, cow_variant);
generate_registry_builder!(build_frog_variant, frog_variant);
generate_registry_builder!(build_pig_variant, pig_variant);
generate_registry_builder!(build_wolf_variant, wolf_variant);
generate_registry_builder!(build_wolf_sound_variant, wolf_sound_variant);
generate_registry_builder!(build_painting_variant, painting_variant);
generate_registry_builder!(build_dimension_type, dimension_type);
generate_registry_builder!(build_damage_type, damage_type);

pub fn build_registries_data() -> Result<Vec<RegistryData>, pumpkin_nbt::Error> {
    let raw_registry_json = include_str!("../new_registry.json");
    let registry: Registry = serde_json::from_str(raw_registry_json).unwrap();

    Ok(vec![
        build_biome(&registry),
        build_cat_variant(&registry),
        build_chicken_variant(&registry),
        build_cow_variant(&registry),
        build_frog_variant(&registry),
        build_pig_variant(&registry),
        build_wolf_variant(&registry),
        build_wolf_sound_variant(&registry),
        build_painting_variant(&registry),
        build_dimension_type(&registry),
        build_damage_type(&registry),
    ])
}
