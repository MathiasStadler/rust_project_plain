#/bin/bash

trap "rm test" EXIT;

echo "Hello, reader!\n" > test;
echo "break";
cat test;
cat test