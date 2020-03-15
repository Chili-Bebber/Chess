extends Node

signal click
signal drag
signal drop

var input_enabled = true

const DRAG_TIME = 0.1

func _ready():
    $DragTimer.connect("timeout", self, "_drag_timer")

func _process(_delta):
    if input_enabled:
        if Input.is_action_just_pressed("click"):
            $DragTimer.start(DRAG_TIME)
        elif Input.is_action_just_released("click") and $DragTimer.is_stopped():
            emit_signal("drop")

# determine if the user clicked or started dragging.
# if the user is still holding the mouse after DRAG_TIME seconds
func _drag_timer():
    if Input.is_action_pressed("click"):
        emit_signal("drag")
    else:
        emit_signal("click")

func suspend_input():
    input_enabled = false
func resume_input():
    input_enabled = true
