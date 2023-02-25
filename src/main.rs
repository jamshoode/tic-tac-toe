#![allow(dead_code, unused_variables, unused_mut)]
use std::io;

struct Player {
    name: String,
    char: String,
}

struct Game {
    status: bool,
    turn: String,
}

impl Game {
    pub fn status(&self) -> bool {
        self.status
    }
}

fn init_players() -> (Player, Player){
    let mut p1_name: String = String::new();
    let mut p2_name: String = String::new();
    let mut p1_char: String = String::new();
    let mut p2_char: String = String::new();

    println!("Enter 1-st player name: ");
    io::stdin().read_line(&mut p1_name).expect("Failed to parse.");

    println!("Enter 2-nd player name: ");
    io::stdin().read_line(&mut p2_name).expect("Failed to parse.");

    println!("Choose character (X or O) for 1-st player: ");
    io::stdin().read_line(&mut p1_char).expect("Failed to parse.");

    println!("Choose character (X or O) for 2-nd player: ");
    io::stdin().read_line(&mut p2_char).expect("Failed to parse.");


    let p1 = Player {
        name: p1_name,
        char: p1_char,
    };

    let p2 = Player {
        name: p2_name,
        char: p2_char,
    };

    (p1, p2)
}

fn game() {
    let (p1, p2): (Player, Player) = init_players();
    let p1_name = p1.name;
    let p2_name = p2.name;
    let mut game = Game {
        status: true,
        turn: p1_name.clone(),
    };
    let mut coords = [
        ["1".to_string(), "2".to_string(), "3".to_string()],
        ["4".to_string(), "5".to_string(), "6".to_string()],
        ["7".to_string(), "8".to_string(), "9".to_string()]
    ];    


    while game.status() != false {
        if game.turn.clone() == p1_name.clone() {
            let mut p_move: String = String::new();

            println!("Choose coordinates: ");
            for i in 0..3 {
                for j in 0..3 {
                    print!("{} ", coords[i][j])
                }
                println!("");
            }
            io::stdin().read_line(&mut p_move).expect("Failed to parse.");

            for x in 0..3 {
                for y in 0..3 {
                    if coords[x][y] == p_move.trim() {
                        coords[x][y] = p1.char.clone().trim().to_string();
                    }
                }
            }


            for x in 0..3 {
                for y in 0..3 {
                    print!("{} ", coords[x][y]);
                }
                println!("");
            }

            game.turn = p2_name.clone();
        }

        if game.turn.clone() == p2_name.clone() {
            let mut p_move: String = String::new();

            println!("Choose coordinates: ");
            for i in 0..3 {
                for j in 0..3 {
                    print!("{} ", coords[i][j])
                }
                println!("");
            }
            io::stdin().read_line(&mut p_move).expect("Failed to parse.");

            for x in 0..3 {
                for y in 0..3 {
                    if coords[x][y] == p_move.trim() {
                        coords[x][y] = p2.char.clone().trim().to_string();
                    }
                }
            }


            for x in 0..3 {
                for y in 0..3 {
                    print!("{} ", coords[x][y]);
                }
                println!("");
            }

            game.turn = p1_name.clone();
        }

        for x in 0..3 {
            for y in 0..3 {
                let mut count = 0;
                if x == y {
                    if coords[x][y] == "X".to_string() || coords[x][y] == "O".to_string() {
                        count = count + 1;
                    }
                } else if (x + y) == (3 - 1) {
                    if coords[x][y] == "X".to_string() || coords[x][y] == "O".to_string() {
                        count = count + 1;
                    }
                }
            }
        }
    }
}

fn main() {
    game();
}
