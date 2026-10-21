# Build this development branch

These are source build instructions. This snapshot does not contain a Windows executable.
The X7 program files uploaded for research are not build inputs or dependencies.

## Windows 10 / 11 x64

Use a Windows 10 or 11 build machine with Rust 1.95 or newer and the Visual Studio C++ build tools with the Windows SDK. From an x64 Native Tools terminal at the repository root:

```powershell
cargo test --locked -p vectorcraft-cdr-bridge --no-default-features
cargo test --locked -p vectorcraft-engine --lib --no-default-features cdr_
cargo build --locked --release -p vectorcraft -p vectorcraft-cli
```

The executables are under `target/release`. These steps build VectorCraft with the experimental CDR importer. Actual Windows runtime testing is still required.

## Windows 7 SP1 x64

Build on Windows 10/11, not on the Windows 7 destination. Use the dedicated target and packaging recipe retained from upstream:

```powershell
powershell -File packaging/windows/windows7.ps1 -Test
```

The recipe pins its nightly compiler, rebuilds the standard library for the Windows 7 target, links the CRT statically, selects OpenGL and checks for known incompatible PE imports. It creates `dist/release/vectorcraft-windows7-x64-portable.zip`. It needs the Visual Studio x64 C++ tools, Windows SDK and network access for dependencies. The destination needs Windows 7 SP1 x64 and a driver with OpenGL 3.3 or later. This build is experimental; do not interpret a successful build as a passed Windows 7 runtime test. Use the checklist in `docs/windows7.md` on the actual computer.

## Format behaviour

File > Open and File > Place offer CDR as an experimental first-page format. Conversion warnings explain limitations. Use Save As to keep a native working copy or export SVG/PDF. There is no CDR writer in this branch. Start with copies of your designs and check the physical size and artwork before production printing.

Upstream copyright, authorship, licensing and branding remain intact. A separate product identity has not been chosen.
