use clap::Parser;
use df_pn::{Board, Error};
use std::fs::read_to_string;

#[derive(Parser)]
struct Args {
    /// The board to check.
    board_file: String,

    /// Number of checkmate searches.
    #[arg(short, long, default_value_t = 1000000)]
    num_searches: usize,
}

fn main() {
    let args = Args::parse();
    let body = read_to_string(&args.board_file)
        .expect(format!("failed to read file: {}", args.board_file).as_str());
    let board = Board::parsed(body);

    println!("{board}\n\n=================================\n");
    match board.get_checkmate_boards(args.num_searches) {
        Ok(Some((boards, count))) => {
            for (i, board) in boards.into_iter().rev().enumerate() {
                println!(
                    "{}\n\n=================================\n",
                    if i % 2 == 0 { board } else { board.reversed() }
                );
            }
            println!("checkmate found in {} searches", count);
        }
        Ok(None) => println!("no checkmate"),
        Err(e) => match e {
            Error::CatchKing(board) => println!("unexpected catch king: {board}"),
        },
    }
}
