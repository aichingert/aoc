#/bin/sh
# (c) aichingert

for day in *.rs ; do
    echo "Running: $day"
    ./run.sh "$day"
    echo "----------------------"
done
