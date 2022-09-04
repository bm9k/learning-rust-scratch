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

fn print_board(board: &Board) {
    let mut output = String::new();

    for row in board {
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

fn check_winner(board: &Board) -> Option<Player> {
    // rows


    // columns


    // diagonals

    return Some(Player::X);
    return None;//Some(Player::X);
}

fn main() {
    let mut board: Board = [[Cell::Empty; 3]; 3];

    let mut player = Player::X;

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

    print_board(&board);

    for coord in move_coords {
        let (row, column) = coord;

        println!("Player {} takes {}, {}\n", player.value(), row, column);

        // mark board
        board[row][column] = Cell::Taken(player);

        // swap player
        player = match player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        print_board(&board);

        let winner = check_winner(&board);

        match winner {
            None => continue,
            Some(player) => {
                println!("Player {} wins!", player.value());
                break;
            }
        }
    }
}
