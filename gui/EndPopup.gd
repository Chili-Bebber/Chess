extends PopupPanel

func _ready():
    $VSplitContainer/NewGameContainer/NewGame.connect("button_up", self, "_hide")
    
func _hide():
    set_visible(false)

func set_text(text: String):
    $VSplitContainer/EndMessageContainer/EndMessage.set_text(text)
