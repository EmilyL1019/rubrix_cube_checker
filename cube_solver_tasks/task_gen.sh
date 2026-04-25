#!/bin/bash

BASE="/Users/emilylight/Desktop/CSC592-EvalAI/rubrix_cube_checker/tests"
PROJECT="/Users/emilylight/Desktop/CSC592-EvalAI/rubrix_cube_checker"
OUT="$PROJECT/cube_solver_tasks/values.tsv"

CUBES=(
"rubrix1.txt"
"rubrix2.txt"
"rubrix3.txt"
)

MOVES=(U U1 U2 D D1 D2 L L1 L2 R R1 R2 F F1 F2 B B1 B2)

# -----------------------------------
# output header
# -----------------------------------
echo -e "starting_cube\tmoves\tresult_cube" > "$OUT"

# -----------------------------------
# generate valid move sequence
# rules:
# - no same face twice in a row
# - no opposite face consecutively
# -----------------------------------
gen_moves() {
    local len=$1
    local seq=""
    local prev=""
    local m
    local f

    for ((i=0; i<len; i++)); do
        while true; do
            m=${MOVES[$RANDOM % ${#MOVES[@]}]}
            f="${m:0:1}"

            # same face twice in row
            [[ "$f" == "$prev" ]] && continue

            # opposite faces consecutively
            case "$prev$f" in
                UD|DU|LR|RL|FB|BF) continue ;;
            esac

            seq="$seq $m"
            prev="$f"
            break
        done
    done

    echo "$seq" | sed 's/^ *//'
}

# -----------------------------------
# apply moves using Rust program
# -----------------------------------
apply() {
    local cube=$1
    local moves=$2
    local file="$BASE/move.txt"

    echo "$moves" > "$file"

    cd "$PROJECT" && cargo run --quiet -- --apply "$cube" "$file"
}

# -----------------------------------
# flatten multiline output to one line
# -----------------------------------
flatten() {
    tr '\n' ' ' | sed 's/[[:space:]]*$//'
}

# -----------------------------------
# main generation loop
# 9 one-move
# 11 two-move
# 11 three-move
# for each cube
# -----------------------------------
for CUBE in "${CUBES[@]}"; do

    CUBE_FILE="$BASE/$CUBE"
    echo "Processing $CUBE_FILE"

    start_cube=$(tr '\n' ' ' < "$CUBE_FILE" | sed 's/[[:space:]]*$//')

    count1=0
    count2=0
    count3=0

    # -------------------------
    # 1 move
    # -------------------------
    while [[ $count1 -lt 9 ]]; do
        m=${MOVES[$RANDOM % ${#MOVES[@]}]}

        result=$(apply "$CUBE_FILE" "$m" | flatten)

        echo -e "$start_cube\t$m\t$result" >> "$OUT"

        ((count1++))
    done

    # -------------------------
    # 2 moves
    # -------------------------
    while [[ $count2 -lt 11 ]]; do
        moves=$(gen_moves 2)

        result=$(apply "$CUBE_FILE" "$moves" | flatten)

        echo -e "$start_cube\t$moves\t$result" >> "$OUT"

        ((count2++))
    done

    # -------------------------
    # 3 moves
    # -------------------------
    while [[ $count3 -lt 11 ]]; do
        moves=$(gen_moves 3)

        result=$(apply "$CUBE_FILE" "$moves" | flatten)

        echo -e "$start_cube\t$moves\t$result" >> "$OUT"

        ((count3++))
    done

done

rm -f "$BASE/move.txt"

echo "Created dataset: $OUT"