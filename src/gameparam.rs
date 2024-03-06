#![allow(dead_code)]

use unity::prelude::*;
use engage::gamedata::*;

#[unity::class("App", "GameParam")]
pub struct GameParam {
    pub parent: StructBaseFields,
    pub name: &'static Il2CppString,
    pub english: &'static Il2CppString,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub enumerator: &'static Il2CppString,
    pub initial: f32,
}

impl Gamedata for GameParam { }