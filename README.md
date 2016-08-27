This is an AIG-based QBF solver written in rust. It works by alternating
between expansion and simplification. This works well in practice, it is
able to verify the correctness of a sorting network that sorts 1024
elements.

The input format is non-CNF, but prenex-normal. Here's an example:

    forall a
    exists b
    forall c
    x = and(~a, b)
    y = or(x, c)
    z = not(x)
    w = or(z, y)
    w
