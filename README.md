This is a QBF solver written in rust. It was written primarily as a learning
exercise.

As things currently stand, it is completely naïve and does the exponential bad
thing without much in the way of optimizations or heuristics. Future plans
involve optimistically sending off subproblems to picosat and selective
expansion.

The input format is non-CNF, but prenex-normal. Here's an example:

    forall a
    exists b
    forall c
    x = and(~a, b)
    y = or(x, c)
    z = not(x)
    w = or(z, y)
    w
