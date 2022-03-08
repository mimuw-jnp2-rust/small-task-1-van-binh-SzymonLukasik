#!/usr/bin/env bash

pushd tests || exit 1
failures=0
all=0
for test in *.in; do
    all=$((all + 1))
    echo "Running test $test"
    testname=${test%.in}
    cargo run < $test > ${testname}.result
    diff -q ${testname}.out ${testname}.result
    if [ $? -ne 0 ]; then
        echo "Test $test failed"
        failures=$((failures + 1))
    else
        echo "Test $test passed"
    fi
    rm ${testname}.result
done
echo "Ran $all tests, $failures failed"
popd || exit 1
if [ $failures -ne 0 ]; then
    exit 1
fi
