use crate::{chesspiece::*, game::*};
use std::{fmt, rc::Rc, io::{self, Stdin}};

// ChessBoard struct
pub struct ChessBoard {
    board: Vec<Vec<Option<Rc<dyn ChessPiece>>>>,
    player: bool,
    input: Stdin,
    score: [u8; 2],
    winner: Option<i8>,
    white_captured: String,
    black_captured: String,
    white_en_passant: Option<[usize; 2]>,
    black_en_passant: Option<[usize; 2]>,
    white_can_castle_left: bool,
    white_can_castle_right: bool,
    black_can_castle_left: bool,
    black_can_castle_right: bool,
    white_king_pos: [usize; 2],
    black_king_pos: [usize; 2],
}
// make ChessBoard a game
impl Game for ChessBoard {
    fn get_player(&self) -> bool {
        self.player
    }
    fn set_player(&mut self, player: bool) {
        self.player = player;
    }
    // reset fields
    fn new_game(&mut self) {
        self.player = true;
        self.board = Self::new_board();
        self.score = [0; 2];
        self.winner = None;
        self.white_captured = String::new();
        self.black_captured = String::new();
        self.white_en_passant = None;
        self.black_en_passant = None;
        self.white_can_castle_left = true;
        self.white_can_castle_right = true;
        self.black_can_castle_left = true;
        self.black_can_castle_right = true;
        self.white_king_pos = [4, 0];
        self.black_king_pos = [4, 7];
    }
    // called from game_loop. Represents the course of a turn
    fn take_turn(&mut self) -> bool {
        if self.test_stalemate(self.player) {
            if self.test_checkmate(self.player) { 
                if self.player {
                    self.winner = Some(-1);
                } else {
                    self.winner = Some(1);
                }
            } else {
                self.winner = Some(0)
            }
            return false;
        }
        if self.player {
            print!("\n♔ move: ");
        } else {
            print!("\n♚ move: ");
        }
        println!("eg. \"a1 a2\" or resign");
        if let Some((start, dest)) = self.parse_input() {
            if let Some(piece) = (self.board[start[0]][start[1]]).clone() {
                if !self.test_check(start, dest, self.player)
                && piece.is_white() == self.player {
                    if piece.test_move(start, dest, self) {
                        // if piece is a pawn
                        if piece.get_piece_type() == &PieceType::Pawn {
                            if self.player {
                                if let Some(space) = self.black_en_passant {
                                    self.capture([dest[0], dest[1]-1]);
                                } else if dest[1]-start[1] == 2 {
                                    self.white_en_passant =
                                        Some([dest[0], dest[1]-1]);
                                } else {
                                    self.capture(dest);
                                }
                            } else {
                                if let Some(space) = self.white_en_passant {
                                    self.capture([dest[0], dest[1]+1]);
                                } else if start[1]-dest[1] == 2 {
                                    self.black_en_passant = 
                                        Some([dest[0], dest[1]+1]);
                                } else {
                                    self.capture(dest);
                                }
                            }
                        } else {
                            // If piece is a king
                            if piece.get_piece_type() == &PieceType::King {
                                if dest[0] == 2 {
                                    if self.player && self.white_can_castle_left {
                                        self.board[0][0] = None;
                                        self.board[3][0] = Some(Rc::new(Rook::new(true)));
                                    } else if !self.player && self.black_can_castle_left {
                                        self.board[0][7] = None;
                                        self.board[3][7] = Some(Rc::new(Rook::new(false)));
                                    }
                                } else if dest[0] == 6 {
                                    if self.player && self.white_can_castle_right {
                                        self.board[7][0] = None;
                                        self.board[5][0] = Some(Rc::new(Rook::new(true)));
                                    } else if !self.player && self.black_can_castle_right {
                                        self.board[7][7] = None;
                                        self.board[5][7] = Some(Rc::new(Rook::new(false)));
                                    }
                                }
                                if self.player {
                                    self.white_king_pos = dest;
                                    self.white_can_castle_left = false;
                                    self.white_can_castle_right = false;
                                } else {
                                    self.black_king_pos = dest;
                                    self.black_can_castle_left = false;
                                    self.black_can_castle_right = false;
                                }
                            // if peice is a rook
                            } else if piece.get_piece_type() == &PieceType::Rook {
                                if start[0] == 0 {
                                    if self.player && self.white_can_castle_left {
                                        self.white_can_castle_left = false;
                                    } else if !self.player && self.black_can_castle_left {
                                        self.black_can_castle_left = false;
                                    }
                                } else if start[0] == 7 {
                                    if self.player && self.white_can_castle_right {
                                        self.white_can_castle_right = false;
                                    } else if !self.player && self.black_can_castle_right {
                                        self.black_can_castle_right = false;
                                    }
                                }
                            }
                            self.capture(dest);
                        }
                        self.board[dest[0]][dest[1]] = Some(piece.clone());
                        self.board[start[0]][start[1]] = None;
                        if piece.get_piece_type() == &PieceType::Pawn {
                            if piece.is_white() && dest[1] == 7 {
                                self.upgrade_pawn(dest, true);
                            } else if dest[1] == 0 {
                                self.upgrade_pawn(dest, false);
                            }
                        }
                        if self.player && self.black_en_passant.is_some() {
                            self.black_en_passant = None;
                        } else if !self.player && self.white_en_passant.is_some() {
                            self.white_en_passant = None;
                        }
                        return true;
                    }
                }
            }
        }
        false
    }
    fn print_game(&self) {
        print!("{}[2J", 27 as char);
        println!("{}", self.get_score());
        println!("{}", self);
    }
    fn get_score(&self) -> String {
        format!("white score: {} \n{}\nblack score: {}\n{}", 
                self.score[0], self.white_captured,
                self.score[1], self.black_captured)
    }
    fn get_winner(&self) -> Option<i8> {
        self.winner
    }
    fn end(&self, winner: i8) {
        if winner > 0 {
            println!("White has won!");
        } else if winner < 0 {
            println!("Black has won!");
        } else {
            println!("Draw!");
        }
    }
}
// allow ChessBoard to be printed
impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (0..8).rev() {
            write!(f, "{}[0;m{} ", 27 as char, row+1)?;
            for col in 0..8 {
                if (col+row)%2 == 0 {
                    write!(f, "{}[40;102m", 27 as char)?;
                } else {
                    write!(f, "{}[0;m", 27 as char)?;
                }
                if let Some(chess_piece) = &self.board[col][row] {
                    write!(f, " {} ", chess_piece)?;
                } else {
                    write!(f, "   ")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "{}[0;m   a  b  c  d  e  f  g  h", 27 as char)
    }
}
impl ChessBoard {
    // constructor
    pub fn new() -> Self {
        ChessBoard {
            board: Self::new_board(),
            player: true,
            input: io::stdin(),
            score: [0; 2],
            winner: None,
            white_captured: String::new(),
            black_captured: String::new(),
            white_en_passant: None,
            black_en_passant: None,
            white_can_castle_left: true,
            white_can_castle_right: true,
            black_can_castle_left: true,
            black_can_castle_right: true,
            white_king_pos: [4, 0],
            black_king_pos: [4, 7],
        }
    }
    // test a move and see (regarless of actual legality) if it will put
    // the current player's king in check
    pub fn test_check(
        &mut self, 
        start: [usize; 2], 
        dest: [usize; 2], 
        is_white: bool) -> bool {
        let start_piece = self.board[start[0]][start[1]].clone();
        let dest_piece = self.board[dest[0]][dest[1]].clone();
        self.board[dest[0]][dest[1]] = start_piece.clone();
        self.board[start[0]][start[1]] = None;
        // we need to check a different position if it's the king that moved
        let king_pos = if let Some(piece) = start_piece.clone() {
            if piece.get_piece_type() == &PieceType::King {
                dest
            } else {
                if is_white {
                    self.white_king_pos
                } else {
                    self.black_king_pos
                }
            }
        } else {
            if is_white {
                self.white_king_pos
            } else {
                self.black_king_pos
            }
        };
        let in_check = self.is_threatened(king_pos, is_white);
        self.board[start[0]][start[1]] = start_piece;
        self.board[dest[0]][dest[1]] = dest_piece;
        in_check
    }
    // test checkmate on one of the two kings
    pub fn test_checkmate(&mut self, is_white: bool) -> bool {
        let king_pos = if is_white {
            self.white_king_pos
        } else {
            self.black_king_pos
        };
        if self.is_threatened(king_pos, is_white) {
            return true;
        }
        false
    }
    // test if the king is in stalemate
    pub fn test_stalemate(&mut self, is_white: bool) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board[col][row].clone() {
                    if piece.is_white() == is_white {
                        if self.test_block([col, row], piece) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
    // test if a piece can block check for resolving checkmate
    fn test_block(&mut self, start: [usize; 2], piece: Rc<dyn ChessPiece>) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                if piece.test_move(start, [col, row], self) {
                    if !self.test_check(start, [col, row], piece.is_white()) {
                        return true;
                    }
                }
            }
        }
        false
    }
    // capture a piece (remove it from board and increment score)
    pub fn capture(&mut self, space: [usize; 2]) {
        if let Some(piece) = &self.board[space[0]][space[1]] {
            if piece.is_white() {
                self.score[1] += piece.get_points();
                self.black_captured.push_str(piece.as_str());
            } else {
                self.score[0] += piece.get_points();
                self.white_captured.push_str(piece.as_str());
            }
            self.board[space[0]][space[1]] = None;
        }
    }
    // check if a square is threatened
    pub fn is_threatened(&mut self, space: [usize; 2], is_white: bool) -> bool {
        // Pawns won't return that they can move to a square they threaten unless
        // there is a piece on it, so we create a "dummy" piece and then delete it
        let space_is_empty = if self.board[space[0]][space[1]].is_none() {
            self.board[space[0]][space[1]] = Some(Rc::new(Pawn::new(is_white)));
            true
        } else {
            false
        };
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board[col][row].clone() {
                    if piece.is_white() != is_white {
                        if piece.test_move([col, row], space, self) {
                            if space_is_empty {
                                self.board[space[0]][space[1]] = None;
                            }
                            return true;
                        }
                    }
                }
            }
        }
        if space_is_empty {
            self.board[space[0]][space[1]] = None;
        }
        false
    }
    // accessors and mutators for king positions
    pub fn get_white_king_pos(&self) -> [usize; 2] {
        self.white_king_pos
    }
    pub fn get_black_king_pos(&self) -> [usize; 2] {
        self.black_king_pos
    }
    pub fn set_white_king_pos(&mut self, pos: [usize; 2]) {
        self.white_king_pos = pos;
    }
    pub fn set_black_king_pos(&mut self, pos: [usize; 2]) {
        self.black_king_pos = pos;
    }
    // accessors and mutators for en passant
    pub fn get_white_en_passant(&self) -> Option<[usize; 2]> {
        self.white_en_passant
    }
    pub fn get_black_en_passant(&self) -> Option<[usize; 2]> {
        self.black_en_passant
    }
    pub fn set_white_en_passant(&mut self, space: Option<[usize; 2]>) {
        self.white_en_passant = space;
    }
    pub fn set_black_en_passant(&mut self, space: Option<[usize; 2]>) {
        self.black_en_passant = space;
    }
    // accessors and mutators for castling
    pub fn get_white_can_castle_left(&self) -> bool {
        self.white_can_castle_left
    }
    pub fn get_white_can_castle_right(&self) -> bool {
        self.white_can_castle_right
    }
    pub fn get_black_can_castle_left(&self) -> bool {
        self.black_can_castle_left
    }
    pub fn get_black_can_castle_right(&self) -> bool {
        self.black_can_castle_right
    }
    // return board
    pub fn get_board(&self) -> &Vec<Vec<Option<Rc<dyn ChessPiece>>>> {
        &self.board
    }
    // set a piece at the given position
    pub fn set(&mut self, pos: [usize; 2], piece: Option<Rc<dyn ChessPiece>>) {
        self.board[pos[0]][pos[1]] = piece;
    }
    // turn a pawn into a different piece (or don't)
    pub fn upgrade_pawn(&mut self, dest: [usize; 2], is_white: bool) {
        loop {
            let mut input_string = String::new();
            self.print_game();
            if is_white {
                println!("\n1:♕ 2:♖ 3:♗ 4:♘");
            } else {
                println!("\n1:♛ 2:♜ 3:♝ 4:♞");
            }
            self.input.read_line(&mut input_string);
            match input_string.trim() {
                "1" => {
                    self.board[dest[0]][dest[1]] = 
                        Some(Rc::new(Queen::new(is_white)));
                    return;
                },
                "2" => {
                    self.board[dest[0]][dest[1]] =
                        Some(Rc::new(Rook::new(is_white)));
                    return;
                },
                "3" => {
                    self.board[dest[0]][dest[1]] =
                        Some(Rc::new(Bishop::new(is_white)));
                    return;
                },
                "4" => {
                    self.board[dest[0]][dest[1]] =
                        Some(Rc::new(Knight::new(is_white)));
                    return;
                },
                _ => {},
            }
        }
    }
    // read input
    pub fn parse_input(&mut self) -> Option<([usize; 2], [usize; 2])> {
        let mut input_string = String::new();
        self.input.read_line(&mut input_string)
            .expect("failed to read from stdin");
        if input_string.trim() == "resign" {
            if self.player {
                self.winner = Some(-1);
            } else {
                self.winner = Some(1);
            }
            return None;
        }
        let input_split: Vec<&str> = input_string.trim().split(" ").collect();
        if input_split.len() != 2 {
            return None;
        }
        let start_bytes: Vec<u8> = input_split[0].bytes().collect();
        let dest_bytes: Vec<u8> = input_split[1].bytes().collect();
        if start_bytes.len() != 2 || dest_bytes.len() != 2 {
            return None;
        }
        let start = [start_bytes[0] as i32 -97, start_bytes[1] as i32 -49];
        let dest = [dest_bytes[0] as i32 -97, dest_bytes[1] as i32 -49];
        if start[0] > 7 || start[0] < 0 || start[1] > 7 || start[1] < 0 
        || dest[0] > 7 || dest[0] < 0 || dest[1] > 7 || dest[1] < 0 {
            return None;
        }
        Some((
            [start[0] as usize, start[1] as usize], 
            [dest[0] as usize, dest[1] as usize]))
    }
    // make a new board
    fn new_board() -> Vec<Vec<Option<Rc<dyn ChessPiece>>>> {
        let mut board: Vec<Vec<Option<Rc<dyn ChessPiece>>>> = vec![vec![None; 8]; 8]; 
        for col in 0..8 {
            // White Pawns
            board[col][1] = Some(Rc::new(Pawn::new(true)));
            // Black Pawns
            board[col][6] = Some(Rc::new(Pawn::new(false)));
        }
        // White Rooks
        board[0][0] = Some(Rc::new(Rook::new(true)));
        board[7][0] = Some(Rc::new(Rook::new(true)));
        // Black Rooks
        board[0][7] = Some(Rc::new(Rook::new(false)));
        board[7][7] = Some(Rc::new(Rook::new(false)));
        // White Knights
        board[1][0] = Some(Rc::new(Knight::new(true)));
        board[6][0] = Some(Rc::new(Knight::new(true)));
        // Black Knights
        board[1][7] = Some(Rc::new(Knight::new(false)));
        board[6][7] = Some(Rc::new(Knight::new(false)));
        // White Bishops
        board[2][0] = Some(Rc::new(Bishop::new(true)));
        board[5][0] = Some(Rc::new(Bishop::new(true)));
        // Black Bishops
        board[2][7] = Some(Rc::new(Bishop::new(false)));
        board[5][7] = Some(Rc::new(Bishop::new(false)));
        // White King and Queen
        board[4][0] = Some(Rc::new(King::new(true)));
        board[3][0] = Some(Rc::new(Queen::new(true)));
        // Black King and Queen
        board[4][7] = Some(Rc::new(King::new(false)));
        board[3][7] = Some(Rc::new(Queen::new(false)));
        // return
        board
    }
}
