id=0
BASE="/Users/emilylight/Desktop/CSC592-EvalAI/rubrix_cube_checker/tests"
OUT="/Users/emilylight/Desktop/CSC592-EvalAI/multi_transformation_rubrix_template.md"

# optional reset file
> "$OUT"

# -------------------
# rubrix1
# -------------------
ls $BASE/rubrix1_transformations/multi_moves/*.txt | gshuf -n 15 | while read file; do
  move=$(basename "$file" _correct.txt | sed 's/^rubrix1_//' | tr '[:lower:]' '[:upper:]')

  output=$(tr '\n' ' ' < "$file" | sed 's/[[:space:]]*$//')

  printf "| %s | U:4 3;4 5, L:2 2;4 2, F:6 1;6 1, R:3 1;6 6, B:2 3;4 5, D:1 5;3 5 | %s | %s |\n" \
  "$id" "$move" "$output" >> "$OUT"

  ((id++))
done

# -------------------
# rubrix2
# -------------------
ls $BASE/rubrix2_transformations/multi_moves/*.txt | gshuf -n 15 | while read file; do
  move=$(basename "$file" _correct.txt | sed 's/^rubrix2_//' | tr '[:lower:]' '[:upper:]')

  output=$(tr '\n' ' ' < "$file" | sed 's/[[:space:]]*$//')

  printf "| %s | U:4 3;4 5, L:2 2;4 2, F:6 1;6 1, R:3 1;6 6, B:2 3;4 5, D:1 5;3 5 | %s | %s |\n" \
  "$id" "$move" "$output" >> "$OUT"

  ((id++))
done