use super::general_card::{CardImage, Attribute, LinkMarker};

#[derive(Debug)]
pub enum DraftedCard {
    Monster(MonsterCard),
    Spell(SpellCard),
    Trap(TrapCard)
}

#[derive(Debug)]
pub struct MonsterCard {
    pub name:        String,
    pub description: String,
    pub images:      Vec<CardImage>,

    pub game_type:   MonsterType,
    pub card_type:   MCardType,

    pub atk:         i32,
    pub def:         i32,
    pub attribute:   Attribute,
    pub level:       u32,

    pub linkval:     Option<u32>,
    pub linkmarkers: Option<Vec<LinkMarker>>,
}

#[derive(Debug)]
pub enum MonsterType {
    Aqua,
    Beast,
    BeastWarrior,
    Cyberse,
    Dinosaur,
    Dragon,
    Fairy,
    Fiend,
    Fish,
    Insect,
    Machine,
    Plant,
    Psychic,
    Pyro,
    Reptile,
    Rock,
    SeaSerpent,
    Spellcaster,
    Thunder,
    Warrior,
    WingedBeast,
    Wyrm,
    Zombie
}

#[derive(Debug)]
pub enum MCardType {
    Normal,
    Effect,
    Fusion,
    Synchro,
    Xyz,
    Link
}

#[derive(Debug)]
pub struct SpellCard {
    pub name:        String,
    pub description: String,
    pub images:      Vec<CardImage>,
    pub spell_type:  SpellType,
}

#[derive(Debug)]
pub enum SpellType {
    Normal,
    Continuous,
    Field,
    Equip,
    QuickPlay
}

#[derive(Debug)]
pub struct TrapCard {
    pub name:        String,
    pub description: String,
    pub images:      Vec<CardImage>,
    pub trap_type:   TrapType,
}


#[derive(Debug)]
pub enum TrapType {
    Normal,
    Continuous,
    Counter
}