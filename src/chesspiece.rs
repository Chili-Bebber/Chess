use std::fmt;
use std::iter::Zip;
use std::vec::IntoIter;
use crate::board::ChessBoard;

#[derive(PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

// Chess Piece trait
pub trait ChessPiece {
    fn test_move(
        &self,
        start: [usize; 2],
        dest: [usize; 2],
        board: &mut ChessBoard) -> bool {
        let dest_player = if let Some(piece) = &board.get_board()[dest[0]][dest[1]] {
            Some(piece.is_white())
        } else {
            None
        };
        if start != dest && !check_occupied(self.is_white(), dest_player) {
            return self.test_space(start, dest, dest_player, board);
        }
        false
    }
    fn test_space(
        &self,
        start: [usize; 2],
        dest: [usize; 2],
        dest_player: Option<bool>,
        board: &mut ChessBoard) -> bool;
    fn get_points(&self) -> u8;
    fn get_piece_type(&self) -> &PieceType;
    fn is_white(&self) -> bool;
    fn get_symbol(&self, index: usize) -> &'static str;
    fn as_str(&self) -> &'static str {
        if self.is_white() {
            return self.get_symbol(0);
        }
        self.get_symbol(1)
    }
}
// Diagonal Move
pub trait DiagonalMove : ChessPiece {
    fn test_diagonal(
        &self,
        start: [usize; 2],
        dest: [usize; 2],
        board: &mut ChessBoard) -> bool {
        let (x_dist, y_dist) = find_dist(start, dest);
        if x_dist.abs() == y_dist.abs() {
            for (x, y) in path_iter(start, dest) {
                if board.get_board()[x][y].is_some() {
                    return false;
                }
            }
            return true;
        }
        false
    }
}
// Straight Move
pub trait StraightMove : ChessPiece {
    fn test_straight(
        &self,
        start: [usize; 2],
        dest: [usize; 2],
        board: &mut ChessBoard) -> bool {
        if start[0] == dest[0] || start[1] == dest[1] {
            for (x, y) in path_iter(start, dest) {
                if board.get_board()[x][y].is_some() {
                    return false;
                }
            }
            return true;
        }
        false
    }
}
// Knight Move
pub trait KnightMove : ChessPiece {
    fn test_knight(
        &self,
        start: [usize; 2],
        dest: [usize; 2]) -> bool {
        let (x_dist, y_dist) = find_dist(start, dest);
        if x_dist.abs() == 2 {
            if y_dist.abs() == 1 {
                return true;
            }
        } else if y_dist.abs() == 2 {
            if x_dist.abs() == 1 {
                return true;
            }
        }
        false
    }
}
// King Move
pub trait KingMove : ChessPiece {
    fn test_king(
        &self,
        start: [usize; 2],
        dest: [usize; 2],
        board: &mut ChessBoard) -> bool {
        let (x_dist, y_dist) = find_dist(start, dest);
        if x_dist.abs() <= 1 && y_dist.abs() <= 1 {
            return true;
        // if player is trying to castle
        } else if x_dist.abs() == 2 && y_dist == 0 
        && !board.is_threatened(start, self.is_white()) {
            let (can_castle_left, can_castle_right) = if self.is_white() {
                (board.get_white_can_castle_left(), board.get_white_can_castle_right())
            } else {
                (board.get_black_can_castle_left(), board.get_black_can_castle_right())
            };
            // if player is trying to castle left
            if dest[0] == 2 && can_castle_left 
            && board.get_board()[3][dest[1]].is_none()
            && board.get_board()[2][dest[1]].is_none()
            && board.get_board()[1][dest[1]].is_none()
            && !board.test_check(start, [3, dest[1]], self.is_white()) {
                let rook = board.get_board()[0][dest[1]].clone();
                board.set([0, dest[1]], None);
                board.set([3, dest[1]], rook.clone());
                if !board.test_check(start, dest, self.is_white()) {
                    board.set([0, dest[1]], rook);
                    board.set([3, dest[1]], None);
                    return true;
                }
                board.set([0, dest[1]], rook);
                board.set([3, dest[1]], None);
            // if player is trying to castle right
            } else if dest[0] == 6 && can_castle_right 
            && board.get_board()[5][dest[1]].is_none()
            && board.get_board()[6][dest[1]].is_none()
            && !board.test_check(start, [dest[0]-1, dest[1]], self.is_white()) {
                let rook = board.get_board()[7][dest[1]].clone();
                board.set([7, dest[1]], None);
                board.set([5, dest[1]], rook.clone());
                if !board.test_check(start, dest, self.is_white()) {
                    board.set([7, dest[1]], rook);
                    board.set([5, dest[1]], None);
                    return true;
                }
                board.set([7, dest[1]], rook);
                board.set([5, dest[1]], None);
            }
        }
        false
    }
}
// Pawn Move
pub trait PawnMove : ChessPiece {
    fn test_pawn(
        &self,
        start: [usize; 2],
        dest: [usize; 2],
        dest_player: Option<bool>,
        board: &mut ChessBoard) -> bool {
        if start[0] == dest[0] {
            if let Some(_player) = dest_player {
                return false;
            }
        }
        let direction = if self.is_white() {
            -1
        } else {
            1
        };
        let (x_dist, y_dist) = find_dist(start, dest);
        let y_diff = start[1] as i32 - dest[1] as i32;
        if y_diff/direction == 1 {
            if x_dist.abs() == 1 {
                // if the square is occupied
                if dest_player.is_some() {
                    return true;
                // if the square is empty
                } else {
                    let en_passant_space = if self.is_white() {
                        board.get_black_en_passant()
                    } else {
                        board.get_white_en_passant()
                    };
                    if let Some(space) = en_passant_space {
                        if dest == space {
                            return true;
                        }
                    }
                }
            } else if x_dist == 0 {
                return true;
            }
        } else if y_diff/direction == 2 && (start[1] == 1 || start[1] == 6) {
            let capture_point = [dest[0], (dest[1] as i32 + direction) as usize];
            let x = capture_point[0];
            let y = capture_point[1];
            if !dest_player.is_some() && x_dist == 0 
            && !board.get_board()[x][y].is_some() {
                return true;
            }
        }
        false
    }
}

fn check_occupied(player: bool, dest_player: Option<bool>) -> bool {
    if let Some(dest_player) = dest_player {
        if player == dest_player {
            return true;
        }
    }
    false
}

// Pawn struct
pub struct Pawn {
    is_white: bool,
    symbols: [&'static str; 2],
    piece_type: PieceType,
}
impl Pawn {
    pub fn new(is_white: bool) -> Self {
        Pawn {
            is_white: is_white,
            symbols: ["♙", "♟"],
            piece_type: PieceType::Pawn,
        }
    }
}
impl PawnMove for Pawn {}
impl ChessPiece for Pawn {
    fn is_white(&self) -> bool {
        self.is_white
    }
    fn test_space(
        &self,
        start: [usize; 2],
        dest: [usize; 2],
        dest_player: Option<bool>,
        board: &mut ChessBoard) -> bool {
        self.test_pawn(start, dest, dest_player, board)
    }
    fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }
    fn get_points(&self) -> u8 {
        1
    }
    fn get_symbol(&self, index: usize) -> &'static str {
        self.symbols[index]
    }
}

// Bishop Struct
pub struct Bishop {
    is_white: bool,
    symbols: [&'static str; 2],
    piece_type: PieceType,
}
impl Bishop {
    pub fn new(is_white: bool) -> Self {
        Bishop {
            is_white: is_white,
            symbols: ["♗", "♝"],
            piece_type: PieceType::Bishop,
        }
    }
}
impl DiagonalMove for Bishop {}
impl ChessPiece for Bishop {
    fn is_white(&self) -> bool {
        self.is_white
    }
    fn test_space(
        &self,
        start: [usize; 2],
        dest: [usize; 2],
        dest_player: Option<bool>,
        board: &mut ChessBoard) -> bool {
        self.test_diagonal(start, dest, board)
    }
    fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }
    fn get_points(&self) -> u8 {
        3
    }
    fn get_symbol(&self, index: usize) -> &'static str {
        self.symbols[index]
    }
}
// Knight Struct
pub struct Knight {
    is_white: bool,
    symbols: [&'static str; 2],
    piece_type: PieceType,
}
impl Knight {
    pub fn new(is_white: bool) -> Self {
        Knight {
            is_white: is_white,
            symbols: ["♘", "♞"],
            piece_type: PieceType::Knight,
        }
    }
}
impl KnightMove for Knight {}
impl ChessPiece for Knight {
    fn is_white(&self) -> bool {
        self.is_white
    }
    fn test_space(
        &self,
        start: [usize; 2],
        dest: [usize; 2],
        dest_player: Option<bool>,
        _board: &mut ChessBoard) -> bool {
        self.test_knight(start, dest)
    }
    fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }
    fn get_points(&self) -> u8 {
        3
    }
    fn get_symbol(&self, index: usize) -> &'static str {
        self.symbols[index]
    }
}
// Rook Struct
pub struct Rook {
    is_white: bool,
    symbols: [&'static str; 2],
    piece_type: PieceType,
}
impl Rook {
    pub fn new(is_white: bool) -> Self {
        Rook {
            is_white: is_white,
            symbols: ["♖", "♜"],
            piece_type: PieceType::Rook,
        }
    }
}
impl StraightMove for Rook {}
impl ChessPiece for Rook {
    fn is_white(&self) -> bool {
        self.is_white
    }
    fn test_space(
        &self,
        start: [usize; 2],
        dest: [usize; 2],
        dest_player: Option<bool>,
        board: &mut ChessBoard) -> bool {
        self.test_straight(start, dest, board)
    }
    fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }
    fn get_points(&self) -> u8 {
        5
    }
    fn get_symbol(&self, index: usize) -> &'static str {
        self.symbols[index]
    }
}
// King Struct
pub struct King {
    is_white: bool,
    symbols: [&'static str; 2],
    piece_type: PieceType,
}
impl King {
    pub fn new(is_white: bool) -> Self {
        King {
            is_white: is_white,
            symbols: ["♔", "♚"],
            piece_type: PieceType::King,
        }
    }
}
impl KingMove for King {}
impl ChessPiece for King {
    fn is_white(&self) -> bool {
        self.is_white
    }
    fn test_space(
        &self,
        start: [usize; 2],
        dest: [usize; 2],
        dest_player: Option<bool>,
        board: &mut ChessBoard) -> bool {
        self.test_king(start, dest, board)
    }
    fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }
    fn get_points(&self) -> u8 {
        0
    }
    fn get_symbol(&self, index: usize) -> &'static str {
        self.symbols[index]
    }
}
// Queen Struct
pub struct Queen {
    is_white: bool,
    symbols: [&'static str; 2],
    piece_type: PieceType,
}
impl Queen {
    pub fn new(is_white: bool) -> Self {
        Queen {
            is_white: is_white,
            symbols: ["♕", "♛"],
            piece_type: PieceType::Queen,
        }
    }
}
impl DiagonalMove for Queen {}
impl StraightMove for Queen {}
impl ChessPiece for Queen {
    fn is_white(&self) -> bool {
        self.is_white
    }
    fn test_space(
        &self,
        start: [usize; 2],
        dest: [usize; 2],
        dest_player: Option<bool>,
        board: &mut ChessBoard) -> bool {
        self.test_straight(start, dest, board) || self.test_diagonal(start, dest, board)
    }
    fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }
    fn get_points(&self) -> u8 {
        9
    }
    fn get_symbol(&self, index: usize) -> &'static str {
        self.symbols[index]
    }
}
// Display trait for ChessPieces
impl fmt::Display for dyn ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
// calculate distance
fn find_dist(start: [usize; 2], dest: [usize; 2]) -> (i32, i32) {
    let x_dist = start[0] as i32 - dest[0] as i32;
    let y_dist = start[1] as i32 - dest[1] as i32;
    (x_dist, y_dist)
}
// make an iterator for traversing a piece's path
fn path_iter(start: [usize; 2], dest: [usize; 2]) -> Zip<IntoIter<usize>, IntoIter<usize>> {
    let (mut x_iter, x_forwards) = if start[0] < dest[0] {
        ((start[0]+1..dest[0]).collect(), true)
    } else if dest[0] < start[0] {
        ((dest[0]+1..start[0]).collect(), false)
    } else {
        (vec![start[0]; find_dist(start, dest).1.abs() as usize], true)
    };
    let (y_iter, y_forwards) = if start[1] < dest[1] {
        ((start[1]+1..dest[1]).collect(), true)
    } else if dest[1] < start[1] {
        ((dest[1]+1..start[1]).collect(), false)
    } else {
        (vec![start[1]; find_dist(start, dest).0.abs() as usize], true)
    };
    if x_forwards != y_forwards {
       x_iter.reverse(); 
    }
    x_iter.into_iter().zip(y_iter.into_iter())
}
