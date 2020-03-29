extends Tabs

onready var chess_director = owner.get_node("ChessDirector")
onready var games = get_parent().get_node("games")

onready var name_edit = $VBoxContainer/MarginContainer/NameEdit
onready var save_button = $VBoxContainer/HSplitContainer/SaveButton
onready var overwrite_container = $VBoxContainer/HSplitContainer/HBoxContainer
onready var confirm_button = $VBoxContainer/HSplitContainer/HBoxContainer/ConfirmButton
onready var cancel_button = $VBoxContainer/HSplitContainer/HBoxContainer/CancelButton

func _ready():
    name_edit.connect("text_entered", self, "_save_button_clicked")
    save_button.connect("button_up", self, "_save_button_clicked")
    confirm_button.connect("button_up", self, "_save_game")
    confirm_button.connect("button_up", self, "_reset_buttons")
    cancel_button.connect("button_up", self, "_reset_buttons")

func _save_button_clicked(text = ""):
    if name_edit.get_text().length() != 0:
        if chess_director.save_file_exists(name_edit.get_text()):
            name_edit.set_editable(false)
            save_button.set_disabled(true)
            overwrite_container.set_visible(true)
        else:
            _save_game()

func _save_game():
    var save_name = name_edit.get_text()
    chess_director.save_game(save_name)
    games.add_entry(save_name)

func _reset_buttons():
    overwrite_container.set_visible(false)
    save_button.set_disabled(false)
    name_edit.set_editable(true)
