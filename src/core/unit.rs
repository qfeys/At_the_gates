use cgmath::Rad;
use core::position::Position;

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct IndivId {
    pub id: u32,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct UnitTypeId {
    pub id: u16,
}

#[derive(Clone, Debug)]
pub struct Indiv {
    pub id: IndivId,
    pub pos: Position,
    pub rot: Rad<f32>,
    pub player_id: u8, // Replace by PlayerID?
    pub type_id: UnitTypeId,
    pub hp: i8,
    pub xp: i8,
}

#[derive(Clone, Debug)]
pub struct UnitType {
    pub name: String,
    pub count: u8,
    pub size: u8,
    pub hp: u8,
    pub defence_skill: u8,
    pub armor: u8,
    pub shield: u8,
    pub attack_skill: i32,
    // pub weapon_type: WeaponType,     // make enum weaopn type
    pub speed: u8,
    pub cost_recruit: f32,
    pub cost_upkeep: f32,
}
