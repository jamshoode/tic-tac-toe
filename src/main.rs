use std::io;

struct Player {
    name: String,
    char: String,
    is_won: bool,
}

struct Game {
    board: Vec<String>,
    player1: Player,
    player2: Player,
    turn: Player,
    status: String,
}

impl Clone for Player {
    fn clone(&self) -> Player {
        Player {
            name: self.name.clone(),
            char: self.char.clone(),
            is_won: self.is_won.clone(),
        }
    }
}
impl Player {
    fn new(name: String, char: String) -> Player {
        Player {
            name,
            char,
            is_won: false,
        }
    }
}

impl Game {
    fn new(player1: Player, player2: Player) -> Game {
        Game {
            board: vec![
                "1".to_string(), "2".to_string(), "3".to_string(),
                "4".to_string(), "5".to_string(), "6".to_string(),
                "7".to_string(), "8".to_string(), "9".to_string()
            ],
            player1: player1.clone(),
            player2: player2.clone(),
            turn: player1.clone(),
            status: "Playing".to_string(),
        }
    }

    fn draw_board(&self) {
        let mut count = 0;

        for i in &self.board {
            if count % 3 == 0 {
                println!();
            }
            print!("{} ", i);
            count += 1;
        }

        println!();
    }

    fn check_win(&mut self) {
        let mut count = 0;
        let mut is_won = false;

        for _ in 0..3 {
            if self.board[count] == self.board[count + 1] && self.board[count] == self.board[count + 2] {
                is_won = true;
            }
            count += 3;
        }
        
        count = 0;
        for _ in 0..3 {
            if self.board[count] == self.board[count + 3] && self.board[count] == self.board[count + 6] {
                is_won = true;
            }
            count += 1;
        }
        
        if self.board[0] == self.board[4] && self.board[0] == self.board[8] {
            is_won = true;
        }
        if self.board[2] == self.board[4] && self.board[2] == self.board[6] {
            is_won = true;
        }

        if is_won {
            self.status = "Won".to_string();
            self.turn.is_won = true;
            println!("{} has won!", self.turn.name.trim());
            self.draw_board();
        }
    }

    fn turn(&mut self) {
        while self.status == "Playing" {
            let mut input = String::new();

            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            self.draw_board();

            println!("{}'s turn", self.turn.name.trim());
            println!("Enter a number to place your character: ");

            io::stdin().read_line(&mut input).expect("Failed to read line");

            let input: usize = input.trim().parse().expect("Please type a number!");

            if input > 9 || input < 1 {
                println!("Please enter a number between 1 and 9");
                continue;
            }

            if self.board[input - 1] == "X" || self.board[input - 1] == "O" {
                println!("That spot is already taken!");
                continue;
            }

            self.board[input - 1] = self.turn.char.clone();

            self.check_win();

            if self.turn.name == self.player1.name {
                self.turn = self.player2.clone();
            } else {
                self.turn = self.player1.clone();
            }

        }
    }
}

fn init_players() -> (Player, Player) {
    let mut player1 = String::new();
    let mut player2 = String::new();

    println!("Enter player 1 name: ");
    io::stdin().read_line(&mut player1).expect("Failed to read line");

    println!("Enter player 2 name: ");
    io::stdin().read_line(&mut player2).expect("Failed to read line");

    let player1 = Player::new(player1, "X".to_string());
    let player2 = Player::new(player2, "O".to_string());

    (player1, player2)
}

fn game() {
    let (p1, p2): (Player, Player) = init_players();

    let mut game: Game = Game::new(p1, p2);
    game.turn();
}

fn main() {
    game();
}
