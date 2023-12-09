use std::{io::Write, char, fs::OpenOptions, path::PathBuf};
use colored::Colorize;
use objects::Player;

mod objects;
mod macros;
mod utils;
use utils::{command, wait_input, display_box, create_player, load_game};

fn main() {

    //assets 
    let mut _history = Vec::<&str>::new(); // 명령어 히스토리
    let mut player_list = Vec::<Player>::new(); // 플레이어 리스트
    let mut select_player_idx = 0; // 현재 선택된 플레이어 인덱스
    cls!();
    load_game(&mut player_list);
    
    if player_list.len() > 0 { // 세이브된 캐릭터가 있다면
        
        let mut list_str: Vec<String> = Vec::new();
        for (i, p) in player_list.iter().enumerate(){
            list_str.push(format!("[{}] Level {} {}", i+1, p.info.level, p.name));
        }
        let max_size = match list_str.iter().map(|line| line.len()).max(){
            Some(size) => size.max(22),
            None => 22,
        };
        loop {
        println!("┌{:─^width$}┐", "players", width = max_size+2);
        println!("│ {:width$} │", "[0] create new player".yellow() ,width = max_size);
        for l in &list_str{
            println!("│ {:width$} │", l ,width = max_size);
        }
        println!("└{:─^width$}┘", "", width = max_size+2);
        
        
        let stdin = std::io::stdin();
        let mut buf = String::new();
        
            buf.clear();
            print!("캐릭터를 선택하세요. (번호 입력) : ");
            flush!();
            stdin.read_line(&mut buf).unwrap();
            let num = buf.trim().parse::<usize>().unwrap();
            if num > 0 && num <= player_list.len(){
                select_player_idx = num - 1;
                break;
            } else if num == 0 {
                println!("캐릭터 생성하기...");
            } else {
                cls!();
                println!("잘못된 입력입니다. 다시 입력해주세요.");
            }
        }
    } else { // 세이브된 캐릭터가 없다면
        println!("캐릭터를 생성하세요 ");
        let player = create_player();
        player_list.push(player);

        let json = serde_json::to_string(&player_list).expect("failed to convert json data to string"); // 저장
        let player_data_path = PathBuf::from("data").join("players.json");
        println!("{:?}", player_data_path);
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("players.json")
            .expect("failed to open file");
        file.write(json.as_bytes()).expect("failed to write player data file");
    }
    
    let mut is_show_command = false; // 커맨드로 인해 보여져야 할 텍스트가 있다면 true

    //메인 게임 루프
    loop {
        if !is_show_command {
            cls!();
            display_box();
            println!("{}", &player_list[select_player_idx].info());
        }
        
        //user input process//
        let user_response = wait_input();
        if user_response.starts_with("/") { // command
            command(user_response, &mut is_show_command);
        }else { // non command
            println!("non command");
            is_show_command = false;
        }
        ////


    }
    
}
/*
// struct ObjectManager {
//     objects : Vec::<GameObject
// }

// impl ObjectManager {
//     fn new() -> Self{
//         ObjectManager {

//         }
//     }

//     fn save(){

//     }
// }
*/