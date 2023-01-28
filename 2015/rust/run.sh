if test "$OSTYPE" == "linux-gnu" ; then
	exe="$(basename $1 .rs)"
	rustc $1 && ./$exe
	rm $exe
else
	rustc $1.rs && ./$1.exe
	rm $1.exe
	rm $1.pdb
fi
