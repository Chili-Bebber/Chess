extends Button

onready var chess_game = get_parent().owner

onready var chess_director = chess_game.get_node("ChessDirector")
onready var piece_controller = chess_game.get_node("Pieces")
onready var log_label = chess_game.get_node("GUI").log_label
onready var turn_label = chess_game.turn_label
onready var gui = chess_game.get_node("GUI")
onready var turn_timer = chess_game.get_node("TurnTimer")
var turn_num = 0

func _ready():
    connect("button_up", self, "_button_up")
    set_text("move %s" % (turn_num+1))
    
func _button_up():
    # reset the draw button
    chess_game.reset_draw_mode()
    # set the timer to not be depleted
    gui.timer_ran_out = false
    if (not turn_timer.is_stopped() or gui.timer_ran_out) and gui.timer_enabled:
        turn_timer.stop()
        turn_timer.start()
    gui.timer_label._update_label()
    # reset the winner storage variable
    get_parent().owner.winner = -2
    # clear the log
    log_label.set_text("")
    # place new pieces
    gui.set_actions_enabled(true)
    get_node("/root/MouseInput").resume_input()
    gui.hide_end_popup()
    piece_controller.clear_board()
    chess_director.load_turn(turn_num)
    if turn_num % 2 == 0:
        get_parent().owner.is_white_turn = false
        turn_label.set_player(false)
    else:
        get_parent().owner.is_white_turn = true
        turn_label.set_player(true)
