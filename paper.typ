/* METADATA */
#let title = [Arcade jump trajectory]
#let author = [
    #link("https://github.com/oschijns")[Olivier Schyns]
]

/* MODULES */
#import "@preview/cetz:0.1.2": canvas, plot
//#import "@preview/bob-draw:0.1.0": *

/* LAYOUT */
#set page(
  paper: "a4",
  header: align(
    right + horizon,
    title
  ),
  numbering: "1",
  columns: 2,
)
#place(
  top + center,
  float: true,
  scope: "parent",
  clearance: 2em,
)[
  #align(center, text(17pt)[*#title*])
  #align(center, [#author])
]
#show heading.where(
  level: 1
): it => block(width: 100%)[
  #set align(center)
  #set text(15pt, weight: "bold")
  #pad(top: 20pt, smallcaps(it.body))
]
#set par(justify: true)


/* FUNCTIONS */

/// Describe a ballistic trajectory in term of
/// p The initial position
/// v The initial vertical velocity
/// g The gravitational force
/// t The time
#let ballistic(p, v, g, t) = {
  g / 2.0 * (t * t) + v * t + p
}

/// Power of 2
#let pow2(x) = {x * x}

// Variations
#let h_from_t_v(t, v) = { 0.5 * v * t}
#let h_from_t_g(t, g) = {-0.5 * g * pow2(t)}
#let h_from_v_g(v, g) = {-0.5 * pow2(v) / g}
#let t_from_h_v(h, v) = {0.5 * h / v}
#let t_from_h_g(h, g) = {calc.sqrt(calc.abs(2.0 * h / g))}
#let t_from_v_g(v, g) = {-v / g}
#let v_from_h_t(h, t) = {2.0 * h / t}
#let v_from_h_g(h, g) = {calc.sqrt(calc.abs(2.0 * h * g))}
#let v_from_t_g(t, g) = {-g * t}
#let g_from_h_t(h, t) = {-2.0 * h / pow2(t)}
#let g_from_h_v(h, v) = {-0.5 * pow2(v) / h}
#let g_from_t_v(t, v) = {-v / t}

/// Configure ballistic trajectory from height of peak and time to peak
#let param_ballistic(h, th) = {
  (v_from_h_t(h, th), g_from_h_t(h, th))
}

/// Create a figure to draw graphs
#let draw_graph(caption, function, x_size: 25, y_size: 25, x_label: "time") = {
  figure(
    canvas(length: 1cm, {
      plot.plot(size: (5, 5),
      x-min: 0,
      x-max: x_size,
      y-min: 0,
      y-max: y_size,
      x-tick-step: 5,
      y-tick-step: 5,
      x-label: x_label,
      y-label: "height",
      y-grid: true, function)
    }), caption: caption)
}

/* DOCUMENT */
= Introduction
Without air resistance in an Euclidean referential,
a projectile follows a parabolic trajectory which we express as
the function $f$ which describe the altitude of the projectile through time.

#{
  // config
  let (v, g) = param_ballistic(20, 10);

  // draw
  draw_graph("Ballistic trajectory", {
    plot.add(domain: (0, 25), t => ballistic(0, v, g, t))
  })
}

We define
- $g$ the gravitational force
- $v_0$ the initial vertical velocity
- $p_0$ the initial altitude

$
f  (t) &= 1/2 g t^2 + v_0 t + p_0 && "    Displacement" \
f' (t) &=     g t   + v_0         && "    Velocity"     \
f''(t) &=     g                   && "    Acceleration"
$

In the following expressions,
we consider that we always start at the ground level such as $p_0 = 0$.

= Parametrization

Rather than approximately fine tuning $g$ and $v_0$ to generate a trajectory,
we want to be able to describe it as a function of height of the peak and time to reach the peak.
This will allow us to have a precise control over the trajectory of our projectile.

Since a ballistic trajectory is parabolic, when the projectile reaches its peak, its velocity is null.
We have the following system where $h$ and $t_h$ are known.

$
f (t_h) &= 1/2 g t_h^2 + v_0 t_h &= h \
f'(t_h) &=     g t_h   + v_0     &= 0
$

with
- $h$ the height of the peak
- $t_h$ the time it takes to reach that peak

We can find the value of $v_0$ by substitution in $f'(t_h)$.

$
f'(t_h) &= 0           \
f'(t_h) &= g t_h + v_0 \
-g t_h  &= v_0
$

We can find an expression of $g$ in term of $h$ and $t_h$ by substitution in $f(t_h)$.

$
h &=  1/2 g t_h^2 + v_0 t_h      \
  &=  1/2 g t_h^2 + (-g t_h) t_h \
  &=  1/2 g t_h^2 - g t_h^2      \
  &= -1/2 g t_h^2                \
-(2 h)/t_h^2 &= g
$

We can reinject the expression found for $g$ in the equation defining $v_0$.

$
v_0 &= -g t_h              \
    &= -(-(2 h)/t_h^2) t_h \
    &= (2 h)/t_h
$

We can express the initial vertical velocity $v_0$ and the gravity $g$
of our system as expressions of the parameters $h$ the altitude of the peak
and $t_h$ the time to reach that peak.

$
v_0 &=  (2 h)/t_h \
g   &= -(2 h)/t_h^2
$

#{
  // config
  let (v1, g1) = param_ballistic(20, 10);
  let (v2, g2) = param_ballistic(10, 15);
  let (v3, g3) = param_ballistic(25, 5);

  // draw
  draw_graph("Parametrization of trajectories based on height and duration", {
    plot.add(domain: (0, 25), t => ballistic(0, v1, g1, t))
    plot.add(domain: (0, 25), t => ballistic(0, v2, g2, t))
    plot.add(domain: (0, 25), t => ballistic(0, v3, g3, t))
  })
}

= Horizontal motion

Using $t_h$ may not be convenient to describe a trajectory. If the projectile is also moving horizontally, we can introduce additional parameters.

We introduce
- $v_x$ as the horizontal velocity
- $d$ as the range of the jump
- $r in lr([0, 1])$ as the ratio between the ascending and descending phases of the jump

The values of $v_0$ and $g$ can be reexpressed trivially as expressions of $h$, $v_x$, $d$ and $r$.

$
t_h &=  (d r)/v_x             \
v_0 &=  (2 h v_x  )/(d r)     \
g   &= -(2 h v_x^2)/(d^2 r^2)
$

#{
  // config
  let h = 20;
  let d = 25;
  let r = 0.6;

  // draw
  let th = d * r;
  let (v1, g1) = param_ballistic(h, th);
  let (v2, g2) = param_ballistic(h, d * (1.0 - r));
  draw_graph("Different ascending and descending phases", {
    plot.add(domain: ( 0, th), t => ballistic(0, v1, g1, t))
    plot.add(domain: (th, 25), t => ballistic(h,  0, g2, t - th))
  })
}

= Variation

In video games, common tropes are being able to vary the height of a jump after initiating it.
Those behavior don't occur in the real world but provide better controls.
Since we have four parameters, constraining two of them gives us the result for the other two.

We start by expressing $g$ in terms of $v_0$ and $t_h$.

$
g &= -(2 h)/t_h^2           \
  &= -(2 h)/t_h times 1/t_h \
  &= -  v_0/t_h
$

We can trivially deduce $v_0 = -g t_h$.

Which gives us an expression of $h$ in terms of $g$ and $t_h$.
$
(2 h)/t_h &= -g t_h       \
h         &= -1/2 g t_h^2 \
$

Then we can find an expression of $h$ in terms of $v_0$ and $t_h$.
$
h &= -1/2 (-v_0/t_h) t_h^2 \
  &=  1/2   v_0 t_h
$

From $v_0 = -g t_h$, we have $t_h = -v_0/g$ which we use to find an expression of $g$ in terms of $h$ and $v_0$.

$
g &= -(2 h    )/t_h^2         \
  &= -(2 h    )/(-v_0 /g  )^2 \
  &= -(2 h    )/(v_0^2/g^2)   \
  &= -(2 h g^2)/v_0^2         \
1 &= -(2 h g  )/v_0^2         \
-v_0^2/(2 h) &= g
$

We can deduce an expression of $v_0$ in terms of $g$ and $h$. But in that case we have two complex numbers as solutions.

$
-v_0^2/(2 h) &= g      \
v_0^2        &= -2 h g \
v_0          &= plus.minus i sqrt(2 h g)
$

We also have $h$ in terms of $v_0$ and $g$.

$
g &= -v_0^2/(2 h) \
h &= -v_0^2/(2 g)
$

Finally, we can find expressions of $t_h$ in terms of $v_0$ and $g$.

$
-g t_h &= v_0    \
t_h    &= -v_0/g
$

Then in terms of $h$ and $v_0$.

$
v_0 &= (2 h)/t_h \
t_h &= (2 h)/v_0
$

And in terms of $h$ and $g$. Again we get complex numbers as solutions.

$
-1/2 g t_h^2 &= h        \
t_h^2        &= -(2 h)/g \
t_h          &= plus.minus i sqrt((2 h)/g)
$

= Conclusion
To summarize, we get the following expressions for $h$, $t_h$, $v_0$ and $g$.

$
h   &=  1/2 v_0 t_h \
    &= -1/2 g t_h^2 \
    &= -v_0^2/(2 g) \
\
\
t_h &= (2 h)/v_0                  \
    &= plus.minus i sqrt((2 h)/g) \
    &= -v_0/g                     \
\
\
v_0 &= (2 h)/t_h                \
    &= plus.minus i sqrt(2 h g) \
    &= -g t_h                   \
\
\
g   &= -(2 h)/t_h^2 \
    &= -v_0^2/(2 h) \
    &= -v_0  /t_h
$

From those formulas, we can implement a variable height jump based on how long we press on the "jump" button. In that case, the initial vertical impulse is the same but the gravity will change when the player release the button.

#{
  // config
  let (v1, g1) = param_ballistic(20, 10);
  let g2 = g_from_h_v(10, v1);

  // draw
  draw_graph("Different heights with a vertical velocity constraint", {
    plot.add(domain: (0, 25), t => ballistic(0, v1, g1, t))
    plot.add(domain: (0, 25), t => ballistic(0, v1, g2, t))
  })
}

We can also implement a double-jump which use the same gravity as the main jump but should be able to reach a smaller height from the point where e started the double-jump.

#{
  // config
  let (v1, g1) = param_ballistic(20, 10);
  let v2 = v_from_h_g(5, g1);
  let t_btn = 12;
  let p2 = ballistic(0, v1, g1, t_btn);

  // draw
  draw_graph("Double-jump with a smaller second jump", {
    plot.add(domain: (0, t_btn), t => ballistic(0, v1, g1, t))
    plot.add(domain: (t_btn, 25), t => ballistic(p2, v2, g1, t - t_btn))
  })
}

And we can implement a wall-jump but instead of picking a height, we want to select the distance we can reach.

#{
  // config
  let (v1, g1) = param_ballistic(15, 15);
  let v2 = v_from_t_g(10, g1);
  let wall = 20;
  let p2 = ballistic(0, v1, g1, wall);

  // draw
  draw_graph("Wall-jump with a predefined reach", {
    plot.add(domain: (0, wall), t => ballistic(0, v1, g1, t))
    plot.add(domain: (0, wall), t => ballistic(p2, v2, g1, wall - t))
    plot.add(((wall, 0), (wall, 25))) // the wall
  }, x_label: "distance")
}

= References
- #link("https://youtu.be/hG9SzQxaCm8")[GDC Building a Better Jump] by Kyle Pittman
