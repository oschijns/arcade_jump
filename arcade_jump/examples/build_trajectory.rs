use arcade_jump::{Error, compute};

fn main() -> Result<(), Error> {
    runtime_evaluate()?;
    Ok(())
}

/// Evaluate values at runtime
fn runtime_evaluate() -> Result<(), Error> {
    let my_height: f32 = 20.0;
    let my_time: f32 = 10.0;

    let (my_impulse, my_gravity) =
        compute![ Height(my_height), Time(my_time) => Impulse, Gravity as f32 ]?;
    let my_lower_gravity = compute![ H(10.0), I(my_impulse) => G as f64 ]?;
    let higher_gravity = compute![ H(my_height), I(my_impulse * 2.0) => G ]?;

    assert_eq!(my_impulse, 4.0);
    assert_eq!(my_gravity, -0.4);
    assert_eq!(my_lower_gravity, -0.8);
    assert_eq!(higher_gravity, -1.6);

    Ok(())
}
