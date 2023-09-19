# R1csViewer

A simple viewer for binary `r1cs` files formatted according to the
[R1CS binary
format](https://github.com/iden3/r1csfile/blob/master/doc/r1cs_bin_format.md).
See [here](https://tlu.tarilabs.com/cryptography/rank-1),
[here](https://coders-errand.com/how-to-build-a-quadratic-arithmetic-program/)
and
[here](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649)
for more on Rank 1 Constraint Systems.

# Installation

In order to use this library, you'll need to have [Rust
installed](https://www.rust-lang.org/tools/install).  

Having installed Rust, next clone this repository.  Then, you can view
a `r1cs` file using the following command:

```
> cargo run file.r1cs
```

# Example Usage

The following illustrates a simple Circom file:

```circom
pragma circom 2.0.0;

template logicalAnd () {

   // Declaration of signals.

   signal input a;
   signal input b;   
   signal output out;

   a * (a-1) === 0;
   b * (b-1) === 0;
   //
   out <== a * b;
}

component main = logicalAnd();
```

This takes two inputs which must be booleans (i.e. either `0` or `1`)
and outputs their logical AND (i.e. `a && b`).  Observe that
multiplication, in this context, is equivalent to logical AND.

We can use the circom compiler as follows to generate the R1CS file
for this program:

```
circom --r1cs logical_and.circom
```

This outputs a file called `logical_and.r1cs` which is a binary file
formatted according to the [R1CS binary
standard](https://github.com/iden3/r1csfile/blob/master/doc/r1cs_bin_format.md).
At this point, we can view the file by running the viewer like so:

```
cargo run logical_and.r1cs
```

And we should see some output like this:

```
Reading file logical_and.r1cs.
Prime: 21888242871839275222246405745257275088548364400416034343698204186575808495617
#Constraints: 3
(-1 + 1.w2) * (1.w2) - () = 0
(-1 + 1.w3) * (1.w3) - () = 0
(-1.w2) * (1.w3) - (-1.w1) = 0
```

To understand this output, its helpful to multiply out the constraints
and simplify them a little.  Doing this gives us:

```
(w2 * w2) - w2 == 0
(w3 * w3) - w3 == 0
w1 - (w2 * w3) == 0
```

To understand these constraints, its helpful to know that `w2`
corresponds with `a`, `w3` corresponds with `b` and `w1` with `out`.
Then, the first two constraints correspond to `a * (a-1) === 0;` and
`b * (b-1) === 0;` from our circom file, whilst the last constraint
corresponds with `out <== a * b;`.
