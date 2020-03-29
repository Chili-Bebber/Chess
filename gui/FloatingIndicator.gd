extends Spatial

export var label: String

func _ready():
    $Viewport/Label.set_text(label)
