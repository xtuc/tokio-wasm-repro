## How to repro

```
$ cargo b --target wasm32-wasi
$ wasmtime target/wasm32-wasi/debug/tokio-repro.wasm

paul
thread 'main' panicked at 'condvar wait not supported', library/std/src/sys/wasi/../unsupported/locks/condvar.rs:21:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Error: failed to run main module `target/wasm32-wasi/debug/tokio-repro.wasm`

Caused by:
    0: failed to invoke command default
    1: wasm trap: wasm `unreachable` instruction executed
       wasm backtrace:
           0: 0x4a919 - <unknown>!__rust_start_panic
           1: 0x4a61f - <unknown>!rust_panic
           2: 0x4a59c - <unknown>!std::panicking::rust_panic_with_hook::h5af4a166307aff48
           3: 0x497c7 - <unknown>!std::panicking::begin_panic_handler::{{closure}}::h42a9871ead2de5e8
           4: 0x49736 - <unknown>!std::sys_common::backtrace::__rust_end_short_backtrace::h42a6bde96d4a4a1f
           5: 0x49eec - <unknown>!rust_begin_unwind
           6: 0x4fde1 - <unknown>!core::panicking::panic_fmt::h33d7d4c3033d60da
           7: 0x4a715 - <unknown>!std::sys::wasi::locks::condvar::Condvar::wait::hdede92926e50dcc8
           8: 0x22941 - <unknown>!std::sys_common::condvar::Condvar::wait::h04c20b05bf1c2126
           9: 0x1d090 - <unknown>!std::sync::condvar::Condvar::wait::hcd6499f64c63a735
          10: 0x1ec4c - <unknown>!tokio::park::thread::Inner::park::h290738cfdfc96527
          11: 0x1ea54 - <unknown>!<tokio::park::thread::ParkThread as tokio::park::Park>::park::h0734a0114782d979
          12: 0x21aef - <unknown>!<tokio::runtime::driver::Driver as tokio::park::Park>::park::h2a5e05bccdfaba21
          13: 0x3f65a - <unknown>!tokio::runtime::scheduler::current_thread::Context::park::{{closure}}::hec71d978169e6a52
          14: 0x3f353 - <unknown>!tokio::runtime::scheduler::current_thread::Context::enter::h78fb76734733aeb8
          15: 0x3efa5 - <unknown>!tokio::runtime::scheduler::current_thread::Context::park::hc79bdd9471d6126b
          16: 0x42fc - <unknown>!tokio::runtime::scheduler::current_thread::CoreGuard::block_on::{{closure}}::h1028ceef227f7b07
          17: 0x3dcb - <unknown>!tokio::runtime::scheduler::current_thread::CoreGuard::enter::{{closure}}::hc4af641961022e0b
          18: 0x10c0 - <unknown>!tokio::macros::scoped_tls::ScopedKey<T>::set::h36b5332d57bb33ae
          19: 0x3c87 - <unknown>!tokio::runtime::scheduler::current_thread::CoreGuard::enter::h0a217f814e087e59
          20: 0x33cb - <unknown>!tokio::runtime::scheduler::current_thread::CoreGuard::block_on::h695ff7e124101719
          21: 0x2fc5 - <unknown>!tokio::runtime::scheduler::current_thread::CurrentThread::block_on::h69794225ca614655
          22: 0x4632 - <unknown>!tokio::runtime::Runtime::block_on::h30382c530f839e14
          23:  0xcf4 - <unknown>!tokio_repro::main::h0ca344617d55d8f0
          24: 0x2a61 - <unknown>!core::ops::function::FnOnce::call_once::hb7a3388f385f1b9b
          25: 0x28cf - <unknown>!std::sys_common::backtrace::__rust_begin_short_backtrace::h0417a9e1beb0cde3
          26: 0x1241 - <unknown>!std::rt::lang_start::{{closure}}::he4aec70a9b1eb3c6
          27: 0x46b46 - <unknown>!std::rt::lang_start_internal::hb6c9ed857072d0ac
          28: 0x11de - <unknown>!std::rt::lang_start::ha7dc09566047fa58
          29:  0xdf5 - <unknown>!__original_main
          30:  0xb95 - <unknown>!_start
          31: 0x544f5 - <unknown>!_start.command_export
       note: using the `WASMTIME_BACKTRACE_DETAILS=1` environment variable to may show more debugging information

[1]    2771389 exit 134   wasmtime target/wasm32-wasi/debug/tokio-repro.wasm
```
