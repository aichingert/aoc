#!/bin/sh
# (c) aichingert

def=$(basename $1 .hs)

ghc $1

./$def
rm "$def.hi"
rm "$def.o"
rm "$def"

