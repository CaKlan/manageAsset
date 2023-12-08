use std::io::Write;
use colored::Colorize;

fn main() {

    let mut player_list = Vec::<Player>::new();

    if player_list.len() > 0 { // 세이브된 캐릭터가 있다면
        println!("캐릭터를 선택하세요.");
    } else {
        println!("캐릭터를 생성하세요 ");
        let player = create_player();
        player_list.push(player);
    }
    
    display_box(&player_list[0]);
    player_list[0].show_info();
    let user_response = wait_input();
}

fn wait_input() -> String{
    let stdin = std::io::stdin();
    let mut user_input = String::new();
    let _ = stdin.read_line(&mut user_input).unwrap();
    user_input
}

fn display_box(player : &Player){
    println!("┌────────────────────────────────────────────────────────────────────────┐"); 
    println!("│  {:^width$}  │", player.name, width = 68); //72 공백
    println!("│  {:^width$}  │", "", width = 68); //72 공백
    println!("│  {:^width$}  │", "", width = 68); //72 공백
    println!("│  {:^width$}  │", "", width = 68); //72 공백
    println!("│  {:^width$}  │", "", width = 68); //72 공백
    println!("│  {:^width$}  │", "", width = 68); //72 공백
    println!("└────────────────────────────────────────────────────────────────────────┘");
}

#[derive(Debug)]
struct GameObjectInfo{
    level:i32,
    hp: f64,
    max_hp: f64,
    mp: f64,
    max_mp: f64,
}

#[derive(Debug)]
struct Player{
    name: String,
    info: GameObjectInfo,
}

impl Player{
    fn new(name:String, info: GameObjectInfo) -> Self {
        Player {
            name,
            info,
        }
    }

    fn show_info(&self){
        println!("  {:^width$}  ", 
        format!("name : {name} | {lvl} | {hp} | {mp}", 
            name = self.name, 
            lvl = format!("lvl : {}", self.info.level).as_str().yellow(), 
            hp = format!("hp : {}/{}", self.info.hp, self.info.max_hp).as_str().red(), 
            mp = format!("mp : {}/{}", self.info.mp, self.info.max_mp).as_str().blue()), 
            width = 97);
    }
}

fn create_player() -> Player{
    let stdin = std::io::stdin();
    let mut buf = String::new();
    print!("캐릭터의 이름 : ");
    let _ = std::io::stdout().flush();
    stdin.read_line(&mut buf);
    let name = buf.trim().to_string();

    // print!("캐릭터의 직업 : ");
    // stdin.read_line(&mut buf);
    // let name = buf;
    Player{
        name: name,
        info: GameObjectInfo {
            level: 1,
            hp: 50.0,
            max_hp: 50.0,
            mp: 50.0,
            max_mp: 50.0,
        }
    }
}