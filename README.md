# gameloop
vds.io Labs Experiment

# goals
- Fixed game simulation speed.
- Variable target framerate.
- Deal reasonably well with cases where simulation or rendering is much faster/slower than the other.
- Be able to use a mechanism for yielding when there is no work to be done that is more resource friendly than spinning.

# resources
- https://gafferongames.com/post/fix_your_timestep/
