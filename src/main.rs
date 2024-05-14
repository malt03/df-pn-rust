use clap::Parser;
use df_pn::{Board, CheckmateResult};
use std::fs::read_to_string;

#[derive(Parser)]
struct Args {
    /// The board to check.
    /// Examples: https://github.com/malt03/df-pn-rust/blob/main/examples
    board_file: String,

    /// Number of checkmate searches.
    /// If not specified, the search is performed without any limitation in the number of searches.
    #[arg(short, long)]
    num_searches: Option<usize>,

    /// Max depth of the search.
    /// If not specified, the search is performed without any limitation in depth.
    #[arg(short = 'd', long)]
    max_depth: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let body = read_to_string(&args.board_file)
        .expect(format!("failed to read file: {}", args.board_file).as_str());
    let board = Board::parsed(body);

    println!("{board}\n\n=================================\n");
    let result = board.get_checkmate_boards(args.num_searches, args.max_depth.map(|d| d + 2));
    let is_checkmate = result.is_checkmate();
    match result {
        CheckmateResult::Checkmate(boards, count)
        | CheckmateResult::NotCheckmate(boards, count) => {
            println!("\n\n");
            println!("===========================================");
            println!("               best boards");
            println!("===========================================");
            println!("\n\n");
            for (i, board) in boards.into_iter().rev().enumerate() {
                println!(
                    "{}\n\n=================================\n",
                    if i % 2 == 0 { board } else { board.reversed() }
                );
            }
            if is_checkmate {
                println!("checkmate found in {} searches", count);
            } else {
                println!("not checkmate found in {} searches", count);
            }
        }
        CheckmateResult::Unproven => println!("could not prove checkmate nor not checkmate"),
    }
}
