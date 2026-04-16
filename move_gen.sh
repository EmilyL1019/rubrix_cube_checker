moves=(b b1 b2 l l1 l2 f f1 f2 u u1 u2 d d1 d2 r r1 r2)

mkdir -p tests/moves/multi_moves

for m1 in "${moves[@]}"; do
  for m2 in "${moves[@]}"; do
    echo "$m1\n$m2" > "tests/moves/multi_moves/${m1}${m2}.txt"
  done
done