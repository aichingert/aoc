#!/bin/sh 

exe="$(basename $1 .rs)"
rustc $1 && ./$exe
rm $exe
