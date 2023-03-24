exe="$(basename $1 .rs)"
rustc $1 && ./$exe || exit 1
rm $exe
