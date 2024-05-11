#[cfg(test)]
mod tests {
    use crate::{
        board::assert_eq_board, Board, Coord, Piece, PieceKind::*, PieceStatus::*, Result,
    };

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
    fn test_create_all_next_boards_king() -> Result<()> {
        let mut b = Board::all_catched();
        b[King][0] = Piece::moved(Coord::new(2, 2), false);
        b.reload_board_map();
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

    #[test]
    fn test_create_all_next_boards_keima_uchi() -> Result<()> {
        let mut b = Board::all_catched();
        b[Keima][0] = Piece::catched(true);
        println!("{}", b);
        b.reload_board_map();
        assert_eq!(b.create_all_next_boards()?.len(), 63);

        Ok(())
    }

    #[test]
    fn test_create_all_next_boards_fu() -> Result<()> {
        let mut b = Board::all_catched();
        b[Fu][0] = Piece::moved(Coord::new(0, 2), false);
        b[Fu][1] = Piece::moved(Coord::new(1, 2), false);
        b.reload_board_map();
        assert_eq_boards(
            b.create_all_next_boards()?,
            [
                "
歩x16 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                           
   ￪歩                     
￪歩                        
                           
                           
                           
                           
                           
                           
------------------
",
                "
歩x16 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                           
￪歩                        
   ￪歩                     
                           
                           
                           
                           
                           
                           
------------------
",
            ],
        );

        Ok(())
    }

    #[test]
    fn test_create_all_next_boards_nari() -> Result<()> {
        let mut b = Board::all_catched();
        b[Fu][0] = Piece::moved(Coord::new(0, 1), false);
        b[Fu][1] = Piece::moved(Coord::new(1, 1), false);
        b.reload_board_map();
        assert_eq_boards(
            b.create_all_next_boards()?,
            [
                "
歩x16 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
   ￪と                     
￪歩                        
                           
                           
                           
                           
                           
                           
                           
------------------
",
                "
歩x16 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
￪と                        
   ￪歩                     
                           
                           
                           
                           
                           
                           
                           
------------------
",
            ],
        );

        Ok(())
    }

    #[test]
    fn test_create_all_next_boards_uchifu() -> Result<()> {
        let mut b = Board::all_catched();
        b[Fu][0] = Piece::moved(Coord::new(0, 1), false);
        b[Fu][1] = Piece::moved(Coord::new(1, 2), false);
        b[Fu][2] = Piece::moved(Coord::new(2, 3), false);
        b[Fu][3] = Piece::moved(Coord::new(3, 4), false);
        b[Fu][4] = Piece::moved(Coord::new(4, 5), false);
        b[Fu][5] = Piece::moved(Coord::new(5, 6), false);
        b[Fu][6] = Piece::moved(Coord::new(6, 7), false);
        b[Fu][7] = Piece::moved(Coord::new(7, 8), false);
        b[Fu][8] = Piece::moved(Coord::new(8, 1), true);
        b[Fu][9] = Piece::init(8, 0, EnemyBoard);
        b[Fu][10] = Piece::catched(true);
        b.reload_board_map();

        assert_eq_board(
            &b,
            "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                     ￪と
   ￪歩                     
      ￪歩                  
         ￪歩               
            ￪歩            
               ￪歩         
                  ￪歩      
                     ￪歩   
------------------
歩",
        );
        assert_eq_boards(
            b.create_all_next_boards()?,
            [
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
￪と                     ￬歩
                        ￪と
   ￪歩                     
      ￪歩                  
         ￪歩               
            ￪歩            
               ￪歩         
                  ￪歩      
                     ￪歩   
------------------
歩",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩￪歩                  ￪と
                           
      ￪歩                  
         ￪歩               
            ￪歩            
               ￪歩         
                  ￪歩      
                     ￪歩   
------------------
歩",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                     ￪と
   ￪歩￪歩                  
                           
         ￪歩               
            ￪歩            
               ￪歩         
                  ￪歩      
                     ￪歩   
------------------
歩",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                     ￪と
   ￪歩                     
      ￪歩￪歩               
                           
            ￪歩            
               ￪歩         
                  ￪歩      
                     ￪歩   
------------------
歩",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                     ￪と
   ￪歩                     
      ￪歩                  
         ￪歩￪歩            
                           
               ￪歩         
                  ￪歩      
                     ￪歩   
------------------
歩",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                     ￪と
   ￪歩                     
      ￪歩                  
         ￪歩               
            ￪歩￪歩         
                           
                  ￪歩      
                     ￪歩   
------------------
歩",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                     ￪と
   ￪歩                     
      ￪歩                  
         ￪歩               
            ￪歩            
               ￪歩￪歩      
                           
                     ￪歩   
------------------
歩",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                     ￪と
   ￪歩                     
      ￪歩                  
         ￪歩               
            ￪歩            
               ￪歩         
                  ￪歩￪歩   
                           
------------------
歩",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￪と
￪歩                        
   ￪歩                     
      ￪歩                  
         ￪歩               
            ￪歩            
               ￪歩         
                  ￪歩      
                     ￪歩   
------------------
歩x2",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                     ￪と￬歩
￪歩                        
   ￪歩                     
      ￪歩                  
         ￪歩               
            ￪歩            
               ￪歩         
                  ￪歩      
                     ￪歩   
------------------
歩",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                  ￪と   
   ￪歩                     
      ￪歩                  
         ￪歩               
            ￪歩            
               ￪歩         
                  ￪歩      
                     ￪歩   
------------------
歩",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                        
   ￪歩                  ￪と
      ￪歩                  
         ￪歩               
            ￪歩            
               ￪歩         
                  ￪歩      
                     ￪歩   
------------------
歩",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                     ￪と
   ￪歩                  ￪歩
      ￪歩                  
         ￪歩               
            ￪歩            
               ￪歩         
                  ￪歩      
                     ￪歩   
------------------
",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                     ￪と
   ￪歩                     
      ￪歩               ￪歩
         ￪歩               
            ￪歩            
               ￪歩         
                  ￪歩      
                     ￪歩   
------------------
",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                     ￪と
   ￪歩                     
      ￪歩                  
         ￪歩            ￪歩
            ￪歩            
               ￪歩         
                  ￪歩      
                     ￪歩   
------------------
",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                     ￪と
   ￪歩                     
      ￪歩                  
         ￪歩               
            ￪歩         ￪歩
               ￪歩         
                  ￪歩      
                     ￪歩   
------------------
",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                     ￪と
   ￪歩                     
      ￪歩                  
         ￪歩               
            ￪歩            
               ￪歩      ￪歩
                  ￪歩      
                     ￪歩   
------------------
",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                     ￪と
   ￪歩                     
      ￪歩                  
         ￪歩               
            ￪歩            
               ￪歩         
                  ￪歩   ￪歩
                     ￪歩   
------------------
",
                "
歩x7 香x4 桂x4 銀x4 金x4 角x2 飛x2 王x2
------------------
                        ￬歩
￪歩                     ￪と
   ￪歩                     
      ￪歩                  
         ￪歩               
            ￪歩            
               ￪歩         
                  ￪歩      
                     ￪歩￪歩
------------------
",
            ],
        );

        Ok(())
    }

    #[test]
    fn test_create_all_next_boards_hisha() -> Result<()> {
        let mut b = Board::all_catched();
        b[Hisha][0] = Piece::moved(Coord::new(1, 1), false);
        b[Hisha][1] = Piece::moved(Coord::new(1, 3), true);
        b.reload_board_map();
        assert_eq_board(
            &b,
            "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
   ￪龍                     
                           
                           
                           
                           
                           
------------------
",
        );
        assert_eq_boards(
            b.create_all_next_boards()?,
            [
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
￪龍                        
                           
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
      ￪龍                  
                           
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
                           
￪龍                        
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
                           
      ￪龍                  
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
      ￪飛                  
                           
   ￪龍                     
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
         ￪飛               
                           
   ￪龍                     
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
            ￪飛            
                           
   ￪龍                     
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
               ￪飛         
                           
   ￪龍                     
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
                  ￪飛      
                           
   ￪龍                     
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
                     ￪飛   
                           
   ￪龍                     
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
                        ￪飛
                           
   ￪龍                     
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
￪飛                        
                           
   ￪龍                     
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
                           
   ￪飛                     
   ￪龍                     
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
   ￪龍                     
                           
                           
   ￪龍                     
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
      ￪龍                  
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
         ￪龍               
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
            ￪龍            
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
               ￪龍         
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
                  ￪龍      
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
                     ￪龍   
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
                        ￪龍
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
￪龍                        
                           
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
                           
   ￪龍                     
                           
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
                           
                           
   ￪龍                     
                           
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
                           
                           
                           
   ￪龍                     
                           
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
                           
                           
                           
                           
   ￪龍                     
                           
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
                           
                           
                           
                           
                           
                           
   ￪龍                     
------------------
",
                "
歩x18 香x4 桂x4 銀x4 金x4 角x2 王x2
------------------
                           
   ￪飛                     
   ￪龍                     
                           
                           
                           
                           
                           
                           
------------------
",
            ],
        );

        Ok(())
    }
}
