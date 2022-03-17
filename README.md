# Reservoir Sampling

I needed a objective to help me learn `Rust` and reservoir sampling became my excuse to learn `Rust`.
It allows perform weighted or unweighted sampling.
Constructive comments are welcome.


## Install
```bash
cargo \
   install \
      --root ~/.local
      --git https://github.com/SamuelLarkin/reservoir_sampling_rs.git
```
or
```bash
cargo \
   install \
      --root ~/.local
```


## Speed Tests
Let use a fairly big file to do some tests.

Let's see how long it takes simply to `gunzip` that file.
```bash
time zcat $PORTAGE/corpora/bac-lac.2021/bitextor_2018/201808/permanent/en-fr.deduped.txt.gz \
| wc
```
```bash
cmd: zcat /space/project/portage/corpora/bac-lac.2021/bitextor_2018/201808/permanent/en-fr.deduped.txt.gz
real: 164.90
user: 93.17
sys: 4.37

51381866 1316984132 16408315928
```

What if that same file was in plain text?
```bash
time wc test.txt
```
```bash
51381866  1316984132 16408315928 test.txt

cmd: wc test.txt
real: 162.70
user: 158.43
sys: 4.04
```
The longer time is probably due to network transfer as this file is on a NAS.
It is faster to transfer small files over the network.


What about an initial unweighted sampling?
```bash
time zcat $PORTAGE/corpora/bac-lac.2021/bitextor_2018/201808/permanent/en-fr.deduped.txt.gz \
| time cargo run --release unweighted --size 1000 \
| wc
```
```bash
    Finished release [optimized] target(s) in 14.90s
     Running `target/release/reservoir_sampling unweighted --size 1000`

cmd: zcat /space/project/portage/corpora/bac-lac.2021/bitextor_2018/201808/permanent/en-fr.deduped.txt.gz
real: 111.82
user: 89.11
sys: 4.03


cmd: cargo run --release unweighted --size 1000
real: 111.82
user: 9.85
sys: 3.53

1000
```
```bash
time cargo run --release unweighted --size 1000 test.txt \
| wc
```
```bash
    Finished release [optimized] target(s) in 0.13s
     Running `target/release/reservoir_sampling unweighted --size 1000 test.txt`

cmd: cargo run --release unweighted --size 1000 test.txt
real: 14.85
user: 9.01
sys: 5.63

1000
```

What about an initial unweighted sampling?
```bash
time cargo run --release weighted --size 1000 test.txt <(cut -f 8 < test.txt) \
| wc
```
```bash
    Finished release [optimized] target(s) in 0.19s
     Running `target/release/reservoir_sampling weighted --size 1000 test.txt /dev/fd/63`

cmd: cargo run --release weighted --size 1000 test.txt /dev/fd/63
real: 31.45
user: 12.67
sys: 5.30

1000
```


## Comparing with my Python Implementation
`benches/hyperfine.sh` samples 1000, 5000, 10000 samples from a stream of 500000 samples.
```bash
Benchmark #1: reservoir_sampling unweighted --size 50 benches/src.txt
  Time (mean ± σ):     678.6 ms ±  45.6 ms    [User: 312.8 ms, System: 84.0 ms]
  Range (min … max):   642.1 ms … 799.4 ms    10 runs

Benchmark #2: cargo run --release unweighted --size 50 < benches/src.txt
  Time (mean ± σ):     266.2 ms ±  10.3 ms    [User: 95.9 ms, System: 60.6 ms]
  Range (min … max):   254.2 ms … 284.9 ms    10 runs

Benchmark #3: reservoir_sampling weighted --size 50 benches/src.txt <(cut -f 8 < benches/src.txt)
  Time (mean ± σ):     795.2 ms ±  80.9 ms    [User: 356.2 ms, System: 89.1 ms]
  Range (min … max):   701.6 ms … 999.1 ms    10 runs

Benchmark #4: cargo run --release weighted --size 50 benches/src.txt <(cut -f 8 < benches/src.txt)
  Time (mean ± σ):     304.1 ms ±  15.0 ms    [User: 99.1 ms, System: 67.8 ms]
  Range (min … max):   287.3 ms … 329.2 ms    10 runs

Benchmark #5: reservoir_sampling unweighted --size 100 benches/src.txt
  Time (mean ± σ):     733.2 ms ± 161.6 ms    [User: 319.6 ms, System: 86.6 ms]
  Range (min … max):   644.0 ms … 1190.1 ms    10 runs

Benchmark #6: cargo run --release unweighted --size 100 < benches/src.txt
  Time (mean ± σ):     277.2 ms ±  30.0 ms    [User: 87.1 ms, System: 67.3 ms]
  Range (min … max):   252.2 ms … 356.7 ms    10 runs

Benchmark #7: reservoir_sampling weighted --size 100 benches/src.txt <(cut -f 8 < benches/src.txt)
  Time (mean ± σ):     828.1 ms ± 253.1 ms    [User: 351.6 ms, System: 89.2 ms]
  Range (min … max):   694.9 ms … 1529.9 ms    10 runs

Benchmark #8: cargo run --release weighted --size 100 benches/src.txt <(cut -f 8 < benches/src.txt)
  Time (mean ± σ):     299.8 ms ±  11.5 ms    [User: 98.0 ms, System: 71.5 ms]
  Range (min … max):   283.7 ms … 315.0 ms    10 runs

Benchmark #9: reservoir_sampling unweighted --size 500 benches/src.txt
  Time (mean ± σ):     890.8 ms ± 462.7 ms    [User: 317.9 ms, System: 89.6 ms]
  Range (min … max):   627.0 ms … 2132.0 ms    10 runs

Benchmark #10: cargo run --release unweighted --size 500 < benches/src.txt
  Time (mean ± σ):     293.1 ms ±  69.1 ms    [User: 89.9 ms, System: 68.3 ms]
  Range (min … max):   252.4 ms … 484.9 ms    10 runs

Benchmark #11: reservoir_sampling weighted --size 500 benches/src.txt <(cut -f 8 < benches/src.txt)
  Time (mean ± σ):     754.3 ms ± 109.1 ms    [User: 355.1 ms, System: 79.4 ms]
  Range (min … max):   666.7 ms … 990.8 ms    10 runs

Benchmark #12: cargo run --release weighted --size 500 benches/src.txt <(cut -f 8 < benches/src.txt)
  Time (mean ± σ):     294.7 ms ±  11.9 ms    [User: 95.0 ms, System: 74.2 ms]
  Range (min … max):   281.6 ms … 318.2 ms    10 runs

Benchmark #13: reservoir_sampling unweighted --size 1000 benches/src.txt
  Time (mean ± σ):     695.7 ms ±  80.5 ms    [User: 329.6 ms, System: 72.8 ms]
  Range (min … max):   645.0 ms … 922.5 ms    10 runs

Benchmark #14: cargo run --release unweighted --size 1000 < benches/src.txt
  Time (mean ± σ):     268.7 ms ±  27.3 ms    [User: 85.4 ms, System: 69.9 ms]
  Range (min … max):   238.6 ms … 334.3 ms    10 runs

Benchmark #15: reservoir_sampling weighted --size 1000 benches/src.txt <(cut -f 8 < benches/src.txt)
  Time (mean ± σ):     759.9 ms ± 111.5 ms    [User: 362.5 ms, System: 77.0 ms]
  Range (min … max):   679.6 ms … 997.5 ms    10 runs

Benchmark #16: cargo run --release weighted --size 1000 benches/src.txt <(cut -f 8 < benches/src.txt)
  Time (mean ± σ):     319.3 ms ±  73.0 ms    [User: 101.7 ms, System: 66.2 ms]
  Range (min … max):   278.8 ms … 524.4 ms    10 runs

Summary
  'cargo run --release unweighted --size 50 < benches/src.txt' ran
    1.01 ± 0.11 times faster than 'cargo run --release unweighted --size 1000 < benches/src.txt'
    1.04 ± 0.12 times faster than 'cargo run --release unweighted --size 100 < benches/src.txt'
    1.10 ± 0.26 times faster than 'cargo run --release unweighted --size 500 < benches/src.txt'
    1.11 ± 0.06 times faster than 'cargo run --release weighted --size 500 benches/src.txt <(cut -f 8 < benches/src.txt)'
    1.13 ± 0.06 times faster than 'cargo run --release weighted --size 100 benches/src.txt <(cut -f 8 < benches/src.txt)'
    1.14 ± 0.07 times faster than 'cargo run --release weighted --size 50 benches/src.txt <(cut -f 8 < benches/src.txt)'
    1.20 ± 0.28 times faster than 'cargo run --release weighted --size 1000 benches/src.txt <(cut -f 8 < benches/src.txt)'
    2.55 ± 0.20 times faster than 'reservoir_sampling unweighted --size 50 benches/src.txt'
    2.61 ± 0.32 times faster than 'reservoir_sampling unweighted --size 1000 benches/src.txt'
    2.75 ± 0.62 times faster than 'reservoir_sampling unweighted --size 100 benches/src.txt'
    2.83 ± 0.42 times faster than 'reservoir_sampling weighted --size 500 benches/src.txt <(cut -f 8 < benches/src.txt)'
    2.85 ± 0.43 times faster than 'reservoir_sampling weighted --size 1000 benches/src.txt <(cut -f 8 < benches/src.txt)'
    2.99 ± 0.33 times faster than 'reservoir_sampling weighted --size 50 benches/src.txt <(cut -f 8 < benches/src.txt)'
    3.11 ± 0.96 times faster than 'reservoir_sampling weighted --size 100 benches/src.txt <(cut -f 8 < benches/src.txt)'
    3.35 ± 1.74 times faster than 'reservoir_sampling unweighted --size 500 benches/src.txt'
```
