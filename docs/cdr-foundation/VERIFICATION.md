# Verification record

Environment: Linux x86_64, Rust 1.99.1. This record is for the development branch only.

| Check | Result |
| --- | --- |
| `cargo test -p vectorcraft-cdr-bridge --no-default-features` | Passed, 3 tests |
| `cargo check -p vectorcraft-engine --no-default-features` | Passed |
| `cargo check --locked -p vectorcraft-cdr-bridge --target wasm32-unknown-unknown` | Passed |
| `cargo test -p vectorcraft-engine --lib --no-default-features cdr_` | Passed, 2 tests; 2641 unrelated unit tests filtered out |
| `cargo fmt -p vectorcraft-cdr-bridge --check` | Passed |
| `git diff --check` | Passed |
| Complete upstream CI gate | Not run |
| Windows desktop executable build | Not run |
| Windows 7 SP1 / 10 / 11 runtime tests | Not run |
| Mobile runtime and touch testing | Not run |
| Real user X7 drawings | Not available for this test pass |

The CDR tests construct original synthetic documents with an independently implemented CDR writer. They verify basic parsing and conversion. They do not establish fidelity for all files from the original application. Real files, matching PDF/PNG exports and installed fonts are needed for production-fidelity checks.

The Windows 7 build remains experimental upstream. Its use of a dedicated compiler target, static CRT and API import checks reduces known startup failures, but does not prove every dependency or feature works on Windows 7.

Build note: the initial engine test command also built unrelated integration executables and exhausted the 32 GiB build disk while linking. Generated binaries were removed, and the focused `--lib` rerun passed.
