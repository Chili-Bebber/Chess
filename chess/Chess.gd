extends Spatial

onready var camera = $CameraArm/Camera
onready var camera_ray = $CameraRay
onready var move_entry = $GUI/ActionGrid/MoveEntry
onready var submit_button = $GUI/ActionGrid/SubmitButton
onready var pawn_promotion_popup = $GUI/PawnPromotionPopup
onready var mouse_input = get_node("/root/MouseInput")

var popup_open = false
var is_white_turn = true
var held_piece
var prev_position
var request_process = false
var signal_queue = [Array(), Array()]
var drag_point = Vector3()
var animation_enabled = true
var move_start = Vector3()
var move_dest = Vector3()

const BOARD_WIDTH = 4.2

signal queue_resolved

func _ready():
    $ChessDirector.connect("game_over", self, "_on_game_over")
    $ChessDirector.connect("move_is_legal", self, "_move_tried")
    $ChessDirector.connect("castle", self, "_castle")
    $ChessDirector.connect("piece_captured", self, "_piece_captured")
    $ChessDirector.connect("pawn_promoted", self, "_pawn_promoted")
    $Pieces.connect("piece_moved", self, "_piece_moved")
    $GUI/OptionsContainer/OptionsPanel/VBoxContainer/AnimationToggle.connect(
        "toggled", self, "_animation_toggled")
    
    move_entry.connect("text_entered", self, "_move_entered")
    submit_button.connect("pressed", self, "_move_entered")
    
    $Pieces.new_game()
    
func _move_entered(input = ""):
    if input == "":
        input = move_entry.get_text()
    move_entry.set_text("")
    if input.length() == 5:
        var input_bytes = input.to_ascii()
        var start = [input_bytes[0]-97, input_bytes[1]-49]
        var dest = [input_bytes[3]-97, input_bytes[4]-49]
        if start[0] >= 0 and start[0] < 8 and start[1] >= 0 and start[1] < 8 \
        and dest[0] >= 0 and dest[0] < 8 and dest[1] >= 0 and dest[1] < 8:
            try_move_text(start, dest)
            return
    $GUI/MoveEntryAnimator.play("move_entry_shake")
    
func _move_tried(move_was_legal: bool):
    $GUI.set_actions_enabled(false)
    mouse_input.suspend_input()
    if move_was_legal:
        is_white_turn = not is_white_turn
        signal_queue[0].append("move")
        signal_queue[1].append(Vector2())
    elif held_piece and held_piece.is_moving:
        signal_queue[0].append("move")
        signal_queue[1].append(Vector2(-1, -1))    
    resolve_queue()
    if held_piece and held_piece.is_moving:
        yield(self, "queue_resolved")
    if not popup_open:
        $GUI.set_actions_enabled(true)
        mouse_input.resume_input()
    $GUI.clear_move_entry()
    
func _piece_moved():
    resolve_queue()
    
func resolve_queue():
    var queue_size = signal_queue[0].size()
    if queue_size > 0:
        var signal_type = signal_queue[0][0]
        var signal_data = signal_queue[1][0]
        signal_queue[0].remove(0)
        signal_queue[1].remove(0)
        if signal_type == "castle":
            castle(signal_data)
        elif signal_type == "pawn_promotion":
            promote_pawn(signal_data)
        elif signal_type == "capture":
            capture_piece(signal_data)
        elif signal_type == "move":
            if signal_data == Vector2():
                held_piece.move(move_dest, 1.5)
            else:
                held_piece.move(get_tile(held_piece.start).get_pos())
    else:
        held_piece = null
        move_start = null
        move_dest = null
        emit_signal("queue_resolved")
      
func capture_piece(position: Vector2):
    var captured_piece = \
        get_tile(PoolIntArray([position.x, position.y])) \
        .get_piece()
    if captured_piece:
        if held_piece.is_moving and captured_piece.get_tile().get_pos() == move_dest:
            held_piece.move(move_dest+Vector3(1.8, 3, 0), 0.6, 2.0, false)
            var capture_timer = Timer.new()
            capture_timer.set_wait_time(0.5)
            capture_timer.set_one_shot(true)
            self.add_child(capture_timer)
            capture_timer.start()
            yield(capture_timer, "timeout")
            capture_timer.queue_free()
        else:
            resolve_queue()
        captured_piece.capture()
    
func castle(position: Vector2):
    var castle = get_tile([position.x, position.y]).get_piece()
    var castle_dest = Vector2()
    if position == Vector2(0,0):
        castle_dest = get_tile([3,0]).get_pos()
    elif position == Vector2(7,0):
        castle_dest = get_tile([5,0]).get_pos()
    elif position == Vector2(0,7):
        castle_dest = get_tile([3,7]).get_pos()
    elif position == Vector2(7,7):
        castle_dest = get_tile([5,7]).get_pos()
    if held_piece:
        held_piece.move(move_dest+Vector3(0,3,0), 1, 2, false)
    castle.move(castle_dest, 1.0, 0.8)
    
func promote_pawn(position: Vector2):
    popup_open = true
    held_piece.move(move_dest, 1, 2, false)
    var mouse_input = get_node("/root/MouseInput")
    mouse_input.suspend_input()
    $GUI.set_actions_enabled(false)
    
    pawn_promotion_popup.popup()
    # wait for signal from pawn promotion popup
    var new_piece_type = yield(pawn_promotion_popup, "piece_type_selected")
    pawn_promotion_popup.hide()
    
    held_piece.promote(new_piece_type)
    
    mouse_input.resume_input()
    $GUI.set_actions_enabled(true)
    popup_open = false
    
    
func _castle(castle_position: Vector2):
    signal_queue[0].append("castle")
    signal_queue[1].append(castle_position)
    
func _piece_captured(capture_position: Vector2):
    signal_queue[0].append("capture")
    signal_queue[1].append(capture_position)

func _pawn_promoted(pawn_position: Vector2):
    signal_queue[0].append("pawn_promotion")
    signal_queue[1].append(pawn_position)

func _on_game_over(winner: int):
    # TODO make a big showoffy ending
    pass
    
func try_move_drag(piece: RigidBody, start: PoolIntArray, dest: PoolIntArray):
    held_piece = piece
    move_start = get_tile(start).get_pos()
    move_dest = get_tile(dest).get_pos()
    $ChessDirector.try_move(start, dest)
    
func try_move_text(start: PoolIntArray, dest: PoolIntArray):
    held_piece = get_tile(start).get_piece()
    move_start = get_tile(start).get_pos()
    move_dest = get_tile(dest).get_pos()
    $ChessDirector.try_move(start, dest)
    var move_was_legal = yield($ChessDirector, "move_is_legal")
    if not move_was_legal:
        $GUI/MoveEntryAnimator.play("move_entry_shake")
    
func _process(delta):
    if not request_process:
        set_process(false)
        
func _input(event):
    if event is InputEventMouseMotion:
        if Input.is_action_pressed("right_click") and prev_position:
            var mouse_displacement = event.position - prev_position
            $CameraArm.rotate_y(-mouse_displacement.x/100.0)
            var camera_x = $CameraArm.get_rotation().x
            var x_rotation = -mouse_displacement.y/100.0 + camera_x
            if x_rotation >= -PI/2.0 and x_rotation <= -PI/8.0:
                $CameraArm.rotate_object_local(Vector3(1, 0, 0), -mouse_displacement.y/100.0)
        prev_position = event.position
        var ray_origin = camera.project_ray_origin(event.position)
        var ray_vector = camera.project_ray_normal(event.position)*100
        camera_ray.set_translation(ray_origin)
        camera_ray.set_cast_to(ray_vector)
        camera_ray.force_raycast_update()
        var point = camera_ray.get_collision_point()
        if point:
            # make sure the player can't move pieces off the board
            if abs(point.x) > BOARD_WIDTH:
                if point.x > 0:
                    point.x = BOARD_WIDTH
                else:
                    point.x = -BOARD_WIDTH
            if abs(point.z) > BOARD_WIDTH:
                if point.z > 0:
                    point.z = BOARD_WIDTH
                else:
                    point.z = -BOARD_WIDTH
            var drag_position = point + Vector3(0, 2, 0)
            drag_point = drag_position
    elif event is InputEventMouseButton:
        var arm_length = $CameraArm.get_length()
        if Input.is_action_pressed("scroll_up") and arm_length > 5.0:
            $CameraArm.set_length(arm_length - 0.5)
        elif Input.is_action_pressed("scroll_down") and arm_length < 20.0:
            $CameraArm.set_length(arm_length + 0.5)
    
func get_tile(pos: PoolIntArray) -> Node:
    var node_path = "Tiles/%s,%s"
    node_path = node_path % [pos[0], pos[1]]
    return get_node(node_path)
    
func _animation_toggled(state: bool):
    animation_enabled = state
