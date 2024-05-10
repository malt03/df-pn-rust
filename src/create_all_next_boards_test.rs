#[cfg(test)]
mod tests {
    use crate::{board::assert_eq_board, Board, Coord, Piece, PieceKind::*, Result};

    use std::collections::HashSet;

    fn assert_eq_boards<T, I>(left: Vec<(Board, T)>, right: I)
    where
        I: IntoIterator<Item = &'static str>,
    {
        let right_set = HashSet::from_iter(
            right
                .into_iter()
                .map(|s| s.strip_prefix("\n").unwrap().to_string()),
        );
        assert_eq!(left.len(), right_set.len());
        let mut left_outputs = HashSet::new();
        for (b, _) in left {
            let mut output = String::new();
            b.dump_to(&mut output, false).unwrap();
            left_outputs.insert(output);
        }
        assert_eq!(left_outputs, right_set);
    }

    #[test]
    fn test_create_all_next_boards() -> Result<()> {
        let all_catched = Board::all_catched();

        let mut b = all_catched.clone();
        b[King][0] = Piece::moved(Coord::new(2, 2), false);
        b.reload_board_map();
        assert_eq_board(
            &b,
            "
歩x18 香x4 桂x4 銀x4 金x4 角x2 飛x2 王
------------------
                           
                           
      ￪王                  
                           
                           
                           
                           
                           
                           
------------------
",
        );
        assert_eq_boards(
            b.create_all_next_boards()?,
            [
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 飛x2 王
------------------
                           
   ￪王                     
                           
                           
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 飛x2 王
------------------
                           
      ￪王                  
                           
                           
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 飛x2 王
------------------
                           
         ￪王               
                           
                           
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 飛x2 王
------------------
                           
                           
   ￪王                     
                           
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 飛x2 王
------------------
                           
                           
         ￪王               
                           
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 飛x2 王
------------------
                           
                           
                           
   ￪王                     
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 飛x2 王
------------------
                           
                           
                           
      ￪王                  
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 飛x2 王
------------------
                           
                           
                           
         ￪王               
                           
                           
                           
                           
                           
------------------
",
            ],
        );

        Ok(())
    }
}
