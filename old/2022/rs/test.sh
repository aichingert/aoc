exe="$(basename $1 .rs)"
rustc $1 --test && ./$exe
rm $exe
