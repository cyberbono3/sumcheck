<h1 align="center">Sumcheck</h1>

The Sumcheck protocol is an interactive proof method widely used in computer science, especially in fields like complexity theory and cryptography, to verify the accuracy of certain computations. It is a key technique in the realm of probabilistic proof systems and is essential for creating efficient zero-knowledge proofs and secure multi-party computation protocols

This is experimental implementation.
Below is my understanding of protocol.

## Goal
Let g be n-variate polynomial over finite filed F. Let g have degree 3.

Compute sum of g(x) over input x = {0,1}^n (hyper cube or bit vector)

Task: offload hard work of computing a sum to prover P.

This is publoc coin procool, so we can apply Fiat-Shamir to make it non-interactive.

Procotol has of n rounds ( number of variables in polynomial g).

Verifier V time O(n) field ops

## Protocol
1. P sends claimed answer C, he sends univariate polynomial) S1(x) claimed to be equal h(x)  = sum of g(x) over input (x1...xn)
2. V picks random r from finite field F and sends it to P
3. V checks if S(r) = h(r) holds. Completeness: if prover P honest, this check will pass.
4. Repeat this process n rounds. Soundness error <= n/|F|. As long as field F is big enouph, we keep this probability negligible.


This library comes with some unit and integration tests. Run these tests with:
```bash
cargo test
```

## Reference Paper
[Libra: Succinct Zero-Knowledge Proofs with Optimal Prover Computation](https://eprint.iacr.org/2019/317) <br/>
Tiancheng Xie, Jiaheng Zhang, Yupeng Zhang, Charalampos Papamanthou, Dawn Song

[Time-Optimal Interactive Proofs for Circuit Evaluation](https://arxiv.org/abs/1304.3812) <br/>
Justin Thaler
