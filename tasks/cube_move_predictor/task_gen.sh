#!/bin/bash

BASE="/Users/emilylight/Desktop/CSC592-EvalAI/rubrix_cube_checker/tests"
PROJECT="/Users/emilylight/Desktop/CSC592-EvalAI/rubrix_cube_checker"
OUT="$PROJECT/tasks/cube_move_predictor/values.csv"
TMP_MOVES="$PROJECT/tmp_moves.txt"

CUBES=(
"rubrix1.txt"
"rubrix2.txt"
"rubrix3.txt"
)

MOVES=(U U1 U2 D D1 D2 L L1 L2 R R1 R2 F F1 F2 B B1 B2)

# reset file
echo -e "starting_cube,final_cube,moves" > "$OUT"

count=0

while [[ $count -lt 30 ]]; do

    # pick random cube
    CUBE_NAME=${CUBES[$RANDOM % ${#CUBES[@]}]}
    CUBE_FILE="$BASE/$CUBE_NAME"

    # pick random move length (1–3)
    LEN=$((1 + RANDOM % 3))

    movestr=""
    prev_face=""

    for ((i=0; i<$LEN; i++)); do
        while true; do
            m=${MOVES[$RANDOM % ${#MOVES[@]}]}
            face="${m:0:1}"

            # ensure no repeated face consecutively
            if [[ "$face" != "$prev_face" ]]; then
                movestr="$movestr $m"
                prev_face="$face"
                break
            fi
        done
    done

    movestr=$(echo "$movestr" | sed 's/^ *//')
    echo "$movestr" > "$TMP_MOVES"

    # run Rust transformation
    result=$(cd "$PROJECT" && cargo run --quiet -- --apply "$CUBE_FILE" "$TMP_MOVES")
    result=$(echo "$result" | tr '\n' ' ' | sed 's/[[:space:]]*$//')

    # flatten cube input
    start_cube=$(tr '\n' ' ' < "$CUBE_FILE" | sed 's/[[:space:]]*$//')

    # write CSV row (NO QUOTES)
    echo -e "$start_cube,$result,$movestr" >> "$OUT"

    ((count++))

done

rm -f "$TMP_MOVES"

echo "Created $OUT with 30 samples"