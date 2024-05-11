use bpaf::Bpaf;
use df_pn::{Board, Error};
use std::fs::read_to_string;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options)]
struct Args {
    /// The board to check.
    #[bpaf(positional("BOARD_FILE"))]
    board_file: String,
}

fn main() {
    let args = args().run();
    let body = read_to_string(&args.board_file)
        .expect(format!("failed to read file: {}", args.board_file).as_str());
    let board = Board::parse(body);

    println!("{board}\n\n=================================\n");
    match board.get_checkmate_boards(1000000) {
        Ok(Some(boards)) => {
            for (i, board) in boards.into_iter().rev().enumerate() {
                println!(
                    "{}\n\n=================================\n",
                    if i % 2 == 0 { board } else { board.reversed() }
                );
            }
        }
        Ok(None) => println!("no checkmate"),
        Err(e) => match e {
            Error::CatchKing(board) => println!("unexpected catch king: {board}"),
        },
    }
}
