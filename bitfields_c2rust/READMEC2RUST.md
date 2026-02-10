# C2Rust-Bitfields-Derive

This crate is used to generate a proc macro in [c2rust-bitfields](https://github.com/immunant/c2rust/tree/master/c2rust-bitfields) and should not be a direct dependency. `c2rust-bitfields` re-exports the proc macro as well as other types and should be used instead.
Патчами служит потдержка 1) big endian через https://github.com/Utishnik/lebe_simd 
2) возможность в feature включить проверку пересечений range
