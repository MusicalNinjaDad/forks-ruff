## comparing `931c66ad19dcaab4ef5feb3c26378f1d9946c31b` to main ...

```text
cargo bench -p ruff_benchmark --bench flake8_executable -- --baseline=main
   Compiling ruff_linter v0.11.5 (/workspaces/ruff/crates/ruff_linter)
   Compiling ruff_benchmark v0.0.0 (/workspaces/ruff/crates/ruff_benchmark)
    Finished `bench` profile [optimized] target(s) in 38.30s
     Running benches/flake8_executable.rs (target/release/deps/flake8_executable-06a3575d2cd1f0b1)
linter/flake8-executable/flake8_executable/ExeNoShebang.py
                        time:   [9.0996 µs 9.1445 µs 9.1924 µs]
                        thrpt:  [27.181 MiB/s 27.324 MiB/s 27.459 MiB/s]
                 change:
                        time:   [-5.8123% -4.9335% -4.0262%] (p = 0.00 < 0.05)
                        thrpt:  [+4.1951% +5.1895% +6.1709%]
                        Performance has improved.
Found 110 outliers among 1000 measurements (11.00%)
  43 (4.30%) high mild
  67 (6.70%) high severe
linter/flake8-executable/flake8_executable/ExeWithShebang.py
                        time:   [9.1686 µs 9.2228 µs 9.2824 µs]
                        thrpt:  [29.281 MiB/s 29.470 MiB/s 29.644 MiB/s]
                 change:
                        time:   [-6.2566% -5.2503% -4.2142%] (p = 0.00 < 0.05)
                        thrpt:  [+4.3996% +5.5412% +6.6742%]
                        Performance has improved.
Found 101 outliers among 1000 measurements (10.10%)
  49 (4.90%) high mild
  52 (5.20%) high severe
linter/flake8-executable/flake8_executable/NotExeNoShebang.py
                        time:   [9.0720 µs 9.1410 µs 9.2203 µs]
                        thrpt:  [27.203 MiB/s 27.439 MiB/s 27.647 MiB/s]
                 change:
                        time:   [-3.4066% -2.5059% -1.4969%] (p = 0.00 < 0.05)
                        thrpt:  [+1.5196% +2.5703% +3.5267%]
                        Performance has improved.
Found 125 outliers among 1000 measurements (12.50%)
  49 (4.90%) high mild
  76 (7.60%) high severe
linter/flake8-executable/flake8_executable/NotExeWithShebang.py
                        time:   [9.3081 µs 9.3810 µs 9.4674 µs]
                        thrpt:  [28.809 MiB/s 29.075 MiB/s 29.303 MiB/s]
                 change:
                        time:   [-2.1660% -1.3733% -0.5070%] (p = 0.00 < 0.05)
                        thrpt:  [+0.5096% +1.3925% +2.2139%]
                        Change within noise threshold.
Found 98 outliers among 1000 measurements (9.80%)
  45 (4.50%) high mild
  53 (5.30%) high severe
```

## The original (re-run) benchmarking `main` at 1000 samples in 20s ...

```text
cargo bench -p ruff_benchmark --bench flake8_executable -- --save-baseline=main
    Finished `bench` profile [optimized] target(s) in 0.32s
     Running benches/flake8_executable.rs (target/release/deps/flake8_executable-6b6bbfb4e17c2465)
linter/flake8-executable/flake8_executable/ExeNoShebang.py
                        time:   [9.7267 µs 9.7932 µs 9.8661 µs]
                        thrpt:  [25.325 MiB/s 25.514 MiB/s 25.688 MiB/s]
                 change:
                        time:   [+3.2807% +4.9205% +6.4474%] (p = 0.00 < 0.05)
                        thrpt:  [-6.0569% -4.6898% -3.1765%]
                        Performance has regressed.
Found 94 outliers among 1000 measurements (9.40%)
  41 (4.10%) high mild
  53 (5.30%) high severe
linter/flake8-executable/flake8_executable/ExeWithShebang.py
                        time:   [9.6341 µs 9.6931 µs 9.7574 µs]
                        thrpt:  [27.856 MiB/s 28.040 MiB/s 28.212 MiB/s]
                 change:
                        time:   [+2.5386% +3.5623% +4.6120%] (p = 0.00 < 0.05)
                        thrpt:  [-4.4086% -3.4398% -2.4757%]
                        Performance has regressed.
Found 106 outliers among 1000 measurements (10.60%)
  51 (5.10%) high mild
  55 (5.50%) high severe
linter/flake8-executable/flake8_executable/NotExeNoShebang.py
                        time:   [9.4330 µs 9.4998 µs 9.5750 µs]
                        thrpt:  [26.195 MiB/s 26.402 MiB/s 26.589 MiB/s]
                 change:
                        time:   [-3.1991% -1.6370% -0.2227%] (p = 0.01 < 0.05)
                        thrpt:  [+0.2232% +1.6642% +3.3048%]
                        Change within noise threshold.
Found 105 outliers among 1000 measurements (10.50%)
  51 (5.10%) high mild
  54 (5.40%) high severe
linter/flake8-executable/flake8_executable/NotExeWithShebang.py
                        time:   [9.4496 µs 9.5047 µs 9.5662 µs]
                        thrpt:  [28.512 MiB/s 28.696 MiB/s 28.864 MiB/s]
                 change:
                        time:   [-0.8933% +0.2019% +1.2545%] (p = 0.79 > 0.05)
                        thrpt:  [-1.2390% -0.2015% +0.9014%]
                        No change in performance detected.
Found 86 outliers among 1000 measurements (8.60%)
  45 (4.50%) high mild
  41 (4.10%) high severe
```
