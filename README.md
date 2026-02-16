Termatama — Headless Tamagotchi P1 in Rust/WASM
===============================================

What this is
- Headless Tamagotchi P1 engine driven by the upstream C tamalib (vendored as a git submodule).
- Rust FFI wrapper plus a null HAL that feeds a terminal framebuffer (crossterm) and synthetic button input.
- State is persisted to `termatama.state` (native) or IndexedDB (WASM, future hook) so pets survive restarts.

Architecture
- Engine core: `vendor/tamalib` C code compiled via `build.rs` into `tamalib_bridge`. Rust wraps it in `sys::TamaEngine`.
- HAL: `src/sys/hal.rs` implements LCD buffer, buttons, timer hooks. LCD is stored as `[[bool; 32]; 16]` and exposed for rendering.
- ROM handling: `src/rom.rs` decodes 12-bit instructions packed in 16-bit big-endian words (see `Padded16Be12`).
- TUI: `src/tui.rs` renders the 32×16 monochrome LCD using Unicode blocks in a fixed terminal viewport.
- Input: `src/main.rs` polls crossterm events; default keybinds A/B/C = Z/X/C, remappable via `--keybind`.
- Timing: fixed-step accumulator; logic and render batches scale with `--speed` to avoid CPU pegging.
- Persistence: `src/state.rs` snapshots CPU registers/flags/LOW_FOOTPRINT memory; saves to `termatama.state` on exit, loads on start.

Building (Windows MinGW)
- Prereq: MSYS2 MinGW-w64 (posix/seh). Ensure its bin is first on PATH so gcc/cc1/as are found.
  - Example: `set PATH=C:\msys64\mingw64\bin;%PATH%`
  - Also set: `set CC=C:\msys64\mingw64\bin\gcc.exe` and `set AR=C:\msys64\mingw64\bin\ar.exe`
- Initialize submodule: `git submodule update --init --recursive`
- Build: `cargo build --release --target x86_64-pc-windows-gnu`
  - For a more standalone exe, add in `.cargo/config.toml`:
    - `rustflags = ["-C", "target-feature=+crt-static"]`
    - `linker = "C:\\msys64\\mingw64\\bin\\gcc.exe"`
    - `ar = "C:\\msys64\\mingw64\\bin\\ar.exe"`

Running
- Place your P1 ROM at `roms/tama.b` (not tracked; you must own it).
- Run: `cargo run -- roms/tama.b`
- Options:
  - `--keybind=A=q,B=w,C=e` (chars)
  - `--speed=2.0` (scales logic/render batches)
  - `--headless` (skip framebuffer; still runs logic/state)
- Exit: Esc or Ctrl+C. State auto-saves to `termatama.state` in the working dir.

WASM (planned)
- IndexedDB for persistence, async input, and text-canvas rendering would mirror the native HAL; hooks are structured but not yet wired.

Repo layout
- `src/` Rust wrapper, TUI, CLI
- `vendor/tamalib` C engine (submodule)
- `vendor/hal_types.h` shared HAL types
- `roms/` user-provided ROMs (ignored) with a README placeholder
- `termatama.state` runtime save (ignored)
