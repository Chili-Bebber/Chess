extends HSplitContainer

onready var chess_director = get_parent().owner.get_node("ChessDirector")
onready var games = get_parent().owner.get_node("GUI/SaveLoadContainer/SaveLoadPanel/TabContainer/games")
onready var save = get_parent().owner.get_node("GUI/SaveLoadContainer/SaveLoadPanel/TabContainer/save")

onready var load_button = $HBoxContainer/LoadButton
onready var delete_button = $HBoxContainer/DeleteButton
onready var confirm_delete_button = $HBoxContainer/ConfirmDelete
onready var cancel_delete_button = $HBoxContainer/CancelDelete
onready var name_label = $GameName
onready var save_name = $GameName.get_text()

func _ready():
    load_button.connect("button_up", self, "_load_game")
    delete_button.connect("button_up", self, "_confirm_delete")
    confirm_delete_button.connect("button_up", self, "_delete")
    cancel_delete_button.connect("button_up", self, "_reset")

func _load_game():
    chess_director.load_game(save_name)
    games._new_game()
    save.name_edit.set_text(save_name)
    
func _confirm_delete():
    load_button.set_visible(false)
    delete_button.set_visible(false)
    confirm_delete_button.set_visible(true)
    cancel_delete_button.set_visible(true)
    name_label.set_text("delete?")

func _reset():
    load_button.set_visible(true)
    delete_button.set_visible(true)
    confirm_delete_button.set_visible(false)
    cancel_delete_button.set_visible(false)
    name_label.set_text(save_name)

func _delete():
    chess_director.delete_save(save_name)
    queue_free()
