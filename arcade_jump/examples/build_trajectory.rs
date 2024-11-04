use arcade_jump_macros::solve_jump_parameter;

fn main() {
    let my_height = 20.0;
    let my_lower_height = 10.0;
    let my_time = 10.0;

    solve_jump_parameter![
        height: my_height, time: my_time => impulse: my_impulse, gravity: my_gravity;
        height: my_lower_height, impulse: my_impulse => gravity: my_lower_gravity;
    ];

    assert_eq!(my_impulse, 4.0);
}
