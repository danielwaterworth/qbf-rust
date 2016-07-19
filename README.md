This is an AIG-based QBF solver written in rust. The current strategy is to do
expansion up to some bound, then enumeration. It also does some basic
AIG-based optimizations.

As QBF solvers go, it's nothing special... yet.

The input format is non-CNF, but prenex-normal. Here's an example:

    forall a
    exists b
    forall c
    x = and(~a, b)
    y = or(x, c)
    z = not(x)
    w = or(z, y)
    w
