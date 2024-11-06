use arcade_jump_macros::solve_jump_parameter;

fn main() {
    runtime_evaluate();
    compile_time_evaluate();
}

/// Evaluate values at runtime
fn runtime_evaluate() {
    let my_height: f32 = 20.0;
    let my_time: f32 = 10.0;

    solve_jump_parameter![
        use f32;
        my_height: Height, my_time: Time => my_impulse: Impulse, my_gravity: Gravity;
        10.0: Height, my_impulse: Impulse => my_lower_gravity: Gravity;
        my_height: H, (my_impulse * 2.0): I => higher_grav: G;
    ];
    assert_eq!(my_impulse, 4.0);
    assert_eq!(my_gravity, -0.4);
    assert_eq!(my_lower_gravity, -0.8);
    assert_eq!(higher_grav, -1.6);
}

/// Evaluate values at compile-time
fn compile_time_evaluate() {
    const HEIGHT: f64 = 20.0;
    const TIME: f64 = 10.0;

    solve_jump_parameter![
        use const f64;
        HEIGHT: Height, TIME: Time => IMPULSE: Impulse, GRAVITY: Gravity;
        10.0: Height, IMPULSE: Impulse => LOWER_GRAVITY: Gravity;
        HEIGHT: H, (IMPULSE * 2.0): I => HIGHER_GRAVITY: G;
    ];
    assert_eq!(IMPULSE, 4.0);
    assert_eq!(GRAVITY, -0.4);
    assert_eq!(LOWER_GRAVITY, -0.8);
    assert_eq!(HIGHER_GRAVITY, -1.6);
}
