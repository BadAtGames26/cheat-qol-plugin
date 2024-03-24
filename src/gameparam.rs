use unity::prelude::*;
use engage::gamedata::*;

#[unity::class("App", "GameParam")]
pub struct GameParam {
    pub parent: StructBaseFields,
    pub name: &'static Il2CppString,
    pub english: &'static Il2CppString,
    pub value: f32,
    min: f32,
    max: f32,
    step: f32,
    enumerator: &'static Il2CppString,
    pub initial: f32,
}

impl Gamedata for GameParam { }