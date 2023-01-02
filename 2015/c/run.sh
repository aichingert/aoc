ex="$1"
g="gcc $1.c "
shift

while test $# != 0 ; do
	g+="../../utils/c/$1.c "
	shift
done

g+="-o $ex"
$g && ./$ex.exe
rm $ex.exe
