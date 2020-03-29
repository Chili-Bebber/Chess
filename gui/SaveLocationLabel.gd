extends Label

func _ready():
    set_text(owner.get_node("ChessDirector").data_dir_string())
