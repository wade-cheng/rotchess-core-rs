# rotchess-core

Core library code for headless rotchess logic in Rust.

This module provides functionality for a `RotchessEmulator` to modify
and record state over time as it is fed information in the form of
events. External code should read the board's state and
draw it for the user. This crate is not responsible for any drawing of state.
It does, however, provide functions that help the drawing program understand
what to draw.
