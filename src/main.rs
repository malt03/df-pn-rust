use bpaf::Bpaf;
use df_pn::Board;
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
}
