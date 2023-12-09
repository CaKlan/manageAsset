use crate::{flush, cls, objects::{Player, GameObjectInfo}};

//커맨드 실행 시 구현 함수입니다
pub fn command(comm : String, is_show_command : &mut bool){
    let comm_str = comm.as_str().replace("/", "");
    let trimmed_str = comm_str.trim();

    if trimmed_str == "exit" { // save & exit game
        print!("정말로 종료하시겠습니까? y/n : ");
        flush!();
        let stdin = std::io::stdin();
        let mut buf = String::new();
        let _ = stdin.read_line(&mut buf);
        
        if buf.trim() == "y" || buf.trim() == "Y"{
            std::process::exit(0);
        }else {
            return;
        }
        
    } else if trimmed_str == "save" { // save current progress

    } else if trimmed_str == "help"{
        print_help();
        *is_show_command = true;
    } else { // unknown command
        println!("알 수 없는 명령어입니다.");
        
        print_help();
        *is_show_command = true;
    }
}

//help 명령어가 실행될때 프린트됩니다.
pub fn print_help(){
    let comms = [
        ("exit", "save & exit game."), 
        ("save", "save the current progress."),
        ("help", "tell you the commands."),
        ("helpqwrqwrqwr", "tell you the commandqwrs.")];

    let padding = match comms.map(|c| c.0.len()).iter().max() {
        Some(l) => *l + 1,
        None => 1_usize
    };
    cls!();
    println!("/{{명령어}}의 형식으로 명령어를 입력하세요.\n");
    
    for c in comms{
        println!("{:width$}: {}", c.0, c.1,width = padding);
    }
}

//유저 입력을 기다립니다
pub fn wait_input() -> String{
    let stdin = std::io::stdin();
    let mut user_input = String::new();
    print!("> ");
    flush!();
    let _ = stdin.read_line(&mut user_input).unwrap();
    user_input
}

//박스 출력
pub fn display_box(){
    println!("┌────────────────────────────────────────────────────────────────────────┐"); 
    println!("│  {:^width$}  │", "", width = 68); //72 공백
    println!("│  {:^width$}  │", "", width = 68); //72 공백
    println!("│  {:^width$}  │", "", width = 68); //72 공백
    println!("│  {:^width$}  │", "", width = 68); //72 공백
    println!("│  {:^width$}  │", "", width = 68); //72 공백
    println!("│  {:^width$}  │", "", width = 68); //72 공백
    println!("└────────────────────────────────────────────────────────────────────────┘");
}

//플레이어를 새로 만듭니다.
pub fn create_player() -> Player{
    let stdin = std::io::stdin();
    let mut buf = String::new();
    print!("캐릭터의 이름 : ");
    flush!();
    let _ = stdin.read_line(&mut buf);
    let name = buf.trim().to_string();

    // print!("캐릭터의 직업 : ");
    // stdin.read_line(&mut buf);
    // let name = buf;
    Player::new(name, GameObjectInfo {
        level: 1,
        hp: 50.0,
        max_hp: 50.0,
        mp: 50.0,
        max_mp: 50.0,
    })
}

//게임에 필요한 리소스를 로딩합니다.
pub fn load_game(player_list : &mut Vec<Player>){
    let players_path = "players.json";
    let buf = match std::fs::read_to_string(players_path) {
        Ok(b) => b,
        Err(_) => String::from("[]")
    };
    println!("로딩 중...");
    let mut loaded_list : Vec<Player> = serde_json::from_str(&buf)
        .expect("Failed to convert str to Vec<Player>");

    player_list.append(&mut loaded_list);
    println!("로딩 끝!");
}