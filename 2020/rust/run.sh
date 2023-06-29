#!/bin/bash

if test "$OSTYPE" == "linux-gnu" ; then
	exe="$(basename $1 .rs)"
	rustc -O $1 && ./$exe || exit 1
	rm $exe
else
	rustc $1.rs && ./$1.exe
	rm $1.exe
	rm $1.pdb
fi
