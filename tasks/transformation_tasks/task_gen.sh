#!/bin/bash

BASE="/Users/emilylight/Desktop/CSC592-EvalAI/rubrix_cube_checker/tests"
PROJECT="/Users/emilylight/Desktop/CSC592-EvalAI/rubrix_cube_checker"
OUT="$PROJECT/tasks/transformation_tasks/values.csv"
TMP_MOVES="$PROJECT/temp_moves.txt"

# cube files
CUBES=(
"rubrix1.txt"
"rubrix2.txt"
"rubrix3.txt"
)

# allowed moves
MOVES=(U U1 U2 D D1 D2 L L1 L2 R R1 R2 F F1 F2 B B1 B2)

# reset output file (TSV)
echo -e "starting_cube,moves,result_cube" > "$OUT"

# ----------------------------
# helpers
# ----------------------------

is_opp() {
    [[ ("$1$2" == "UD" || "$1$2" == "DU" ||
        "$1$2" == "LR" || "$1$2" == "RL" ||
        "$1$2" == "FB" || "$1$2" == "BF") ]]
}

is_valid_len2() {
    local m1 m2
    m1=$(echo $1 | awk '{print $1}')
    m2=$(echo $1 | awk '{print $2}')

    f1="${m1:0:1}"
    f2="${m2:0:1}"

    # no same face
    [[ "$f1" == "$f2" ]] && return 1

    # no opposite faces
    case "$f1$f2" in
        UD|DU|LR|RL|FB|BF)
            return 1
            ;;
    esac

    return 0
}

is_valid_len3() {
    local m1 m2 m3
    m1=$(echo $1 | awk '{print $1}')
    m2=$(echo $1 | awk '{print $2}')
    m3=$(echo $1 | awk '{print $3}')

    f1="${m1:0:1}"
    f2="${m2:0:1}"
    f3="${m3:0:1}"

    # no same face consecutively
    [[ "$f1" == "$f2" ]] && return 1
    [[ "$f2" == "$f3" ]] && return 1

    # no opposite consecutively
    is_opp "$f1" "$f2" && return 1
    is_opp "$f2" "$f3" && return 1

    # special rule: if first two are opposite, third must be new face
    if is_opp "$f1" "$f2"; then
        [[ "$f3" == "$f1" || "$f3" == "$f2" ]] && return 1
    fi

    return 0
}

# ----------------------------
# dataset generation
# ----------------------------

for CUBE_FILE in "${CUBES[@]}"; do
    CUBE_FILE="$BASE/$CUBE_FILE"
    echo "$CUBE_FILE"

    cube_contents=$(tr '\n' ' ' < "$CUBE_FILE" | sed 's/[[:space:]]*$//')

    # ------------------------
    # length 1 (all 18)
    # ------------------------
    for movestr in "${MOVES[@]}"; do

        echo "$movestr" > "$TMP_MOVES"

        result=$(cd "$PROJECT" && cargo run --quiet -- --apply "$CUBE_FILE" "$TMP_MOVES")
        result=$(echo "$result" | tr '\n' ' ' | sed 's/[[:space:]]*$//')

        echo -e "$cube_contents,$movestr,$result" >> "$OUT"
    done

    # ------------------------
    # length 2 (random 18 valid)
    # ------------------------
    while IFS= read -r movestr; do
        if is_valid_len2 "$movestr"; then

            echo "$movestr" > "$TMP_MOVES"

            result=$(cd "$PROJECT" && cargo run --quiet -- --apply "$CUBE_FILE" "$TMP_MOVES")
            result=$(echo "$result" | tr '\n' ' ' | sed 's/[[:space:]]*$//')

            echo -e "$cube_contents,$movestr,$result" >> "$OUT"
        fi

    done < <(
        for m1 in "${MOVES[@]}"; do
            for m2 in "${MOVES[@]}"; do
                echo "$m1 $m2"
            done
        done | gshuf -n 18
    )

    # ------------------------
    # length 3 (random 18 valid)
    # ------------------------
    while IFS= read -r movestr; do
        if is_valid_len3 "$movestr"; then

            echo "$movestr" > "$TMP_MOVES"

            result=$(cd "$PROJECT" && cargo run --quiet -- --apply "$CUBE_FILE" "$TMP_MOVES")
            result=$(echo "$result" | tr '\n' ' ' | sed 's/[[:space:]]*$//')

            echo -e "$cube_contents,$movestr,$result" >> "$OUT"
        fi

    done < <(
        for m1 in "${MOVES[@]}"; do
            for m2 in "${MOVES[@]}"; do
                for m3 in "${MOVES[@]}"; do
                    echo "$m1 $m2 $m3"
                done
            done
        done | gshuf -n 18
    )

done

rm -f "$TMP_MOVES"

echo "Created $OUT"