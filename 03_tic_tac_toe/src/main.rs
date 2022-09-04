// [value; length] syntax requires value implements Copy trait
// https://stackoverflow.com/a/29437510
// not sure if this is the best solution
#[derive(Copy, Clone)]
enum Player {
    X,
    O,
}

impl Player {
    fn value(&self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }
}

type Board = [[Option<Player>; 3]; 3];

fn print_board(board: &Board) {
    let mut output = String::new();

    for row in board {
        for cell in row {
            output.push(match cell {
                Some(player) => player.value(),
                // not sure how to explicitly match enum values without nested match
                // e.g.
                // Player::X => 'X',
                // Player::O => 'O',
                None => '-',
            })
        }
        output.push('\n');
    }

    println!("Board:\n{}", output);
}

fn main() {
    let mut board: Board = [[None; 3]; 3];

    let mut player = Player::X;

    let move_coords = [
        (1, 1),
        (0, 0),
        (2, 1),
        (0, 1),
        (0, 2),
        (1, 0),
        (2, 0),
    ];

    print_board(&board);

    for coord in move_coords {
        let (row, column) = coord;

        println!("Player {} takes {}, {}", player.value(), row, column);

        // mark board
        board[row][column] = Some(player);

        // swap player
        player = match player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        print_board(&board);
    }
}
