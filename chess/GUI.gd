extends Node

var log_hidden = true
var options_hidden = true

onready var log_button = $LogPanel/MarginContainer/VSplitContainer/HSplitContainer/ShowLogButton
onready var log_animator = $LogPanelAnimator
onready var options_button = $OptionsContainer/CenterContainer/Button
onready var options_animator = $OptionsContainerAnimator
onready var log_label = $LogPanel/MarginContainer/VSplitContainer/Log
onready var move_entry = $ActionGrid/MoveEntry
onready var submit_button = $ActionGrid/SubmitButton
onready var resign_button = $ActionGrid/ResignButton
onready var draw_button = $ActionGrid/DrawButton

func _ready():
    get_parent().get_node("ChessDirector").connect("log_update", self, "_update_log")
    #get_parent().get_node("ChessDirector").connect("")
    log_animator.set_speed_scale(2.0)
    options_animator.set_speed_scale(2.0)
    log_button.connect("button_up", self, "_toggle_log")
    options_button.connect("button_up", self, "_toggle_options")

func _toggle_options():
    options_hidden = not options_hidden
    if options_hidden:
        options_animator.play_backwards("show")
    else:
        options_animator.play("show")
    

func _toggle_log():
    log_hidden = not log_hidden
    if log_hidden:
        log_animator.play_backwards("show")
        log_button.set_text("show")
    else:
        log_animator.play("show")
        log_button.set_text("hide")
        
func _update_log(entry: String):
    log_label.add_text(entry)
    
func set_actions_enabled(state: bool):
    move_entry.set_editable(state)
    submit_button.set_disabled(not state)
    resign_button.set_disabled(not state)
    draw_button.set_disabled(not state)

func clear_move_entry():
    move_entry.set_text("")
