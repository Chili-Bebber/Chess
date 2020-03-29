extends Node

var log_hidden = true
var options_hidden = true
var save_load_hidden = true
var timer_ran_out = false
var timer_enabled = false

onready var log_button = $LogPanel/MarginContainer/VSplitContainer/HSplitContainer/ShowLogButton
onready var log_animator = $LogPanelAnimator
onready var options_button = $OptionsContainer/CenterContainer/Button
onready var options_animator = $OptionsContainerAnimator
onready var log_label = $LogPanel/MarginContainer/VSplitContainer/Log
onready var score_label = $ScorePanel/MarginContainer/ScoreLabel
onready var move_entry = $ActionGrid/MoveEntry
onready var submit_button = $ActionGrid/SubmitButton
onready var resign_button = $ActionGrid/ResignButton
onready var draw_button = $ActionGrid/DrawButton
onready var save_load_button = $SaveLoadContainer/CenterContainer/Button
onready var save_load_animator = $SaveLoadAnimator
onready var minutes = $OptionsContainer/OptionsPanel/VBoxContainer/TimerContainer/Minutes
onready var seconds = $OptionsContainer/OptionsPanel/VBoxContainer/TimerContainer/Seconds
onready var turn_timer = owner.get_node("TurnTimer")
onready var timer_label = $InfoContainer/Timer
onready var timer_toggle = $OptionsContainer/OptionsPanel/VBoxContainer/TimerToggle
onready var indicator_toggle = $OptionsContainer/OptionsPanel/VBoxContainer/RankFileToggle

onready var chess_director = get_parent().get_node("ChessDirector")

func _ready():
    chess_director.connect("log_update", self, "_update_log")
    chess_director.connect("score_update", self, "_update_score")
    #get_parent().get_node("ChessDirector").connect("")
    log_animator.set_speed_scale(4.0)
    options_animator.set_speed_scale(4.0)
    log_button.connect("button_up", self, "_toggle_log")
    options_button.connect("button_up", self, "_toggle_options")
    save_load_button.connect("button_up", self, "_toggle_save_load")
    resign_button.connect("button_up", self, "_resign")
    minutes.connect("value_changed", self, "_minutes_changed")
    seconds.connect("value_changed", self, "_seconds_changed")
    timer_toggle.connect("toggled", self, "_timer_toggled")
    indicator_toggle.connect("toggled", self, "_indicators_toggled")
    
func _indicators_toggled(state: bool):
    owner.get_node("Indicators").set_visible(state)
    
func _timer_toggled(state: bool):
    timer_enabled = state
    if !timer_ran_out:
        if state:
            turn_timer.start()
        else:
            turn_timer.stop()
        timer_label._update_label()
        timer_label.set_visible(state)
    
func _minutes_changed(num_minutes: float):
    turn_timer.set_wait_time(num_minutes*60 + seconds.get_value())
    
func _seconds_changed(num_seconds: float):
    turn_timer.set_wait_time(minutes.get_value()*60 + num_seconds)

    
func hide_end_popup():
    $EndPopup.set_visible(false)
    
func _resign():
    if owner.is_white_turn:
        owner.end_game("White Resigns", true, true)
        _update_log("White Resigns\n")
    else:
        owner.end_game("Black Resigns", true, false)
        _update_log("Black Resigns\n")

func _toggle_save_load():
    save_load_hidden = not save_load_hidden
    if save_load_hidden:
        save_load_animator.play_backwards("show")
    else:
        save_load_animator.play("show")

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
    
func _update_score(score_string: String):
    score_label.set_text(score_string)
    
func set_actions_enabled(state: bool):
    move_entry.set_editable(state)
    submit_button.set_disabled(not state)
    resign_button.set_disabled(not state)
    draw_button.set_disabled(not state)

func clear_move_entry():
    move_entry.set_text("")
