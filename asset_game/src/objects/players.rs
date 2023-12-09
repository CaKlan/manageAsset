use colored::Colorize;
use serde::{Serialize, Deserialize};

use super::GameObjectInfo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player{
    pub name: String,
    pub info: GameObjectInfo,
}

impl Player{
    pub fn new(name:String, info: GameObjectInfo) -> Self {
        let player = Player {
            name,
            info,
        };
        player
    }

    pub fn info(&self) -> String{
        format!("  {:^width$}  ", 
        format!("name : {name} | {lvl} | {hp} | {mp}", 
            name = self.name, 
            lvl = format!("lvl : {}", self.info.level).as_str().yellow(), 
            hp = format!("hp : {}/{}", self.info.hp, self.info.max_hp).as_str().red(), 
            mp = format!("mp : {}/{}", self.info.mp, self.info.max_mp).as_str().blue()), 
            width = 97)
    }
}