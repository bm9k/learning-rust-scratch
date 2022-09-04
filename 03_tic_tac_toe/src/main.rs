use std::io::{self, Write};

// [value; length] syntax requires value implements Copy trait
// https://stackoverflow.com/a/29437510
// not sure if this is the best solution
#[derive(Copy, Clone, PartialEq, Eq)]
enum Player {
    X,
    O,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Taken(Player),
}

impl Player {
    fn value(&self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }
}

impl Cell {
    fn value(&self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::Taken(player) => player.value(),
        }
    }
}

type Board = [[Cell; 3]; 3];

enum GameResult {
    Won(Player),
    Incomplete,
}

struct Game {
    board: Board,
    active: Player,
}

impl Game {
    fn new() -> Game {
        let board = [[Cell::Empty; 3]; 3];
        Game {
            board,
            active: Player::X,
        }
    }

    fn print_board(&self) {
        println!("\n+---+---+---+");

        for (i, row) in self.board.iter().enumerate() {
            // TODO: dedupe
            println!(
                "| {}{}\x1b[0m | {}{}\x1b[0m | {}{}\x1b[0m |",
                match row[0] {
                    Cell::Taken(Player::X) => "\x1b[33m",
                    _ => "\x1b[35m",
                },
                row[0].value(),
                match row[1] {
                    Cell::Taken(Player::X) => "\x1b[33m",
                    _ => "\x1b[35m",
                },
                row[1].value(),
                match row[2] {
                    Cell::Taken(Player::X) => "\x1b[33m",
                    _ => "\x1b[35m",
                },
                row[2].value()
            );
            println!("| {} | {} | {} |", i * 3 + 1, i * 3 + 2, i * 3 + 3);
            println!("+---+---+---+");
        }

        println!("");
    }

    fn check_winner(&self) -> GameResult {
        // TODO: refactor/dedupe
        let board = self.board;
        // rows
        for i in 0..3 {
            if (board[i][0] == board[i][1]) && (board[i][0] == board[i][2]) {
                if let Cell::Taken(player) = board[i][0] {
                    return GameResult::Won(player);
                }
            }
        }

        // columns
        for i in 0..3 {
            if (board[0][i] == board[1][i]) && (board[0][i] == board[2][i]) {
                if let Cell::Taken(player) = board[0][i] {
                    return GameResult::Won(player);
                }
            }
        }

        // diagonals
        if (board[0][0] == board[1][1] && board[1][1] == board[2][2])
            || (board[0][2] == board[1][1] && board[1][1] == board[2][0])
        {
            if let Cell::Taken(player) = board[1][1] {
                return GameResult::Won(player);
            }
        }

        GameResult::Incomplete
    }

    fn take(&mut self, row: usize, column: usize) {
        self.board[row][column] = Cell::Taken(self.active);
    }

    fn next_player(&mut self) {
        self.active = match self.active {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }
}

fn address_to_row_column(address: usize) -> (usize, usize) {
    return ((address - 1) / 3, (address - 1) % 3);
}

fn prompt_address_once() -> Result<(usize, usize), String> {
    print!("Please choose a number from 1-9: ");
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Read failed");

    if let Ok(num) = line.trim().to_string().parse::<usize>() {
        if let 1..=9 = num {
            return Ok(address_to_row_column(num));
        }
    }

    Err("Must be num from 1-9".to_string())
}

fn prompt_play_again() -> bool {
    print!("Would you like to play again? [Y/n]: ");
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Read failed");

    let line = line.trim();

    return !(line == "N" || line == "n");

}

fn prompt_address(game: &Game) -> (usize, usize) {
    loop {
        match prompt_address_once() {
            Ok((row, column)) => match game.board[row][column] {
                Cell::Empty => break (row, column),
                Cell::Taken(_) => {
                    println!("That cell is already taken\n");
                    continue;
                }
            },
            Err(_) => {
                println!("Must enter a number from 1-9\n");
                continue;
            }
        }
    }
}

fn main() {
    loop {
        println!("\n🔥 🔥 🔥 🔥 🔥 🔥\n");
        println!("🔥 \x1b[31mTic\x1b[0m-\x1b[32mTac\x1b[0m-\x1b[34mToe\x1b[0m 🔥");
        println!("\n🔥 🔥 🔥 🔥 🔥 🔥");
        let mut game = Game::new();

        loop {
            game.print_board();
            println!("Player {}'s turn", game.active.value());

            let (row, column) = prompt_address(&game);
            game.take(row, column);
            game.next_player();

            match game.check_winner() {
                GameResult::Incomplete => continue,
                GameResult::Won(player) => {
                    game.print_board();
                    println!("Player {} wins!", player.value());
                    break;
                }
            }
        }

        if !prompt_play_again() {
            break;
        }
    }
}
