use std::fmt;
use godot::Node;

pub trait Game : fmt::Display {
    // make a new game
    fn new_game(&mut self);
    // actual game
    /*fn game_loop(&mut self) {
        let winner = loop {
            self.print_game();
            self.next_turn();
            if let Some(winner) = self.get_winner() {
                break winner;
            }
        };
        self.end(winner);
    }*/
    // display the game visually
    fn print_game(&self);
    // swap whose turn it is
    fn switch_player(&mut self) {
        self.set_player(!self.get_player());
    }
    // take the next turn, call switch player if move was valid
    unsafe fn next_turn(&mut self, owner: Node, start: [usize; 2], dest: [usize; 2]) -> bool {
        if self.take_turn(owner, start, dest) {
            self.switch_player();
            return true;
        }
        false
    }
    // take a turn of the game
    unsafe fn take_turn(&mut self, owner: Node, start: [usize; 2], dest: [usize; 2]) -> bool;
    // getter for which player's turn it is
    fn get_player(&self) -> bool;
    // setter for which player's turn it is
    fn set_player(&mut self, player: bool);
    // return the player who won if there is one
    fn get_winner(&self) -> Option<i8>;
    // finish the game
    fn end(&self, winner: i8);
    // represent the game score with a string
    fn get_score(&self) -> String;
}
