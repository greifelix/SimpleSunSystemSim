# Simple Sun System Simulator

This is a hobby project to play around with bevy and learning a little bit more about the Solar system.


## Approach

- For the distances I use astronomical units
- For the planet sizes I use the earth as a reference size - then I use scaling functions where bigger planets are scaled down more than smaller planets
- The sun is scaled down even more

- I use elliptical orbits right now. Maybe I will try to use a bevy physics engine in the future and derive
the planet positions by the laws of physics.

$$
\begin{aligned}
x(\phi) = \frac{p}{1+\epsilon*\cos(\phi)}\cos(\phi)\\
y(\phi) = \frac{p}{1+\epsilon*\cos(\phi)}\sin(\phi)
\end{aligned}
$$

with $p = \frac{b^2}{a}$ and $\epsilon = \frac{e}{a}$.

The constant $a$ is the long half axis of the elipse and b the short half axis. $e$ is the distance of the center of the elipse to one focal point (i.e.  the sun). $\phi$ is the current angle, for $\phi=0: (x,y)=(a-e,0)$. Note that the coordinate is relative to the focal point.


## Disclaimer

- This does obviously not aim to be physically accurate
- I will need to update the "planet speeds" they are probably not yet correct
- I inserted the coordinates and starting positions with ai for quick prototyping.
They are probably NOT correct. I need to double check in the futures