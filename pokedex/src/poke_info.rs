use serde;

#[derive(Debug, serde::Deserialize)]
pub enum Type {
    Normal,
    Feuer,
    Wasser,
    Pflanze,
    Elektro,
    Gift,
    Boden,
    Flug,
    Käfer,
    Kampf,
}
#[derive(Debug, serde::Deserialize)]
pub struct Pokemon {
    pub name: String,
    //lvl: u8,
    pub description: String,
    pub poke_type: Type,
    //weakness: 
    pub height: String,
    pub weight: String,
    pub abilities: String,
}
