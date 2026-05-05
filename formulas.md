# Formulas

We can define variations of jump trajectory with four parameters
- $h$   the peak of the jump
- $t_h$ the time to reach the peak of the jump
- $v_0$ the initial vertical impulse
- $g$   the gravity force

Given two of the four parameters, 
we can compute the other two parameters 
by using the following formulas.


$$
\begin{align*}
h   &=  \frac{1}{2} v_0 t_h \\
    &= -\frac{1}{2} g t_h^2 \\
    &= -\frac{v_0^2}{2 g}   \\
\\
t_h &= \frac{2 h}{v_0}            \\
    &= \pm i \sqrt{\frac{2 h}{g}} \\
    &= -\frac{v_0}{g}             \\
\\
v_0 &= \frac{2 h}{t_h}    \\
    &= \pm i \sqrt{2 h g} \\
    &= -g t_h             \\
\\
g   &= -\frac{2 h}{t_h^2} \\
    &= -\frac{v_0^2}{2 h} \\
    &= -\frac{v_0}{t_h}
\end{align*}
$$

