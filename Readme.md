<h1 align="center">The Sumcheck protocol</h1>

`sumcheck` is a Rust library that implements the sumcheck for n-variate polynomials. 

The Sumcheck protocol is an interactive proof method that sums up all evaluations of n-variate polynomial `g` over boolean hypercube `{0,1}^n`

## Protocol brief overview
Let `g` be n-variate polynomial over finite field `F`. Let `g` have degree 3.

Compute sum of `g(x)` over input `x = {0,1}^n`

Task: offload hard work of computing a sum to prover P.

This is public coin procool, so we can apply Fiat-Shamir to make it non-interactive.

Procotol has of `n` rounds ( number of variables in polynomial `g`).

Verifier `V` time O(n) field ops

## Protocol steps
1. `P` sends claimed answer C,  `S1(x)` claimed to be equal `h(x)  = sum of g(x) over input (x1...xn)`
2. V picks random `r` from finite field `F` and sends it to P
3. V checks if `S(r) = h(r)` holds. Completeness: if prover P honest, this check will pass.
4. Repeat this process `n` rounds.
    Soundness error <= n/|F|. As long as field F is big enouph, we keep this probability negligible.

**WARNING**: This is an academic proof-of-concept prototype, and in particular has not received careful code review. This implementation is NOT ready for production use.

## Build guide

The library compiles on the `stable` toolchain of the Rust compiler. To install the latest version of Rust, first install `rustup` by following the instructions [here](https://rustup.rs/), or via your platform's package manager. Once `rustup` is installed, install the Rust toolchain by invoking:
```bash
rustup install stable
```

After that, use `cargo` (the standard Rust build tool) to build the library:
```bash
git clone https://github.com/cyberbono3/sumcheck.git
cargo build --release
```

This library comes with some unit and integration tests. Run these tests with:
```bash
cargo test
```

Lastly, this library is instrumented with profiling infrastructure that prints detailed traces of execution time. To enable this, compile with `cargo build --features print-trace`.

## License

This library is licensed under either of the following licenses, at your discretion.

* [Apache License Version 2.0](LICENSE-APACHE)
* [MIT License](LICENSE-MIT)

Unless you explicitly state otherwise, any contribution that you submit to this library shall be dual licensed as above (as defined in the Apache v2 License), without any additional terms or conditions.

## Reference Paper
[Proofs, Arguments, and Zero-Knowledge](https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf) <br/>
Justin Thaler







T
