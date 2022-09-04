// [value; length] syntax requires value implements Copy trait
// https://stackoverflow.com/a/29437510
// not sure if this is the best solution
#[derive(Copy, Clone)]
enum Player {
    X,
    O,
}

#[derive(Copy, Clone)]
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

    fn check_winner(&self) -> Option<Player> {
        // TODO: implement
        // rows


        // columns


        // diagonals

        // return None;
        return Some(Player::X);
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
        (1, 1),
        (0, 0),
        (2, 1),
        (0, 1),
        (0, 2),
        (1, 0),
        (2, 0),
        (2, 2),
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
            None => continue,
            Some(player) => {
                println!("Player {} wins!", player.value());
                break;
            }
        }
    }
}
