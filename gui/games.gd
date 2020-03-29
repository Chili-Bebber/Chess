extends Tabs

const GAME_ENTRY = preload("res://gui/GameEntry.tscn")

onready var moves = owner.get_node("GUI/SaveLoadContainer/SaveLoadPanel/TabContainer/moves")

onready var new_game_button = $VSplitContainer/CenterContainer/Button
onready var piece_controller = owner.get_node("Pieces")
onready var chess_director = owner.get_node("ChessDirector")
onready var list_container = $VSplitContainer/MarginContainer/ScrollContainer/VBoxContainer
onready var gui = owner.get_node("GUI")
onready var turn_timer = owner.get_node("TurnTimer")
onready var name_entry = gui.get_node("SaveLoadContainer/SaveLoadPanel/TabContainer/save/VBoxContainer/MarginContainer/NameEdit")

func _ready():
    new_game_button.connect("button_up", self, "_new_game")
    owner.get_node("GUI/EndPopup/VSplitContainer/NewGameContainer/NewGame").connect("button_up", self, "_new_game")
    populate_save_list()
    
func _new_game():
    gui.hide_end_popup()
    moves.update_moves()
    piece_controller.new_game()
    chess_director.reset_game()
    gui.log_label.set_text("")
    gui.score_label.set_text("White Score: 0\n\nBlack Score: 0\n\n")
    name_entry.set_text("")
    reset_misc()

func get_save_names() -> PoolStringArray:
    return chess_director.get_save_names()
    
func populate_save_list():
    for child in list_container.get_children():
        if not child.is_in_group("flag_placeholder"):
            child.queue_free()
    for entry_name in get_save_names():
        add_entry(entry_name)
        
func add_entry(entry_name: String):
    var entry = GAME_ENTRY.instance()
    entry.get_node("GameName").set_text(entry_name)
    list_container.add_child(entry)

func reset_misc():
    owner.reset_draw_mode()
    get_node("/root/MouseInput").resume_input()
    if (not turn_timer.is_stopped() or gui.timer_ran_out) and gui.timer_enabled:
        turn_timer.stop()
        turn_timer.start()
    gui.timer_ran_out = false
    gui.timer_label._update_label()
    gui.set_actions_enabled(true)
    owner.held_piece = null
    owner.is_white_turn = true
    owner.winner = -2
