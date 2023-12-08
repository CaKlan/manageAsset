
fn main() {

    let mut player_list = Vec::<Player>::new();

    if player_list.len() > 0 { // 세이브된 캐릭터가 있다면
        println!("캐릭터를 선택하세요.");
    } else {
        println!("캐릭터를 생성하세요 ");
    }
    
    display_box();
    let user_response = wait_input();
}

fn wait_input() -> String{
    let stdin = std::io::stdin();
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).unwrap();
    user_input
}

fn display_box(){
    println!("┌────────────────────────────────────────────────────────────────────────┐");
    println!("│                                                                        │");
    println!("│                                                                        │");
    println!("│                                                                        │");
    println!("│                                                                        │");
    println!("│                                                                        │");
    println!("│                                                                        │");
    println!("└────────────────────────────────────────────────────────────────────────┘");
}

struct GameObjectInfo{
    level:f64,
    hp: f64,
    max_hp: f64,
    mp: f64,
    max_mp: f64,
}

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
}

fn create_player() -> Player{
    let stdin = std::io::stdin();
    let mut buf = String::new();
    print!("캐릭터의 이름 : ");
}