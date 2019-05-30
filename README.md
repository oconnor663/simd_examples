# talk notes

- themes, atomics vs SIMD, machines don't work the way they used to
- define SIMD, SSE, AVX
- quick look through the core::arch docs
- sum_portable
- sum_avx2
- sum_avx2_static
- sum_avx2_dynamic
- mention helper libraries
- benchmaarks
- smaller and larger inputs
  - at 10_000_000 ints the relationship depends on turbo boost
- cargo asm portable, SSE2 autovectorization
- cpu native, AVX2 autovectorization
- what happens if you make an alignment mistake
- bounds checks and the godbolt diff
