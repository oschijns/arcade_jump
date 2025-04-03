use arcade_jump::resolver::ResolveError;
use arcade_jump_macros::jump_parameters;

fn main() -> Result<(), ResolveError> {
    runtime_evaluate()?;
    compile_time_evaluate()?;
    Ok(())
}

/// Evaluate values at runtime
fn runtime_evaluate() -> Result<(), ResolveError> {
    let my_height: f32 = 20.0;
    let my_time: f32 = 10.0;

    jump_parameters![
        use f32;
        my_height: Height, my_time: Time => my_impulse: Impulse, my_gravity: Gravity;
        10.0: Height, my_impulse: Impulse => my_lower_gravity: Gravity;
        my_height: H, (my_impulse * 2.0): I => higher_grav: G;
    ];
    assert_eq!(my_impulse, 4.0);
    assert_eq!(my_gravity, -0.4);
    assert_eq!(my_lower_gravity, -0.8);
    assert_eq!(higher_grav, -1.6);

    Ok(())
}

/// Evaluate values at compile-time
fn compile_time_evaluate() -> Result<(), ResolveError> {
    const HEIGHT: f64 = 20.0;
    const TIME: f64 = 10.0;

    jump_parameters![
        use f64;
        HEIGHT: Height, TIME: Time => impulse: Impulse, gravity: Gravity;
        10.0: Height, impulse: Impulse => lower_gravity: Gravity;
        HEIGHT: H, (impulse * 2.0): I => higher_gravity: G;
    ];
    assert_eq!(impulse, 4.0);
    assert_eq!(gravity, -0.4);
    assert_eq!(lower_gravity, -0.8);
    assert_eq!(higher_gravity, -1.6);

    Ok(())
}
