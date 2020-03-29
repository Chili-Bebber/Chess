extends Label

onready var turn_timer = owner.get_node("TurnTimer")

func _ready():
    $SecondTimer.connect("timeout", self, "_update_label")
    _update_label()
    
func _update_label():
    if not turn_timer.is_stopped():
        var total_seconds = turn_timer.get_time_left()
        var minutes = int(total_seconds/60)
        var seconds = int(total_seconds - minutes*60)
        set_text("%s:%s" % [minutes, seconds])
