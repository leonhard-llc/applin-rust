applin-rust
======
[![crates.io version](https://img.shields.io/crates/v/applin.svg)](https://crates.io/crates/applin)
[![unsafe forbidden](https://raw.githubusercontent.com/leonhard-llc/applin-rust/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![pipeline status](https://github.com/leonhard-llc/applin-rust/workflows/CI/badge.svg)](https://github.com/leonhard-llc/applin-rust/actions)

You can use the Applin™ Server-Driven UI framework to build a mobile app
with only server-side code.

<https://www.applin.dev/>

Use a provided boilerplate app for the frontend.
Use this library for the backend.

Documentation: <https://www.applin.dev/docs/>

Example and live demo: <https://github.com/leonhard-llc/applin-rust-demo>

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

0/0        0/0          0/0    0/0     0/0      🔒  applin 0.2.7
0/0        7/20         0/0    0/0     0/0      ☢️  ├── nanorand 0.7.0
3/7        47/225       0/1    0/0     1/3      ☢️  │   └── getrandom 0.2.12
0/0        0/0          0/0    0/0     0/0      ❓  │       ├── cfg-if 1.0.0
1/90       10/582       0/2    0/0     5/63     ☢️  │       └── libc 0.2.152
0/0        75/121       5/9    0/0     2/4      ☢️  ├── once_cell 1.19.0
0/0        5/5          0/0    0/0     0/0      ☢️  ├── serde 1.0.195
0/0        0/0          0/0    0/0     0/0      ❓  │   └── serde_derive 1.0.195
0/0        15/15        0/0    0/0     3/3      ☢️  │       ├── proc-macro2 1.0.78
0/0        4/4          0/0    0/0     0/0      ☢️  │       │   └── unicode-ident 1.0.12
0/0        0/0          0/0    0/0     0/0      ❓  │       ├── quote 1.0.35
0/0        15/15        0/0    0/0     3/3      ☢️  │       │   └── proc-macro2 1.0.78
0/0        80/80        3/3    0/0     2/2      ☢️  │       └── syn 2.0.48
0/0        15/15        0/0    0/0     3/3      ☢️  │           ├── proc-macro2 1.0.78
0/0        0/0          0/0    0/0     0/0      ❓  │           ├── quote 1.0.35
0/0        4/4          0/0    0/0     0/0      ☢️  │           └── unicode-ident 1.0.12
0/0        0/0          0/0    0/0     0/0      🔒  └── servlin 0.4.3
0/0        4/4          0/0    0/0     2/2      ☢️      ├── async-fs 1.6.0
                                                           │   [build-dependencies]
0/0        0/0          0/0    0/0     0/0      ❓      │   └── autocfg 1.1.0
4/4        230/230      40/40  0/0     12/12    ☢️      │   ├── async-lock 2.8.0
0/0        106/116      4/8    0/0     0/0      ☢️      │   │   └── event-listener 2.5.3
0/0        0/0          0/0    0/0     0/0      🔒      │   ├── blocking 1.5.1
0/0        0/0          0/0    0/0     0/0      🔒      │   │   ├── async-channel 2.1.1
0/0        170/170      2/2    0/0     1/1      ☢️      │   │   │   ├── concurrent-queue 2.4.0
4/4        12/76        4/16   0/0     0/3      ☢️      │   │   │   │   └── crossbeam-utils 0.8.19
0/0        59/87        6/12   0/0     0/0      ☢️      │   │   │   ├── event-listener 4.0.3
0/0        170/170      2/2    0/0     1/1      ☢️      │   │   │   │   ├── concurrent-queue 2.4.0
0/0        0/0          0/0    0/0     0/0      🔒      │   │   │   │   ├── parking 2.2.0
0/0        11/191       0/0    0/0     2/2      ☢️      │   │   │   │   └── pin-project-lite 0.2.13
0/0        2/2          0/0    0/0     0/0      ☢️      │   │   │   ├── event-listener-strategy 0.4.0
0/0        59/87        6/12   0/0     0/0      ☢️      │   │   │   │   ├── event-listener 4.0.3
0/0        11/191       0/0    0/0     2/2      ☢️      │   │   │   │   └── pin-project-lite 0.2.13
0/0        37/37        2/2    0/0     0/0      ☢️      │   │   │   ├── futures-core 0.3.30
0/0        11/191       0/0    0/0     2/2      ☢️      │   │   │   └── pin-project-lite 0.2.13
4/4        228/228      40/40  0/0     13/13    ☢️      │   │   ├── async-lock 3.3.0
0/0        59/87        6/12   0/0     0/0      ☢️      │   │   │   ├── event-listener 4.0.3
0/0        2/2          0/0    0/0     0/0      ☢️      │   │   │   ├── event-listener-strategy 0.4.0
0/0        11/191       0/0    0/0     2/2      ☢️      │   │   │   └── pin-project-lite 0.2.13
1/1        860/866      4/4    0/0     13/13    ☢️      │   │   ├── async-task 4.7.0
0/0        0/0          0/0    0/0     0/0      🔒      │   │   ├── fastrand 2.0.1
0/0        0/0          0/0    0/0     0/0      ❓      │   │   ├── futures-io 0.3.30
0/0        0/0          0/0    0/0     0/0      ❓      │   │   ├── futures-lite 2.2.0
0/0        0/0          0/0    0/0     0/0      🔒      │   │   │   ├── fastrand 2.0.1
0/0        37/37        2/2    0/0     0/0      ☢️      │   │   │   ├── futures-core 0.3.30
0/0        0/0          0/0    0/0     0/0      ❓      │   │   │   ├── futures-io 0.3.30
27/41      2047/2495    2/2    0/0     109/147  ☢️      │   │   │   ├── memchr 2.7.1
2/2        18/20        1/1    0/0     0/0      ☢️      │   │   │   │   └── log 0.4.20
0/0        5/5          0/0    0/0     0/0      ☢️      │   │   │   │       └── serde 1.0.195
0/0        0/0          0/0    0/0     0/0      🔒      │   │   │   ├── parking 2.2.0
0/0        11/191       0/0    0/0     2/2      ☢️      │   │   │   └── pin-project-lite 0.2.13
0/0        28/28        2/2    0/0     0/0      ☢️      │   │   ├── piper 0.2.1
0/0        33/33        2/2    0/0     0/0      ☢️      │   │   │   ├── atomic-waker 1.1.2
0/0        0/0          0/0    0/0     0/0      🔒      │   │   │   ├── fastrand 2.0.1
0/0        0/0          0/0    0/0     0/0      ❓      │   │   │   └── futures-io 0.3.30
0/0        14/14        1/1    0/0     0/0      ☢️      │   │   └── tracing 0.1.40
2/2        18/20        1/1    0/0     0/0      ☢️      │   │       ├── log 0.4.20
0/0        11/191       0/0    0/0     2/2      ☢️      │   │       ├── pin-project-lite 0.2.13
0/0        96/96        5/5    0/0     2/2      ☢️      │   │       └── tracing-core 0.1.32
0/0        75/121       5/9    0/0     2/4      ☢️      │   │           └── once_cell 1.19.0
0/0        0/0          0/0    0/0     0/0      ❓      │   └── futures-lite 1.13.0
0/0        0/0          0/0    0/0     0/0      🔒      │       ├── fastrand 1.9.0
0/0        37/37        2/2    0/0     0/0      ☢️      │       ├── futures-core 0.3.30
0/0        0/0          0/0    0/0     0/0      ❓      │       ├── futures-io 0.3.30
27/41      2047/2495    2/2    0/0     109/147  ☢️      │       ├── memchr 2.7.1
0/0        0/0          0/0    0/0     0/0      🔒      │       ├── parking 2.2.0
0/0        11/191       0/0    0/0     2/2      ☢️      │       ├── pin-project-lite 0.2.13
0/0        0/0          0/0    0/0     0/0      🔒      │       └── waker-fn 1.1.1
0/0        0/0          0/0    0/0     0/0      🔒      ├── async-net 1.8.0
0/0        2/4          0/0    0/0     0/0      ☢️      │   ├── async-io 1.13.0
                                                           │   │   [build-dependencies]
0/0        0/0          0/0    0/0     0/0      ❓      │   │   └── autocfg 1.1.0
4/4        230/230      40/40  0/0     12/12    ☢️      │   │   ├── async-lock 2.8.0
0/0        0/0          0/0    0/0     0/0      ❓      │   │   ├── cfg-if 1.0.0
0/0        170/170      2/2    0/0     1/1      ☢️      │   │   ├── concurrent-queue 2.4.0
0/0        0/0          0/0    0/0     0/0      ❓      │   │   ├── futures-lite 1.13.0
2/2        18/20        1/1    0/0     0/0      ☢️      │   │   ├── log 0.4.20
0/0        0/0          0/0    0/0     0/0      🔒      │   │   ├── parking 2.2.0
0/1        11/250       5/16   1/4     0/5      ☢️      │   │   ├── polling 2.8.0
0/0        0/0          0/0    0/0     0/0      ❓      │   │   │   ├── cfg-if 1.0.0
1/90       10/582       0/2    0/0     5/63     ☢️      │   │   │   ├── libc 0.2.152
2/2        18/20        1/1    0/0     0/0      ☢️      │   │   │   └── log 0.4.20
                                                           │   │   │   [build-dependencies]
0/0        0/0          0/0    0/0     0/0      ❓      │   │   │   └── autocfg 1.1.0
44/371     1847/6661    1/2    0/0     6/22     ☢️      │   │   ├── rustix 0.37.27
0/0        0/0          0/0    0/0     0/0      ❓      │   │   │   ├── bitflags 1.3.2
0/0        35/103       0/0    0/0     0/0      ☢️      │   │   │   ├── errno 0.3.8
1/90       10/582       0/2    0/0     5/63     ☢️      │   │   │   │   └── libc 0.2.152
0/0        24/666       27/36  2/2     6/14     ☢️      │   │   │   ├── io-lifetimes 1.0.11
1/90       10/582       0/2    0/0     5/63     ☢️      │   │   │   │   ├── libc 0.2.152
3/6        542/675      2/4    0/0     3/4      ☢️      │   │   │   │   └── socket2 0.4.10
1/90       10/582       0/2    0/0     5/63     ☢️      │   │   │   │       └── libc 0.2.152
0/0        7/7          0/0    0/0     0/0      ☢️      │   │   │   ├── itoa 1.0.10
1/90       10/582       0/2    0/0     5/63     ☢️      │   │   │   └── libc 0.2.152
0/0        24/24        0/0    0/0     3/3      ☢️      │   │   ├── slab 0.4.9
                                                           │   │   │   [build-dependencies]
0/0        0/0          0/0    0/0     0/0      ❓      │   │   │   └── autocfg 1.1.0
0/0        5/5          0/0    0/0     0/0      ☢️      │   │   │   └── serde 1.0.195
3/6        542/675      2/4    0/0     3/4      ☢️      │   │   ├── socket2 0.4.10
0/0        0/0          0/0    0/0     0/0      🔒      │   │   └── waker-fn 1.1.1
0/0        0/0          0/0    0/0     0/0      🔒      │   ├── blocking 1.5.1
0/0        0/0          0/0    0/0     0/0      ❓      │   └── futures-lite 1.13.0
0/0        0/0          0/0    0/0     0/0      🔒      ├── fixed-buffer 0.5.0
0/0        0/0          0/0    0/0     0/0      ❓      │   └── futures-io 0.3.30
0/0        0/0          0/0    0/0     0/0      ❓      ├── futures-io 0.3.30
0/0        0/0          0/0    0/0     0/0      ❓      ├── futures-lite 1.13.0
0/0        75/121       5/9    0/0     2/4      ☢️      ├── once_cell 1.19.0
0/0        0/0          0/0    0/0     0/0      🔒      ├── permit 0.2.1
0/0        32/32        0/0    0/0     0/0      ☢️      ├── rand 0.8.5
1/90       10/582       0/2    0/0     5/63     ☢️      │   ├── libc 0.2.152
2/2        18/20        1/1    0/0     0/0      ☢️      │   ├── log 0.4.20
0/0        0/0          0/0    0/0     0/0      ❓      │   ├── rand_chacha 0.3.1
2/2        636/712      0/0    0/0     17/25    ☢️      │   │   ├── ppv-lite86 0.2.17
0/0        2/2          0/0    0/0     0/0      ☢️      │   │   ├── rand_core 0.6.4
3/7        47/225       0/1    0/0     1/3      ☢️      │   │   │   ├── getrandom 0.2.12
0/0        5/5          0/0    0/0     0/0      ☢️      │   │   │   └── serde 1.0.195
0/0        5/5          0/0    0/0     0/0      ☢️      │   │   └── serde 1.0.195
0/0        2/2          0/0    0/0     0/0      ☢️      │   ├── rand_core 0.6.4
0/0        5/5          0/0    0/0     0/0      ☢️      │   └── serde 1.0.195
0/0        0/0          0/0    0/0     0/0      🔒      ├── safe-regex 0.2.5
0/0        0/0          0/0    0/0     0/0      🔒      │   └── safe-regex-macro 0.2.5
0/0        0/0          0/0    0/0     0/0      🔒      │       ├── safe-proc-macro2 1.0.67
0/0        4/4          0/0    0/0     0/0      ☢️      │       │   └── unicode-ident 1.0.12
0/0        0/0          0/0    0/0     0/0      🔒      │       └── safe-regex-compiler 0.2.5
0/0        0/0          0/0    0/0     0/0      🔒      │           ├── safe-proc-macro2 1.0.67
0/0        0/0          0/0    0/0     0/0      🔒      │           └── safe-quote 1.0.15
0/0        0/0          0/0    0/0     0/0      🔒      │               └── safe-proc-macro2 1.0.67
0/0        0/0          0/0    0/0     0/0      🔒      ├── safina-executor 0.3.3
0/0        0/0          0/0    0/0     0/0      🔒      │   ├── safina-sync 0.2.4
0/0        0/0          0/0    0/0     0/0      🔒      │   └── safina-threadpool 0.2.4
0/0        0/0          0/0    0/0     0/0      🔒      ├── safina-sync 0.2.4
0/0        0/0          0/0    0/0     0/0      🔒      ├── safina-timer 0.1.11
0/0        75/121       5/9    0/0     2/4      ☢️      │   └── once_cell 1.19.0
0/0        5/5          0/0    0/0     0/0      ☢️      ├── serde 1.0.195
0/0        4/7          0/0    0/0     0/0      ☢️      ├── serde_json 1.0.111
0/0        7/7          0/0    0/0     0/0      ☢️      │   ├── itoa 1.0.10
7/9        579/715      0/0    0/0     2/2      ☢️      │   ├── ryu 1.0.16
0/0        5/5          0/0    0/0     0/0      ☢️      │   └── serde 1.0.195
0/0        0/0          0/0    0/0     0/0      🔒      ├── temp-dir 0.1.12
0/0        0/0          0/0    0/0     0/0      🔒      ├── temp-file 0.1.8
0/0        0/0          0/0    0/0     0/0      ❓      └── url 2.5.0
0/0        2/2          0/0    0/0     0/0      ☢️          ├── form_urlencoded 1.2.1
0/0        8/8          0/0    0/0     0/0      ☢️          │   └── percent-encoding 2.3.1
0/0        0/0          0/0    0/0     0/0      ❓          ├── idna 0.5.0
0/0        5/5          0/0    0/0     0/0      ☢️          │   ├── unicode-bidi 0.3.15
0/0        5/5          0/0    0/0     0/0      ☢️          │   │   └── serde 1.0.195
0/0        20/20        0/0    0/0     0/0      ☢️          │   └── unicode-normalization 0.1.22
0/0        0/0          0/0    0/0     0/0      🔒          │       └── tinyvec 1.6.0
0/0        5/5          0/0    0/0     0/0      ☢️          │           ├── serde 1.0.195
0/0        0/0          0/0    0/0     0/0      🔒          │           └── tinyvec_macros 0.1.1
0/0        8/8          0/0    0/0     0/0      ☢️          ├── percent-encoding 2.3.1
0/0        5/5          0/0    0/0     0/0      ☢️          └── serde 1.0.195

102/542    7978/15638   158/210 3/6     204/345

```
# Changelog
- v0.2.7
    - Support `ApplinIos` 0.38.0.
    - Add `checkbox_button`.
- v0.2.6 - Add `RowList::push`, `RowList::new`, `OptWidgetList::push`, and `OptWidgetList::new`.
- v0.2.5 - Add `WidgetList::push` and `WidgetList::new`.
- v0.2.4
    - Support `ApplinIos` 0.36.0.
    - Add `reset_var` and `stop_actions` actions.
- v0.2.3 - Add `id` field to `Action` and `ModalButton`, for testing.
- v0.2.2 - Add `with_validated` to input widgets. Supports `ApplinIos` 0.33.0.
- v0.2.1
    - Add `id` fields for testing.
    - Make `Action` fields public.
    - Make `Real32` and enums `Copy`.
- v0.2.0
    - Support `ApplinIos` 0.32.0.
    - Remove `on_user_error_poll` action and make it a parameter of the `rpc` action.
- v0.1.7
    - Support `ApplinIos` 0.31.0.
    - Add `modal` action.
    - Add `aspect_ratio` to `choose_photo` and `take_photo` actions.
- v0.1.6
    - Support `ApplinIos` 0.28.0.
    - Add `logout` action.
    - Add `selector` widget.
- v0.1.5
    - Support `ApplinIos` 0.25.0.
    - Add `poll_delay_ms` to checkbox and textfield.
    - Replace checkbox `rpc` field with `actions`.
- v0.1.4
    - Add `From<Option<Into<Widget>>>` for `Widget` for use with `error_text`.
    - Add `SessionCookie`, `Id`, and `Secret`.
    - Organize into modules for easier discovery.
- v0.1.3 - Add `on_user_error_poll` action.
- v0.1.2 - Bugfixes
- v0.1.1 - Update documentation.
- v0.1.0 - First published version
