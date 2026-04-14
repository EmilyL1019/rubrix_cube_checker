# Rubix Cube Solver
## Overview

This project simulates a Rubik’s cube, applies moves, and compares cube states for correctness.

## Build
cargo build

## Run
cargo run -- <cube_file> <moves_file> <llm_output>

## Supported Moves
* F, F1, F2
* R, R1, R2
* L, L1, L2
* U, U1, U2
* D, D1, D2
* B, B1, B2
* Composite moves (example: dr2f1)