use arcade_jump_macros::solve_jump_parameter;

fn main() {
    let my_height = 20.0;
    let my_time = 10.0;

    solve_jump_parameter![
        my_height: Height, my_time: Time => my_impulse: Impulse, my_gravity: Gravity;
        10.0: Height, (my_impulse * 2.0): Impulse => my_lower_gravity: Gravity;
    ];
    assert_eq!(my_impulse, 4.0);
    assert_eq!(my_gravity, -0.4);
    assert_eq!(my_lower_gravity, -0.8);
}
