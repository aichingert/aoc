set -e

marst 16.alg -o 16.c
gcc 16.c -lalgol -lm -o 16

./16

rm 16
rm 16.c
