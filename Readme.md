Applin™
======
[![crates.io version](https://img.shields.io/crates/v/applin.svg)](https://crates.io/crates/applin)
[![unsafe forbidden](https://raw.githubusercontent.com/leonhard-llc/applin-rust/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![pipeline status](https://github.com/leonhard-llc/applin-rust/workflows/CI/badge.svg)](https://github.com/leonhard-llc/applin-rust/actions)

You can use the Applin™ Server-Driven UI framework to build a mobile app
with only server-side code.

<https://www.applin.dev/>

Use a provided boilerplate app for the frontend.
And use this library for the backend.

Documentation: <https://www.applin.dev/docs/>

Example: <https://github.com/leonhard-llc/applin-rust-demo>

# Cargo Geiger Safety Report
```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols: 
    🔒  = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ❓  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    ☢️  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Dependency

0/0        0/0          0/0    0/0     0/0      🔒  applin 0.1.0
0/0        0/5          0/0    0/0     0/0      ❓  ├── serde 1.0.190
0/0        0/0          0/0    0/0     0/0      ❓  │   └── serde_derive 1.0.190
0/0        0/15         0/0    0/0     0/3      ❓  │       ├── proc-macro2 1.0.69
0/0        0/4          0/0    0/0     0/0      ❓  │       │   └── unicode-ident 1.0.12
0/0        0/0          0/0    0/0     0/0      ❓  │       ├── quote 1.0.33
0/0        0/15         0/0    0/0     0/3      ❓  │       │   └── proc-macro2 1.0.69
0/0        0/79         0/3    0/0     0/2      ❓  │       └── syn 2.0.38
0/0        0/15         0/0    0/0     0/3      ❓  │           ├── proc-macro2 1.0.69
0/0        0/0          0/0    0/0     0/0      ❓  │           ├── quote 1.0.33
0/0        0/4          0/0    0/0     0/0      ❓  │           └── unicode-ident 1.0.12
0/0        0/0          0/0    0/0     0/0      🔒  └── servlin 0.4.0
0/0        0/4          0/0    0/0     0/2      ❓      ├── async-fs 1.6.0
0/4        0/230        0/40   0/0     0/12     ❓      │   ├── async-lock 2.8.0
0/0        0/116        0/8    0/0     0/0      ❓      │   │   └── event-listener 2.5.3
0/0        0/0          0/0    0/0     0/0      🔒      │   ├── blocking 1.4.1
0/0        0/0          0/0    0/0     0/0      🔒      │   │   ├── async-channel 1.9.0
0/0        0/170        0/2    0/0     0/1      ❓      │   │   │   ├── concurrent-queue 2.3.0
0/4        0/94         0/16   0/0     0/3      ❓      │   │   │   │   └── crossbeam-utils 0.8.16
0/0        0/0          0/0    0/0     0/0      ❓      │   │   │   │       └── cfg-if 1.0.0
0/0        0/116        0/8    0/0     0/0      ❓      │   │   │   ├── event-listener 2.5.3
0/0        0/37         0/2    0/0     0/0      ❓      │   │   │   └── futures-core 0.3.29
0/4        0/230        0/40   0/0     0/12     ❓      │   │   ├── async-lock 2.8.0
0/1        0/864        0/4    0/0     0/12     ❓      │   │   ├── async-task 4.5.0
0/0        0/0          0/0    0/0     0/0      🔒      │   │   ├── fastrand 2.0.1
0/0        0/0          0/0    0/0     0/0      ❓      │   │   ├── futures-io 0.3.29
0/0        0/0          0/0    0/0     0/0      ❓      │   │   ├── futures-lite 1.13.0
0/0        0/0          0/0    0/0     0/0      🔒      │   │   │   ├── fastrand 1.9.0
0/0        0/37         0/2    0/0     0/0      ❓      │   │   │   ├── futures-core 0.3.29
0/0        0/0          0/0    0/0     0/0      ❓      │   │   │   ├── futures-io 0.3.29
0/41       0/2501       0/2    0/0     0/147    ❓      │   │   │   ├── memchr 2.6.4
0/2        0/20         0/1    0/0     0/0      ❓      │   │   │   │   └── log 0.4.20
0/0        0/5          0/0    0/0     0/0      ❓      │   │   │   │       └── serde 1.0.190
0/0        0/0          0/0    0/0     0/0      🔒      │   │   │   ├── parking 2.2.0
0/0        0/191        0/0    0/0     0/2      ❓      │   │   │   ├── pin-project-lite 0.2.13
0/0        0/0          0/0    0/0     0/0      🔒      │   │   │   └── waker-fn 1.1.1
0/0        0/28         0/2    0/0     0/0      ❓      │   │   ├── piper 0.2.1
0/0        0/33         0/2    0/0     0/0      ❓      │   │   │   ├── atomic-waker 1.1.2
0/0        0/0          0/0    0/0     0/0      🔒      │   │   │   ├── fastrand 2.0.1
0/0        0/0          0/0    0/0     0/0      ❓      │   │   │   └── futures-io 0.3.29
0/0        0/14         0/1    0/0     0/0      ❓      │   │   └── tracing 0.1.40
0/2        0/20         0/1    0/0     0/0      ❓      │   │       ├── log 0.4.20
0/0        0/191        0/0    0/0     0/2      ❓      │   │       ├── pin-project-lite 0.2.13
0/0        0/96         0/5    0/0     0/2      ❓      │   │       └── tracing-core 0.1.32
0/0        0/121        0/9    0/0     0/4      ❓      │   │           └── once_cell 1.18.0
0/0        0/0          0/0    0/0     0/0      ❓      │   └── futures-lite 1.13.0
                                                           │   [build-dependencies]
0/0        0/0          0/0    0/0     0/0      ❓      │   └── autocfg 1.1.0
0/0        0/0          0/0    0/0     0/0      🔒      ├── async-net 1.8.0
0/0        0/4          0/0    0/0     0/0      ❓      │   ├── async-io 1.13.0
0/4        0/230        0/40   0/0     0/12     ❓      │   │   ├── async-lock 2.8.0
0/0        0/0          0/0    0/0     0/0      ❓      │   │   ├── cfg-if 1.0.0
0/0        0/170        0/2    0/0     0/1      ❓      │   │   ├── concurrent-queue 2.3.0
0/0        0/0          0/0    0/0     0/0      ❓      │   │   ├── futures-lite 1.13.0
0/2        0/20         0/1    0/0     0/0      ❓      │   │   ├── log 0.4.20
0/0        0/0          0/0    0/0     0/0      🔒      │   │   ├── parking 2.2.0
0/1        0/250        0/16   0/4     0/5      ❓      │   │   ├── polling 2.8.0
0/0        0/0          0/0    0/0     0/0      ❓      │   │   │   ├── cfg-if 1.0.0
0/90       0/554        0/2    0/0     0/55     ❓      │   │   │   ├── libc 0.2.149
0/2        0/20         0/1    0/0     0/0      ❓      │   │   │   └── log 0.4.20
                                                           │   │   │   [build-dependencies]
0/0        0/0          0/0    0/0     0/0      ❓      │   │   │   └── autocfg 1.1.0
0/371      0/6661       0/2    0/0     0/22     ❓      │   │   ├── rustix 0.37.27
0/0        0/0          0/0    0/0     0/0      ❓      │   │   │   ├── bitflags 1.3.2
0/0        0/100        0/0    0/0     0/0      ❓      │   │   │   ├── errno 0.3.5
0/90       0/554        0/2    0/0     0/55     ❓      │   │   │   │   └── libc 0.2.149
0/0        0/666        0/36   0/2     0/14     ❓      │   │   │   ├── io-lifetimes 1.0.11
0/90       0/554        0/2    0/0     0/55     ❓      │   │   │   │   ├── libc 0.2.149
0/6        0/675        0/4    0/0     0/4      ❓      │   │   │   │   └── socket2 0.4.10
0/90       0/554        0/2    0/0     0/55     ❓      │   │   │   │       └── libc 0.2.149
0/0        0/7          0/0    0/0     0/0      ❓      │   │   │   ├── itoa 1.0.9
0/90       0/554        0/2    0/0     0/55     ❓      │   │   │   └── libc 0.2.149
0/0        0/24         0/0    0/0     0/3      ❓      │   │   ├── slab 0.4.9
0/0        0/5          0/0    0/0     0/0      ❓      │   │   │   └── serde 1.0.190
                                                           │   │   │   [build-dependencies]
0/0        0/0          0/0    0/0     0/0      ❓      │   │   │   └── autocfg 1.1.0
0/6        0/675        0/4    0/0     0/4      ❓      │   │   ├── socket2 0.4.10
0/0        0/0          0/0    0/0     0/0      🔒      │   │   └── waker-fn 1.1.1
                                                           │   │   [build-dependencies]
0/0        0/0          0/0    0/0     0/0      ❓      │   │   └── autocfg 1.1.0
0/0        0/0          0/0    0/0     0/0      🔒      │   ├── blocking 1.4.1
0/0        0/0          0/0    0/0     0/0      ❓      │   └── futures-lite 1.13.0
0/0        0/0          0/0    0/0     0/0      🔒      ├── fixed-buffer 0.5.0
0/0        0/0          0/0    0/0     0/0      ❓      │   └── futures-io 0.3.29
0/0        0/0          0/0    0/0     0/0      ❓      ├── futures-io 0.3.29
0/0        0/0          0/0    0/0     0/0      ❓      ├── futures-lite 1.13.0
0/0        0/121        0/9    0/0     0/4      ❓      ├── once_cell 1.18.0
0/0        0/0          0/0    0/0     0/0      🔒      ├── permit 0.2.1
0/0        0/32         0/0    0/0     0/0      ❓      ├── rand 0.8.5
0/90       0/554        0/2    0/0     0/55     ❓      │   ├── libc 0.2.149
0/2        0/20         0/1    0/0     0/0      ❓      │   ├── log 0.4.20
0/0        0/0          0/0    0/0     0/0      ❓      │   ├── rand_chacha 0.3.1
0/2        0/712        0/0    0/0     0/25     ❓      │   │   ├── ppv-lite86 0.2.17
0/0        0/2          0/0    0/0     0/0      ❓      │   │   ├── rand_core 0.6.4
0/7        0/228        0/1    0/0     0/3      ❓      │   │   │   ├── getrandom 0.2.10
0/0        0/0          0/0    0/0     0/0      ❓      │   │   │   │   ├── cfg-if 1.0.0
0/90       0/554        0/2    0/0     0/55     ❓      │   │   │   │   └── libc 0.2.149
0/0        0/5          0/0    0/0     0/0      ❓      │   │   │   └── serde 1.0.190
0/0        0/5          0/0    0/0     0/0      ❓      │   │   └── serde 1.0.190
0/0        0/2          0/0    0/0     0/0      ❓      │   ├── rand_core 0.6.4
0/0        0/5          0/0    0/0     0/0      ❓      │   └── serde 1.0.190
0/0        0/0          0/0    0/0     0/0      🔒      ├── safe-regex 0.2.5
0/0        0/0          0/0    0/0     0/0      🔒      │   └── safe-regex-macro 0.2.5
0/0        0/0          0/0    0/0     0/0      🔒      │       ├── safe-proc-macro2 1.0.67
0/0        0/4          0/0    0/0     0/0      ❓      │       │   └── unicode-ident 1.0.12
0/0        0/0          0/0    0/0     0/0      🔒      │       └── safe-regex-compiler 0.2.5
0/0        0/0          0/0    0/0     0/0      🔒      │           ├── safe-proc-macro2 1.0.67
0/0        0/0          0/0    0/0     0/0      🔒      │           └── safe-quote 1.0.15
0/0        0/0          0/0    0/0     0/0      🔒      │               └── safe-proc-macro2 1.0.67
0/0        0/0          0/0    0/0     0/0      🔒      ├── safina-executor 0.3.3
0/0        0/0          0/0    0/0     0/0      🔒      │   ├── safina-sync 0.2.4
0/0        0/0          0/0    0/0     0/0      🔒      │   └── safina-threadpool 0.2.4
0/0        0/0          0/0    0/0     0/0      🔒      ├── safina-sync 0.2.4
0/0        0/0          0/0    0/0     0/0      🔒      ├── safina-timer 0.1.11
0/0        0/121        0/9    0/0     0/4      ❓      │   └── once_cell 1.18.0
0/0        0/5          0/0    0/0     0/0      ❓      ├── serde 1.0.190
0/0        0/7          0/0    0/0     0/0      ❓      ├── serde_json 1.0.108
0/0        0/7          0/0    0/0     0/0      ❓      │   ├── itoa 1.0.9
0/9        0/715        0/0    0/0     0/2      ❓      │   ├── ryu 1.0.15
0/0        0/5          0/0    0/0     0/0      ❓      │   └── serde 1.0.190
0/0        0/0          0/0    0/0     0/0      🔒      ├── temp-dir 0.1.11
0/0        0/0          0/0    0/0     0/0      🔒      ├── temp-file 0.1.7
0/0        0/0          0/0    0/0     0/0      ❓      └── url 2.4.1
0/0        0/2          0/0    0/0     0/0      ❓          ├── form_urlencoded 1.2.0
0/0        0/8          0/0    0/0     0/0      ❓          │   └── percent-encoding 2.3.0
0/0        0/0          0/0    0/0     0/0      ❓          ├── idna 0.4.0
0/0        0/5          0/0    0/0     0/0      ❓          │   ├── unicode-bidi 0.3.13
0/0        0/5          0/0    0/0     0/0      ❓          │   │   └── serde 1.0.190
0/0        0/20         0/0    0/0     0/0      ❓          │   └── unicode-normalization 0.1.22
0/0        0/0          0/0    0/0     0/0      🔒          │       └── tinyvec 1.6.0
0/0        0/5          0/0    0/0     0/0      ❓          │           ├── serde 1.0.190
0/0        0/0          0/0    0/0     0/0      🔒          │           └── tinyvec_macros 0.1.1
0/0        0/8          0/0    0/0     0/0      ❓          ├── percent-encoding 2.3.0
0/0        0/5          0/0    0/0     0/0      ❓          └── serde 1.0.190

0/538      0/15294      0/158  0/6     0/323  

```
# Changelog
- v0.1.0 - First published version
