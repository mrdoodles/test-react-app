#!/bin/sh -l

# for error use normal exit with code
# if [ true ]; then
echo "Error"
#   exit 1
# fi

echo "::debug ::Debug Message"
echo "::warning ::Warning Message"
echo "::error ::Error Message"

echo "::add-mask::$1"

echo "Hello $1"
time=$(date)

echo "::set-output name=time::$time"

echo "::group::Some expandable group"
echo "Some stuff"
echo "Some stuff"
echo "Some stuff"
echo "::endGroup::"

#echo "::set-env name=HELLO::hello"
echo "HELLO=hello" >> $GITHUB_ENV