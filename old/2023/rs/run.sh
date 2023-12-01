exe="$(basename $1 .rs)"
rustc -O $1 && ./$exe
rm $exe
