extends Label

func _ready():
    set_player(true)

func set_player(is_white: bool):
    if is_white:
        set_text("White's Move")
    else:
        set_text("Black's Move")
