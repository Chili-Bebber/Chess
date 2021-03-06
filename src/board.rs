extern crate dirs;

use crate::{chesspiece::*, game::*};
use std::{fs::{self, File}, fmt, rc::Rc, io::{self, Stdin, prelude::*}, path::PathBuf};
use godot::{
    init::{PropertyHint, PropertyUsage, SignalArgument, Signal, ClassBuilder},
    user_data::MutexData,
    Node, Int32Array, Variant, GodotString, Vector2, NodePath, StringArray
};

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
    game_save: String,
    turn_num: usize,
}

unsafe impl Send for ChessBoard {}

impl godot::NativeClass for ChessBoard {
    type Base = Node;
    type UserData = MutexData<ChessBoard>;
    fn class_name() -> &'static str {
        "ChessBoard"
    }
    fn init(owner: Self::Base) -> Self {
        Self::new()
    }
    // set up all the signals
    fn register_properties(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "game_over",
            args: &[SignalArgument {
                name: "winner",
                default: Variant::from_i64(0),
                hint: PropertyHint::None,
                usage: PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(Signal {
            name: "log_update",
            args: &[SignalArgument {
                name: "entry",
                default: Variant::from_str(""),
                hint: PropertyHint::None,
                usage: PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(Signal {
            name: "score_update",
            args: &[SignalArgument {
                name: "score",
                default: Variant::from_str(""),
                hint: PropertyHint::None,
                usage: PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(Signal {
            name: "move_is_legal",
            args: &[SignalArgument {
                name: "is_legal",
                default: Variant::from_bool(false),
                hint: PropertyHint::None,
                usage: PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(Signal {
            name: "piece_captured",
            args: &[SignalArgument {
                name: "position",
                default: Variant::from_vector2(&Vector2::zero()),
                hint: PropertyHint::None,
                usage: PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(Signal {
            name: "castle",
            args: &[SignalArgument {
                name: "position",
                default: Variant::from_vector2(&Vector2::zero()),
                hint: PropertyHint::None,
                usage: PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(Signal {
            name: "pawn_promoted",
            args: &[SignalArgument {
                name: "position",
                default: Variant::from_vector2(&Vector2::zero()),
                hint: PropertyHint::None,
                usage: PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(Signal {
            name: "save_loaded",
            args: &[SignalArgument {
                name: "board_state",
                default: Variant::from_str(""),
                hint: PropertyHint::None,
                usage: PropertyUsage::DEFAULT,
            }],
        });
        // currently unused
        builder.add_signal(Signal {
            name: "error",
            args: &[SignalArgument {
                name: "message",
                default: Variant::from_str(""),
                hint: PropertyHint::None,
                usage: PropertyUsage::DEFAULT,
            }],
        });
        // not self explanatory. This signal tells godot to
        // refresh how many moves are loadable by the user
        builder.add_signal(Signal {
            name: "update_moves",
            args: &[SignalArgument {
                name: "turns",
                default: Variant::from_i64(0),
                hint: PropertyHint::None,
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }
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
        self.turn_num = 0;
    }
    // called from godot whenever the player does something
    // represents the course of a turn
    unsafe fn take_turn(&mut self, mut owner: Node, start: [usize; 2], dest: [usize;2]) -> bool {
        if let Some(piece) = (self.board[start[0]][start[1]]).clone() {
            let mut pawn_promoted = false;
            if !self.test_check(start, dest, self.player)
            && piece.is_white() == self.player {
                //let save_clone = self.game_save.clone();
                if piece.test_move(start, dest, self) {
                    let mut save_vec: Vec<&str> = self.game_save.lines().collect(); 
                    // truncate save to current move after a move is made
                    if save_vec.len()/17 > self.turn_num {
                        let truncated_save_iter = save_vec.drain(0..self.turn_num*17);
                        let truncated_save = truncated_save_iter.fold(
                            String::new(), |mut save, line| {
                                save.push_str(&format!("{}\n", line));
                                save
                            }
                        );
                        self.game_save = truncated_save;
                    }
                    self.turn_num += 1;
                    let move_string = format!("{}: {} {}{} → {}{}",
                                              self.turn_num,
                                              piece,
                                              (start[0]+97) as u8 as char,
                                              start[1]+1,
                                              (dest[0]+97) as u8 as char,
                                              dest[1]+1);
                    owner.emit_signal(
                        GodotString::from_str("log_update"),
                        &[Variant::from_str(&move_string)]);
                    // we need to store the string representation of any captures that happen and
                    // then write them later because of how pawn promotion is out of sync with the
                    // rust turn order.
                    let mut capture_string = String::new();
                    // if piece is a pawn
                    if piece.get_piece_type() == &PieceType::Pawn {
                        if self.player {
                            if self.black_en_passant.is_some() && dest == self.black_en_passant.unwrap() {
                                capture_string = self.capture(owner, [dest[0], dest[1]-1]);
                            } else if dest[1]-start[1] == 2 {
                                self.white_en_passant = Some([dest[0], dest[1]-1]);
                            } else {
                                capture_string = self.capture(owner, dest);
                            }
                        } else {
                            if self.white_en_passant.is_some() && dest == self.white_en_passant.unwrap() {
                                capture_string = self.capture(owner, [dest[0], dest[1]+1]);
                            } else if start[1]-dest[1] == 2 {
                                self.black_en_passant = Some([dest[0], dest[1]+1]);
                            } else {
                                capture_string = self.capture(owner, dest);
                            }
                        }
                    } else {
                        // If piece is a king
                        if piece.get_piece_type() == &PieceType::King {
                            if dest[0] == 2 {
                                if self.player && self.white_can_castle_left {
                                    owner.emit_signal(
                                        GodotString::from_str("castle"),
                                        &[Variant::from_vector2(&Vector2::new(0.0, 0.0))]);
                                    self.board[0][0] = None;
                                    self.board[3][0] = Some(Rc::new(Rook::new(true)));
                                } else if !self.player && self.black_can_castle_left {
                                    owner.emit_signal(
                                        GodotString::from_str("castle"),
                                        &[Variant::from_vector2(&Vector2::new(0.0, 7.0))]);
                                    self.board[0][7] = None;
                                    self.board[3][7] = Some(Rc::new(Rook::new(false)));
                                }
                            } else if dest[0] == 6 {
                                if self.player && self.white_can_castle_right {
                                    owner.emit_signal(
                                        GodotString::from_str("castle"),
                                        &[Variant::from_vector2(&Vector2::new(7.0, 0.0))]);
                                    self.board[7][0] = None;
                                    self.board[5][0] = Some(Rc::new(Rook::new(true)));
                                } else if !self.player && self.black_can_castle_right {
                                    owner.emit_signal(
                                        GodotString::from_str("castle"),
                                        &[Variant::from_vector2(&Vector2::new(7.0, 7.0))]);
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
                        // if piece is a rook
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
                        capture_string = self.capture(owner, dest);
                    }
                    self.board[dest[0]][dest[1]] = Some(piece.clone());
                    self.board[start[0]][start[1]] = None;
                    if piece.get_piece_type() == &PieceType::Pawn 
                    && (dest[1] == 0 || dest[1] == 7) {
                        pawn_promoted = true;
                        self.upgrade_pawn_signal(owner, dest);
                    }
                    if self.player && self.black_en_passant.is_some() {
                        self.black_en_passant = None;
                    } else if !self.player && self.white_en_passant.is_some() {
                        self.white_en_passant = None;
                    }
                    // save file stuff
                    // score
                    self.game_save.push_str(&self.get_score());
                    // white's allowed castling directions
                    let white_castle_str = match (self.white_can_castle_left, 
                                                  self.white_can_castle_right) {
                        (true, true) => "b\n",
                        (true, false) => "l\n",
                        (false, true) => "r\n",
                        _ => "\n",
                    };
                    self.game_save.push_str(white_castle_str);
                    // black's allowed castling directions
                    let black_castle_str = match (self.black_can_castle_left,
                                                  self.black_can_castle_right) {
                        (true, true) => "b\n",
                        (true, false) => "l\n",
                        (false, true) => "r\n",
                        _ => "\n",
                    };
                    self.game_save.push_str(black_castle_str);
                    // white's en passant col
                    let white_en_passant_string = match (self.white_en_passant) {
                        Some(space) => format!("{}\n", space[0]),
                        None => String::from("\n"), 
                    };
                    // black's en passant col
                    self.game_save.push_str(&white_en_passant_string);
                    let black_en_passant_string = match (self.black_en_passant) {
                        Some(space) => format!("{}\n", space[0]),
                        None => String::from("\n"),
                    };
                    self.game_save.push_str(&black_en_passant_string);
                    // most recent move
                    self.game_save.push_str(&move_string);
                    self.game_save.push_str(&capture_string);
                    // tell godot to refresh the list of moves
                    owner.emit_signal(
                        GodotString::from_str("update_moves"),
                        &[Variant::from_i64(self.turn_num as i64)]);
                    if !pawn_promoted {
                        self.test_checkmate_stalemate(owner);
                        owner.emit_signal(
                            GodotString::from_str("log_update"),
                            &[Variant::from_str("\n")]);
                        self.game_save.push_str("\n");
                        // the board
                        self.game_save.push_str(&format!("{}", self));
                    }
                    return true;
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
        format!("White Score: {} \n{}\nBlack Score: {}\n{}\n", 
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
// allow ChessBoard to be represented as a string.
// used to be for displaying the board, but now it's used
// for saving the game into a file to be loaded later
impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(chess_piece) = &self.board[col][row] {
                    write!(f, "{}", chess_piece)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}
#[methods]
impl ChessBoard {
    // test for checkmate and stalemate and tell godot if anything was detected
    unsafe fn test_checkmate_stalemate(&mut self, mut owner: Node) {
        if self.test_stalemate(!self.player) {
            if self.test_checkmate(!self.player) { 
                if self.player {
                    owner.emit_signal(
                        GodotString::from_str("game_over"),
                        &[Variant::from_i64(1)]);
                } else {
                    owner.emit_signal(
                        GodotString::from_str("game_over"),
                        &[Variant::from_i64(-1)]);
                }
                self.game_save.push_str(" checkmate");
            } else {
                owner.emit_signal(
                    GodotString::from_str("game_over"),
                    &[Variant::from_i64(0)]);
                self.game_save.push_str(" stalemate");
            }
        }
    }
    // just a call to new_game() that is  exposed to godot
    #[export]
    fn reset_game(&mut self, owner: Node) {
        self.new_game();
    }
    // movement method, but this one is exposed to godot and emits signals
    #[export]
    unsafe fn try_move(&mut self, owner: Node, start: Int32Array, dest: Int32Array) {
        let start = [start.get(0) as usize, start.get(1) as usize];
        let dest = [dest.get(0) as usize, dest.get(1) as usize];
        owner.clone().emit_signal(
            GodotString::from_str("move_is_legal"),
            &[Variant::from_bool(self.next_turn(owner, start, dest))]);
    }
    // write the contents of game_save field to a file
    #[export]
    fn save_game(&self, owner: Node, save_name: GodotString) {
        let save_name = save_name.to_string();
        let data_dir = if let Some(dir) = Self::get_data_dir() {
            dir
        } else {
            return;
        };
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir);
        }
        if let Ok(mut save_file) = File::create(data_dir.join(&save_name)) {
            save_file.write(self.game_save.as_bytes());
        }
    }
    // delete a saved game
    #[export]
    fn delete_save(&self, owner: Node, save_name: GodotString) {
        let save_name = save_name.to_string();
        let save_path = if let Some(dir) = Self::get_data_dir() {
            dir.join(save_name)
        } else {
            return;
        };
        fs::remove_file(save_path);
    }
    // check if a file exists so godot knows whether or not to
    // prompt the user to overwrite
    #[export]
    fn save_file_exists(&self, owner: Node, save_name: GodotString) -> bool {
        let save_name = save_name.to_string();
        let save_dir = if let Some(dir) = Self::get_data_dir() {
            dir
        } else {
            return false;
        };
        if save_dir.exists() && save_dir.join(save_name).exists() {
            return true;
        }
        false
    }
    // load the contents of a file into the game_save field
    #[export]
    unsafe fn load_game(&mut self, mut owner: Node, save_name: GodotString) {
        let save_name = save_name.to_string();
        let save_path = if let Some(dir) = Self::get_data_dir() {
            dir.join(save_name)
        } else {
            return;
        };
        let save_file = File::open(save_path);
        match save_file {
            Err(e) => {
                owner.emit_signal(
                    GodotString::from_str("error"),
                    &[Variant::from_str(&e.to_string())]);
            },
            Ok(mut file) => {
                let mut save_contents = String::new();
                file.read_to_string(&mut save_contents);
                self.game_save = save_contents;
            },
        }
    }
    // send godot the names of the avaiable saved games
    #[export]
    fn get_save_names(&self, owner: Node) -> StringArray {
        let mut names = StringArray::new();
        let save_dir = if let Some(dir) = Self::get_data_dir() {
            dir
        } else {
            return names;
        };
        if let Ok(entries) = save_dir.read_dir() {
            for entry in entries {
                let entry = entry.unwrap();
                if let Some(name) = entry.file_name().to_str() {
                    names.push(&GodotString::from_str(name));
                }
            }
        }
        names
    }
    // return how many turns the loaded game has
    #[export]
    fn get_num_turns(&self, owner: Node) -> usize {
        self.game_save.lines().collect::<Vec<&str>>().len()/17
    }
    // get the path to where saves are stored (platform dependant)
    fn get_data_dir() -> Option<PathBuf> {
        if let Some(dir) = dirs::data_dir() {
            Some(dir.join("dexter_chess_saves/"))
        } else {
            None
        }
    }
    // return the path to the data dir as a string so godot can display it
    #[export]
    fn data_dir_string(&self, owner: Node) -> GodotString {
        GodotString::from_str(Self::get_data_dir().unwrap().to_str().unwrap())
    }
    // load a specific turn from the game
    #[export]
    unsafe fn load_turn(&mut self, mut owner: Node, turn: usize) {
        // plus 1 since the current move will be AFTER the turn that
        // gets loaded
        self.turn_num =  turn+1;
        let mut piece_controller = owner
            .get_parent()
            .unwrap()
            .get_node(NodePath::from_str("Pieces"))
            .unwrap();
        // wrapper over godot method to make new pieces
        let mut godot_instance_piece = |is_white: bool, piece_type: &str, pos: [usize; 2]| {
            piece_controller.call(
                GodotString::from_str("instance_piece"),
                &[Variant::from_bool(is_white), 
                Variant::from_str(piece_type), 
                Variant::from_vector2(&Vector2::new(pos[0] as f32, pos[1] as f32))]);
        };
        let mut save_vec: Vec<&str> = self.game_save.lines().collect(); 
        let start_index = 17*turn;
        // send previous log entries
        for log_entry in 0..=turn {
            owner.emit_signal(
                GodotString::from_str("log_update"),
                &[Variant::from_str(&format!("{}\n", save_vec[log_entry*17+8]))]);
        }
        let this_turn: Vec<&str> = save_vec.drain(start_index..(start_index+17)).collect();
        // update the score
        // read into the rust code's fields
        self.score[0] = this_turn[0]
            .trim()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap();
        self.score[1] = this_turn[2]
            .trim()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap();
        self.white_captured = String::from(this_turn[1]);
        self.black_captured = String::from(this_turn[3]);
        // read into godot
        let score = format!("{}\n{}\n{}\n{}\n", 
                            this_turn[0], this_turn[1], this_turn[2], this_turn[3]);
        owner.emit_signal(
            GodotString::from_str("score_update"),
            &[Variant::from_str(&score)]);
        // set the player
        if turn%2 == 0 {
            self.player = false;
        } else {
            self.player = true;
        }
        // set white player's castle directions
        match this_turn[4] {
            "l" | "b" => self.white_can_castle_left = true,
            "r" | "b" => self.white_can_castle_right = true,
            _ => {},
        }
        // set black player's castle directions
        match this_turn[5] {
            "l" | "b" => self.black_can_castle_left = true,
            "r" | "b" => self.black_can_castle_right = true,
            _ => {},
        }
        // white en passant
        if let Ok(col) = this_turn[6].parse() {
            self.white_en_passant = Some([col, 2]);
        } else {
            self.white_en_passant = None;
        }
        // black en passant
        if let Ok(col) = this_turn[7].parse() {
            self.black_en_passant = Some([col, 5]);
        } else {
            self.black_en_passant = None;
        }
        // load the board
        for row in 0..8 {
            let chars = this_turn[9+row].chars();
            for (col, char) in chars.enumerate() {
                let piece: Option<Rc<dyn ChessPiece>> = match char {
                    '♙' => {
                        godot_instance_piece(true, "pawn", [col, row]);
                        Some(Rc::new(Pawn::new(true)))
                    },
                    '♘' => {
                        godot_instance_piece(true, "knight", [col, row]);
                        Some(Rc::new(Knight::new(true)))
                    },
                    '♗' => {
                        godot_instance_piece(true, "bishop", [col, row]);
                        Some(Rc::new(Bishop::new(true)))
                    },
                    '♖' => {
                        godot_instance_piece(true, "rook", [col, row]);
                        Some(Rc::new(Rook::new(true)))
                    },
                    '♕' => {
                        godot_instance_piece(true, "queen", [col, row]);
                        Some(Rc::new(Queen::new(true)))
                    },
                    '♔' => {
                        godot_instance_piece(true, "king", [col, row]);
                        Some(Rc::new(King::new(true)))
                    },
                    '♟' => {
                        godot_instance_piece(false, "pawn", [col, row]);
                        Some(Rc::new(Pawn::new(false)))
                    },
                    '♞' => {
                        godot_instance_piece(false, "knight", [col, row]);
                        Some(Rc::new(Knight::new(false)))
                    },
                    '♝' => {
                        godot_instance_piece(false, "bishop", [col, row]);
                        Some(Rc::new(Bishop::new(false)))
                    },
                    '♜' => {
                        godot_instance_piece(false, "rook", [col, row]);
                        Some(Rc::new(Rook::new(false)))
                    },
                    '♛' => {
                        godot_instance_piece(false, "queen", [col, row]);
                        Some(Rc::new(Queen::new(false)))
                    },
                    '♚' => {
                        godot_instance_piece(false, "king", [col, row]);
                        Some(Rc::new(King::new(false)))
                    },
                    _ => None,
                };
                self.board[col][row] = piece;
            }
        }
        let move_line = this_turn[8].trim();
        // make the game end popup show up in godot
        if move_line.contains("checkmate") {
            let winner = if self.player {
                -1
            } else {
                1
            };
            owner.emit_signal(
                GodotString::from_str("game_over"),
                &[Variant::from_i64(winner)]);
        } else if move_line.contains("stalemate") {
        } else if move_line.contains("draw") {
        }
    }
    unsafe fn _init(owner: Node) -> Self {
        Self::new()
    }
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
            game_save: String::new(),
            turn_num: 0,
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
    pub unsafe fn capture(&mut self, mut owner: Node, space: [usize; 2]) -> String {
        if let Some(piece) = &self.board[space[0]][space[1]] {
            if piece.is_white() {
                self.score[1] += piece.get_points();
                self.black_captured.push_str(piece.as_str());
            } else {
                self.score[0] += piece.get_points();
                self.white_captured.push_str(piece.as_str());
            }
            owner.emit_signal(
                GodotString::from_str("piece_captured"),
                &[Variant::from_vector2(&Vector2::new(space[0] as f32, space[1] as f32))]);
            let capture_string = format!(" captures {}", piece);
            owner.emit_signal(
                GodotString::from_str("log_update"),
                &[Variant::from_str(&capture_string)]);
            self.board[space[0]][space[1]] = None;
            owner.emit_signal(
                GodotString::from_str("score_update"),
                &[Variant::from_str(self.get_score())]);
            return capture_string;
        }
        String::new()
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
    pub unsafe fn upgrade_pawn_signal(&self, mut owner: Node, space: [usize; 2]) {
        owner.emit_signal(
            GodotString::from_str("pawn_promoted"),
            &[Variant::from_vector2(&Vector2::new(space[0] as f32, space[1] as f32))]);
    }
    // turn a pawn into a different piece
    #[export]
    pub unsafe fn upgrade_pawn(&mut self, mut owner: Node, dest: Vector2, piece_type: GodotString) {
        // do this since this method isn't called in sync with the turns
        // so basing the colour off the player turn could cause problems if players move
        // too fast.
        let dest = [dest.x as usize, dest.y as usize];
        let is_white = self.board[dest[0]][dest[1]].clone().unwrap().is_white();
        match piece_type.to_string().as_str() {
            "queen" => {
                self.board[dest[0]][dest[1]] =
                    Some(Rc::new(Queen::new(is_white)));
            },
            "rook" => {
                self.board[dest[0]][dest[1]] =
                    Some(Rc::new(Rook::new(is_white)));
            },
            "3" => {
                self.board[dest[0]][dest[1]] =
                    Some(Rc::new(Bishop::new(is_white)));
            },
            "4" => {
                self.board[dest[0]][dest[1]] =
                    Some(Rc::new(Knight::new(is_white)));
            },
            _ => {},
        }
        let promote_string = format!(" promoted to {}",
                                     self.board[dest[0]][dest[1]].as_ref().unwrap());
        owner.emit_signal(
            GodotString::from_str("log_update"),
            &[Variant::from_str(&promote_string)]);
        self.game_save.push_str(&promote_string);
        self.test_checkmate_stalemate(owner);
        owner.emit_signal(
            GodotString::from_str("log_update"),
            &[Variant::from_str("\n")]);
        self.game_save.push_str("\n");
        self.game_save.push_str(&format!("{}", self));
    }
    // read input not used anymore, but I don't want to go modify my game trait
    // so it's still here
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
    // make a new board. I could have repurposed my file loading method here, but I want
    // to have the default board hardcoded so that users can't delete the default board
    // save and break the game.
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
