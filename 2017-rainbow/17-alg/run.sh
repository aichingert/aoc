set -e

day=17

marst $day.alg -o $day.c
gcc $day.c -lalgol -lm -o $day

rm $day.c

./$day

rm $day
