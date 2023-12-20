#!/bin/sh

SRC_DIR=$1

# env
source /home/sadman/.wasienv/wasienv.sh

# build
cd $SRC_DIR
rm -rf target
ts=$(date +%s)
tsn=$(date +%N)
mvn clean package
echo Compile time: $((($(date +%s%N) - $ts$tsn) / 1000000)) ms
