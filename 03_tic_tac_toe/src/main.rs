// [value; length] syntax requires value implements Copy trait
// https://stackoverflow.com/a/29437510
// not sure if this is the best solution
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
enum Player {
    X,
    O,
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
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

type Board = [[Cell; 3]; 3];

enum GameResult {
    Won(Player),
    Incomplete
}

struct Game {
    board: Board,
    active: Player,
}

impl Game {
    fn new() -> Game {
        let board = [[Cell::Empty; 3]; 3];
        Game{board, active: Player::X}
    }

    fn print_board(&self) {
        let mut output = String::new();

        for row in self.board {
            for cell in row {
                output.push(match cell {
                    Cell::Taken(player) => player.value(),
                    Cell::Empty => '-',
                })
            }
            output.push('\n');
        }

        println!("{}", output);
    }

    fn check_winner(&self) -> GameResult {
        // TODO: refactor/dedupe
        let board = self.board;
        // rows
        for i in 0..3 {
            if (board[i][0] == board[i][1]) && (board[i][0] == board[i][2]) {
                match board[i][0] {
                    Cell::Empty => continue,
                    Cell::Taken(player) => {
                        return GameResult::Won(player);
                    }
                }
            }
        }

        // columns
        for i in 0..3 {
            if (board[0][i] == board[1][i]) && (board[0][i] == board[2][i]) {
                match board[0][i] {
                    Cell::Empty => continue,
                    Cell::Taken(player) => {
                        return GameResult::Won(player);
                    }
                }
            }
        }

        // diagonals
        if (board[0][0] == board[1][1] && board[1][1] == board[2][2]) || 
                (board[0][2] == board[1][1] && board[1][1] == board[2][0]) {
            match board[1][1] {
                Cell::Empty => {},
                Cell::Taken(player) => {
                    return GameResult::Won(player);
                }
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

fn main() {
    let mut game = Game::new();

    let move_coords = [
        // diagonal
        (1, 1),
        (0, 0),
        (2, 1),
        (0, 1),
        (0, 2),
        (1, 0),
        (2, 0),
        (2, 2),

        // row
        // (0, 0),
        // (2, 0),
        // (0, 1),
        // (2, 2),
        // (0, 2),
        // (2, 1),

        // column
        // (0, 0),
        // (0, 2),
        // (1, 0),
        // (1, 2),
        // (2, 0),
        // (2, 2),
    ];

    game.print_board();

    for coord in move_coords {
        let (row, column) = coord;

        // TODO: error checking
        println!("Player {} takes {}, {}\n", game.active.value(), row, column);
        game.take(row, column);

        game.next_player();

        game.print_board();

        let winner = game.check_winner();

        match winner {
            GameResult::Incomplete => continue,
            GameResult::Won(player) => {
                println!("Player {} wins!", player.value());
                break;
            }
        }
    }
}
