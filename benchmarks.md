
# Monte Carlo Benchmarks for Calculating Pi in Rust

I used [Hyperfine](https://github.com/sharkdp/hyperfine) to run a bunch of benchmarks
on [my Monte Carlo Pi simulation in Rust](README.md), and I wanted to share what I learned.

These tests were done on my October 2020 iMac with a 3.6 GHz 10-Core Intel Core i9 processor and 32 GB of RAM.

The column to pay attention to is "relative", where 1.00 represents the best run.  
I will also have that line bolded.


First, the development binary, 1 million points on a 1 million x 1 million grid:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/debug/monte_carlo -c 1000000 -b 1000 -n 1 -g 1000000 ` | 893.1 ± 8.2 | 887.0 | 915.7 | 3.77 ± 0.09 |
| `target/debug/monte_carlo -c 1000000 -b 1000 -n 2 -g 1000000 ` | 449.7 ± 3.5 | 446.2 | 456.1 | 1.90 ± 0.04 |
| `target/debug/monte_carlo -c 1000000 -b 1000 -n 3 -g 1000000 ` | 305.4 ± 4.5 | 302.1 | 315.6 | 1.29 ± 0.03 |
| **`target/debug/monte_carlo -c 1000000 -b 1000 -n 4 -g 1000000 `** | **236.7 ± 5.0** | **230.7** | **245.6** | **1.00** |


Now let's try the release/production binary with the same settings:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/monte_carlo -c 1000000 -b 1000 -n 1 -g 1000000 ` | 16.7 ± 0.8 | 15.8 | 23.9 | 2.56 ± 0.23 |
| `target/release/monte_carlo -c 1000000 -b 1000 -n 2 -g 1000000 ` | 10.2 ± 0.3 | 9.6 | 11.6 | 1.56 ± 0.13 |
| `target/release/monte_carlo -c 1000000 -b 1000 -n 3 -g 1000000 ` | 7.8 ± 0.9 | 7.1 | 19.3 | 1.19 ± 0.17 |
| **`target/release/monte_carlo -c 1000000 -b 1000 -n 4 -g 1000000 `** | **6.5 ± 0.5** | **5.9** | **9.3** | **1.00** |

Whoa!  Much faster!  That last run took just 6.5 _milliseconds_!

Going forward, we'll stick with the release binary, and let's try 10 million points:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/monte_carlo -c 10000000 -b 1000 -n 1 -g 1000000 ` | 139.3 ± 1.3 | 136.9 | 142.2 | 3.32 ± 0.20 |
| `target/release/monte_carlo -c 10000000 -b 1000 -n 2 -g 1000000 ` | 78.6 ± 1.8 | 76.9 | 87.9 | 1.87 ± 0.12 |
| `target/release/monte_carlo -c 10000000 -b 1000 -n 3 -g 1000000 ` | 54.2 ± 3.3 | 51.8 | 74.4 | 1.29 ± 0.11 |
| **`target/release/monte_carlo -c 10000000 -b 1000 -n 4 -g 1000000 `** | **42.0 ± 2.6** | **39.3** | **58.2** | **1.00** |


Now let's try upping the batch size (the number of points generated per batch in a thread) 
from 1000 to 10,000:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/monte_carlo -c 10000000 -b 10000 -n 1 -g 1000000 ` | 135.4 ± 1.7 | 132.7 | 139.5 | 3.40 ± 0.13 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 2 -g 1000000 ` | 71.5 ± 2.7 | 69.8 | 87.0 | 1.80 ± 0.09 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 3 -g 1000000 ` | 49.9 ± 1.0 | 48.2 | 52.3 | 1.25 ± 0.05 |
| **`target/release/monte_carlo -c 10000000 -b 10000 -n 4 -g 1000000 `** | **39.8 ± 1.4** | **37.6** | **45.2** | **1.00** |


We got a slight speedup, so let's try a batch size of 100,000 points:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/monte_carlo -c 10000000 -b 100000 -n 1 -g 1000000 ` | 150.8 ± 4.8 | 145.9 | 168.5 | 3.22 ± 0.23 |
| `target/release/monte_carlo -c 10000000 -b 100000 -n 2 -g 1000000 ` | 80.8 ± 2.0 | 77.6 | 84.4 | 1.72 ± 0.12 |
| `target/release/monte_carlo -c 10000000 -b 100000 -n 3 -g 1000000 ` | 58.1 ± 2.9 | 54.2 | 72.9 | 1.24 ± 0.10 |
| **`target/release/monte_carlo -c 10000000 -b 100000 -n 4 -g 1000000 `** | **46.8 ± 3.0** | **43.9** | **64.9** | **1.00** |


Oops--that got worse.  Let's go back to 10,000 points per batch, and try Turbo Mode:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/monte_carlo -c 10000000 -b 10000 -n 1 -g 1000000 --turbo` | 129.4 ± 1.9 | 127.3 | 135.7 | 3.37 ± 0.21 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 2 -g 1000000 --turbo` | 68.3 ± 1.7 | 66.9 | 78.4 | 1.78 ± 0.12 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 3 -g 1000000 --turbo` | 48.0 ± 1.4 | 46.2 | 53.1 | 1.25 ± 0.08 |
| **`target/release/monte_carlo -c 10000000 -b 10000 -n 4 -g 1000000 --turbo`** | **38.4 ± 2.3** | **35.8** | **53.1** | **1.00** |

That was a small, but noticeable boost.


Now let's try caching:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/monte_carlo -c 10000000 -b 10000 -n 1 -g 1000000 --cache` | 225.9 ± 4.0 | 220.1 | 234.4 | 2.24 ± 0.07 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 2 -g 1000000 --cache` | 146.4 ± 2.7 | 142.7 | 155.0 | 1.45 ± 0.05 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 3 -g 1000000 --cache` | 116.9 ± 6.5 | 111.7 | 142.4 | 1.16 ± 0.07 |
| **`target/release/monte_carlo -c 10000000 -b 10000 -n 4 -g 1000000 --cache`** | **101.0 ± 2.7** | **96.9** | **107.0** | **1.00** |

Huh--that took much longer, over twice as long, in fact!


Let's try pre-computing the cache instead:

cache-precompute.md
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/monte_carlo -c 10000000 -b 10000 -n 1 -g 1000000 --cache-precompute` | 220.5 ± 7.4 | 210.3 | 240.9 | 2.35 ± 0.14 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 2 -g 1000000 --cache-precompute` | 139.1 ± 3.5 | 134.2 | 145.7 | 1.48 ± 0.08 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 3 -g 1000000 --cache-precompute` | 109.6 ± 3.4 | 104.8 | 116.6 | 1.17 ± 0.07 |
| **`target/release/monte_carlo -c 10000000 -b 10000 -n 4 -g 1000000 --cache-precompute`** | **93.7 ± 4.5** | **89.7** | **114.9** | **1.00** |

Well, it got a little better, but is still roughly 2x the time it took just in turbo mode.
It seems that the extra clock cycles spent consulting the cache just isn't worth it.


Now let's try jacking up the grid from 1 million to 100 million points per axis:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/monte_carlo -c 10000000 -b 10000 -n 1 -g 100000000 ` | 204.5 ± 2.9 | 202.0 | 213.3 | 3.53 ± 0.09 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 2 -g 100000000 ` | 106.8 ± 2.8 | 105.1 | 119.0 | 1.84 ± 0.06 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 3 -g 100000000 ` | 75.1 ± 6.0 | 71.7 | 108.4 | 1.29 ± 0.11 |
| **`target/release/monte_carlo -c 10000000 -b 10000 -n 4 -g 100000000 `** | **58.0 ± 1.2** | **56.3** | **61.0** | **1.00** |

Not too much longer than the first run with 1 million points in the grid, but not too much more
accurate--I'm still only seeing Pi calculated correctly to three decimal places.


Now let's try that run again with the cache:

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/monte_carlo -c 10000000 -b 10000 -n 1 -g 100000000 --cache` | 1.043 ± 0.009 | 1.032 | 1.062 | 1.07 ± 0.02 |
| **`target/release/monte_carlo -c 10000000 -b 10000 -n 2 -g 100000000 --cache`** | **0.977 ± 0.015** | **0.963** | **1.010** | **1.00** |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 3 -g 100000000 --cache` | 1.016 ± 0.022 | 0.980 | 1.058 | 1.04 ± 0.03 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 4 -g 100000000 --cache` | 1.118 ± 0.182 | 1.047 | 1.635 | 1.14 ± 0.19 |

WOW--not only did that take much longer (5x for a single thread), adding more cores made it longer.
This is probably because each core has its own cache and there was excessive memory usage for an 
array with 100 million elements.  Also, Rust can have an array with 100 million elements!

Okay, let's try pre-computing the cache:

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| **`target/release/monte_carlo -c 10000000 -b 10000 -n 1 -g 100000000 --cache-precompute`** | **1.089 ± 0.010** | **1.072** | **1.108** | **1.00** |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 2 -g 100000000 --cache-precompute` | 1.112 ± 0.012 | 1.093 | 1.138 | 1.02 ± 0.01 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 3 -g 100000000 --cache-precompute` | 1.253 ± 0.010 | 1.238 | 1.266 | 1.15 ± 0.01 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 4 -g 100000000 --cache-precompute` | 1.431 ± 0.018 | 1.408 | 1.472 | 1.31 ± 0.02 |

Oh god.  The numbers got even worse.  I suppose pre-populating an array with 100 million elements
wasn't the best idea.

Time to get stupid.  Let's try a grid with **1 billion** points:

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/monte_carlo -c 10000000 -b 10000 -n 1 -g 1000000000 ` | 143.6 ± 2.9 | 140.0 | 150.4 | 3.42 ± 0.18 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 2 -g 1000000000 ` | 76.3 ± 1.6 | 74.1 | 81.6 | 1.82 ± 0.10 |
| `target/release/monte_carlo -c 10000000 -b 10000 -n 3 -g 1000000000 ` | 53.3 ± 2.4 | 51.7 | 67.9 | 1.27 ± 0.08 |
| **`target/release/monte_carlo -c 10000000 -b 10000 -n 4 -g 1000000000 `** | **42.0 ± 2.0** | **40.1** | **56.4** | **1.00** |

That actually worked!

Okay, let's get more stupid.  Stupider, if you will.  Let's try generating 100 million points:

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/monte_carlo -c 100000000 -b 10000 -n 1 -g 1000000000 ` | 1.388 ± 0.019 | 1.369 | 1.426 | 3.63 ± 0.06 |
| `target/release/monte_carlo -c 100000000 -b 10000 -n 2 -g 1000000000 ` | 0.721 ± 0.007 | 0.713 | 0.738 | 1.89 ± 0.03 |
| `target/release/monte_carlo -c 100000000 -b 10000 -n 3 -g 1000000000 ` | 0.496 ± 0.007 | 0.487 | 0.508 | 1.30 ± 0.02 |
| **`target/release/monte_carlo -c 100000000 -b 10000 -n 4 -g 1000000000 `** | **0.382 ± 0.004** | **0.376** | **0.388** | **1.00** |

Once again, Rust performed like a champ.  I'm now getting 4 points of precision with Pi.  
Not bad for .3 seconds of work.

## Conclusions

Rust code is really fast!

