#!/bin/bash

BASE="/Users/emilylight/Desktop/CSC592-EvalAI/rubrix_cube_checker/tests"

CUBES=("rubrix1" "rubrix2")
MOVE_DIRS=("single_moves" "multi_moves")

for cube in "${CUBES[@]}"; do
  cube_file="$BASE/${cube}.txt"
  out_dir="$BASE/${cube}_transformations"

  mkdir -p "$out_dir"

  for dir in "${MOVE_DIRS[@]}"; do
    for move_file in "$BASE/moves/$dir"/*.txt; do
      move_name=$(basename "$move_file" .txt)

      output_file="$out_dir/${cube}_${move_name}_correct.txt"

      cargo run -- "$cube_file" "$move_file" > "$output_file"
    done
  done
done