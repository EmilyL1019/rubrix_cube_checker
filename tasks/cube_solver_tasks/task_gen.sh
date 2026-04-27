#!/bin/bash

BASE="/Users/emilylight/Desktop/CSC592-EvalAI/rubrix_cube_checker/tests"
PROJECT="/Users/emilylight/Desktop/CSC592-EvalAI/rubrix_cube_checker"
OUT="$PROJECT/tasks/cube_solver_tasks/values.tsv"
OUTFILE="$PROJECT/tasks/cube_solver_tasks/out.txt"

MOVES=(U U1 U2 D D1 D2 L L1 L2 R R1 R2 F F1 F2 B B1 B2)

NUM_CUBES=30
CUBE="$BASE/solved.txt"

# reset dataset
echo -e "starting_cube\tnum_moves\texample_ref" > "$OUT"

count=0

while [[ $count -lt $NUM_CUBES ]]; do

    # random length 1-3
    LEN=$((1 + RANDOM % 3))

    movestr=""
    prev_face=""

    # build move sequence
    for ((i=0; i<LEN; i++)); do
        while true; do
            m=${MOVES[$RANDOM % ${#MOVES[@]}]}
            face="${m:0:1}"

            # no same face twice in a row
            if [[ "$face" != "$prev_face" ]]; then
                movestr="$movestr $m"
                prev_face="$face"
                break
            fi
        done
    done

    # trim leading space
    movestr="$(echo "$movestr" | sed 's/^ *//')"

    echo "cargo run -- --apply $CUBE $movestr > $OUTFILE"
    # apply moves to solved cube
    cargo run -- --apply "$CUBE" "$movestr" > "$OUTFILE"
    # capture scrambled cube (this is the starting cube)
    start_cube=$(tr '\n' ' ' < "$OUTFILE" | sed 's/[[:space:]]*$//')

    ref=$(cd "$PROJECT" && cargo run --quiet -- "$OUTFILE")
    echo -e "$start_cube,$ref" >> "$OUT"
    count=$((count + 1))
done
echo "Created dataset: $OUT"