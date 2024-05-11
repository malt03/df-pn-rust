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

    println!("{board}");
    match board.get_checkmate_board(1000000) {
        Ok(Some(board)) => println!("{board}"),
        Ok(None) => println!("no checkmate"),
        Err(e) => match e {
            Error::CatchKing(board) => println!("catch king: {board}"),
        },
    }
}
