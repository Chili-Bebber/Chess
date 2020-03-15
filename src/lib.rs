mod board;
mod chesspiece;
mod game;
#[macro_use]
extern crate gdnative as godot;
use board::ChessBoard;
fn init(handle: godot::init::InitHandle) {
    handle.add_class::<ChessBoard>();
}
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
