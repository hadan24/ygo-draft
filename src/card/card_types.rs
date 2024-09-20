use super::general_card::
    {Attribute, Card, CardImage, CardType, GameType, LinkMarker};
use super::Error;

#[derive(Debug)]
pub enum DraftedCard {
    Monster (MonsterCard),
    Spell   (SpellCard),
    Trap    (TrapCard)
}

#[derive(Debug)]
pub struct MonsterCard {
    pub name:        String,
    pub description: String,
    pub images:      Vec<CardImage>,

    pub game_type:   MonsterType,
    pub card_type:   MCardType,

    pub attack:      i32,
    pub defense:     i32,
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

impl MonsterCard {
    pub fn new(c: Card) -> Result<Self, Error> {
        Ok(Self {
            name: c.name,
            description: c.desc,
            images: c.card_images,
            game_type: Self::match_game_type(c.game_type)?,
            card_type: Self::match_card_type(c.card_type)?,
            attack: match c.atk {
                Some(atk) => atk,
                None => todo!()
            },
            defense: match c.def {
                Some(def) => def,
                None => todo!()
            },
            attribute: match c.attribute {
                Some(att) => att,
                None => todo!()
            },
            level: match c.level {
                Some(lvl) => lvl,
                None => todo!()
            },
            linkval: c.linkval,
            linkmarkers: c.linkmarkers,
        })
    }

    fn match_game_type(t: GameType) -> Result<MonsterType, Error> {
        use MonsterType::*;
        match t {
            GameType::Aqua          => Ok(Aqua),
            GameType::Beast         => Ok(Beast),
            GameType::BeastWarrior  => Ok(BeastWarrior),
            GameType::Cyberse       => Ok(Cyberse),
            GameType::Dinosaur      => Ok(Dinosaur),
            GameType::Dragon        => Ok(Dragon),
            GameType::Fairy         => Ok(Fairy),
            GameType::Fiend         => Ok(Fiend),
            GameType::Fish          => Ok(Fish),
            GameType::Insect        => Ok(Insect),
            GameType::Machine       => Ok(Machine),
            GameType::Plant         => Ok(Plant),
            GameType::Psychic       => Ok(Psychic),
            GameType::Pyro          => Ok(Pyro),
            GameType::Reptile       => Ok(Reptile),
            GameType::Rock          => Ok(Rock),
            GameType::SeaSerpent    => Ok(SeaSerpent),
            GameType::Spellcaster   => Ok(Spellcaster),
            GameType::Thunder       => Ok(Thunder),
            GameType::Warrior       => Ok(Warrior),
            GameType::WingedBeast   => Ok(WingedBeast),
            GameType::Wyrm          => Ok(Wyrm),
            GameType::Zombie        => Ok(Zombie),
            _ => todo!("Return error")
        }
    }
    fn match_card_type(t: CardType) -> Result<MCardType, Error> {
        use MCardType::*;
        match t {
            CardType::Normal => Ok(Normal),
            CardType::Effect => Ok(Effect),
            CardType::Fusion => Ok(Fusion),
            CardType::Synchro => Ok(Synchro),
            CardType::Xyz    => Ok(Xyz),
            CardType::Link   => Ok(Link),
            _ => todo!("Return error")
        }
    }
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
    Equip,
    Field,
    QuickPlay
}

impl SpellCard {
    pub fn new(c: Card) -> Result<Self, Error> {
        use SpellType::*;
        Ok(SpellCard {
            name: c.name,
            description: c.desc,
            images: c.card_images,
            spell_type: match c.game_type {
                GameType::Normal     => Normal,
                GameType::Continuous => Continuous,
                GameType::Equip      => Equip,
                GameType::Field      => Field,
                GameType::QuickPlay  => QuickPlay,
                _ => todo!("Return error")
            }
        })
    }
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

impl TrapCard {
    pub fn new(c: Card) -> Result<Self, Error> {
        use TrapType::*;
        Ok(TrapCard {
            name: c.name,
            description: c.desc,
            images: c.card_images,
            trap_type: match c.game_type {
                GameType::Normal     => Normal,
                GameType::Continuous => Continuous,
                GameType::Counter    => Counter,
                _ => todo!("Return error")
            }
        })
    }
}