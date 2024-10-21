use arcade_jump::*;

fn main() {
    // primary high jump
    let (impulse, gravity) = solve![{20.0, 10.0, ?, ?} as f32];

    // secondary low jump, keep the same initial impulse but change the height reached
    let (time2, gravity2) = solve![{10.0, ?, impulse, ?} as f32];

    // third wall jump, keep the same gravity but vary the impulse
    let (height3, impulse3) = solve![{?, 5.0, ?, gravity} as f32];

    println!["Primary: (impulse: {}, gravity: {})", impulse, gravity];
    println!["Secondary: (time: {}, gravity: {})", time2, gravity2];
    println!["Third: (height: {}, impulse: {})", height3, impulse3];
}
