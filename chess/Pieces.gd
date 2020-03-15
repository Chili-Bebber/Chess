extends Node

signal piece_moved
const PIECE = preload("res://chess/piece/ChessPiece.tscn")
onready var chess_game = get_parent()

func _piece_moved():
    emit_signal("piece_moved")
    
func new_game():
    #pawns
    for i in range(8):
        var white_pawn = PIECE.instance()
        white_pawn.set_fields(true, "pawn")
        white_pawn.set_translation(
            chess_game.get_tile(PoolIntArray([i, 1])).get_pos())
        self.add_child(white_pawn)
        var black_pawn = PIECE.instance()
        black_pawn.set_fields(false, "pawn")
        black_pawn.set_translation(
            chess_game.get_tile(PoolIntArray([i, 6])).get_pos())
        self.add_child(black_pawn)
    # rooks
    var white_rook_left = PIECE.instance()
    white_rook_left.set_fields(true, "rook")
    white_rook_left.set_translation(
        chess_game.get_tile(PoolIntArray([0,0])).get_pos())
    self.add_child(white_rook_left)
    var white_rook_right = PIECE.instance()
    white_rook_right.set_fields(true, "rook")
    white_rook_right.set_translation(
        chess_game.get_tile(PoolIntArray([7,0])).get_pos())
    self.add_child(white_rook_right)
    var black_rook_left = PIECE.instance()
    black_rook_left.set_fields(false, "rook")
    black_rook_left.set_translation(
        chess_game.get_tile(PoolIntArray([0,7])).get_pos())
    self.add_child(black_rook_left)
    var black_rook_right = PIECE.instance()
    black_rook_right.set_fields(false, "rook")
    black_rook_right.set_translation(
        chess_game.get_tile(PoolIntArray([7,7])).get_pos())
    self.add_child(black_rook_right)
    # bishops
    var white_bishop_left = PIECE.instance()
    white_bishop_left.set_fields(true, "bishop")
    white_bishop_left.set_translation(
        chess_game.get_tile(PoolIntArray([2,0])).get_pos())
    self.add_child(white_bishop_left)
    var white_bishop_right = PIECE.instance()
    white_bishop_right.set_fields(true, "bishop")
    white_bishop_right.set_translation(
        chess_game.get_tile(PoolIntArray([5,0])).get_pos())
    self.add_child(white_bishop_right)
    var black_bishop_left = PIECE.instance()
    black_bishop_left.set_fields(false, "bishop")
    black_bishop_left.set_translation(
        chess_game.get_tile(PoolIntArray([2,7])).get_pos())
    self.add_child(black_bishop_left)
    var black_bishop_right = PIECE.instance()
    black_bishop_right.set_fields(false, "bishop")
    black_bishop_right.set_translation(
        chess_game.get_tile(PoolIntArray([5,7])).get_pos())
    self.add_child(black_bishop_right)
    # knights
    var white_knight_left = PIECE.instance()
    white_knight_left.set_fields(true, "knight")
    white_knight_left.set_translation(
        chess_game.get_tile(PoolIntArray([1,0])).get_pos())
    self.add_child(white_knight_left)
    var white_knight_right = PIECE.instance()
    white_knight_right.set_fields(true, "knight")
    white_knight_right.set_translation(
        chess_game.get_tile(PoolIntArray([6,0])).get_pos())
    self.add_child(white_knight_right)
    var black_knight_left = PIECE.instance()
    black_knight_left.set_fields(false, "knight")
    black_knight_left.set_translation(
        chess_game.get_tile(PoolIntArray([1,7])).get_pos())
    self.add_child(black_knight_left)
    var black_knight_right = PIECE.instance()
    black_knight_right.set_fields(false, "knight")
    black_knight_right.set_translation(
        chess_game.get_tile(PoolIntArray([6,7])).get_pos())
    self.add_child(black_knight_right)
    # kings
    var white_king = PIECE.instance()
    white_king.set_fields(true, "king")
    white_king.set_translation(
        chess_game.get_tile(PoolIntArray([4,0])).get_pos())
    self.add_child(white_king)
    var black_king = PIECE.instance()
    black_king.set_fields(false, "king")
    black_king.set_translation(
        chess_game.get_tile(PoolIntArray([4,7])).get_pos())
    self.add_child(black_king)
    # queens
    var white_queen = PIECE.instance()
    white_queen.set_fields(true, "queen")
    white_queen.set_translation(
        chess_game.get_tile(PoolIntArray([3,0])).get_pos())
    self.add_child(white_queen)
    var black_queen = PIECE.instance()
    black_queen.set_fields(false, "queen")
    black_queen.set_translation(
        chess_game.get_tile(PoolIntArray([3,7])).get_pos())
    self.add_child(black_queen)
