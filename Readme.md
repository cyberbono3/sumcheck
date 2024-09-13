<align="center">Sumcheck Protocol</h1>

The Sumcheck protocol is an interactive proof method widely used in computer science, especially in fields like complexity theory and cryptography, to verify the accuracy of certain computations. It is a key technique in the realm of probabilistic proof systems and is essential for creating efficient zero-knowledge proofs and secure multi-party computation protocols

**WARNING**: This is a proof-of-concept prototype, and in particular has not received careful code review. 
This implementation is NOT ready for production use.

## Build guide

```bash
rustup install stable
```

After that, use `cargo` (the standard Rust build tool) to build the library:
```bash
git clone https://github.com/cyberbono3/sumcheck.git
cd sumcheck
cargo build --release
```

This library comes with some unit and integration tests. Run these tests with:
```bash
cargo test
```

## Reference Paper
[Libra: Succinct Zero-Knowledge Proofs with Optimal Prover Computation](https://eprint.iacr.org/2019/317) <br/>
Tiancheng Xie, Jiaheng Zhang, Yupeng Zhang, Charalampos Papamanthou, Dawn Song

[Time-Optimal Interactive Proofs for Circuit Evaluation](https://arxiv.org/abs/1304.3812) <br/>
Justin Thaler
