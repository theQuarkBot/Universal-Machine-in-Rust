#!/usr/bin/env bash
# 
# Purpose: 
# 

## Usage: ./um_test.bash
## 
## 
## 
## 
## author: rab
## date:  Fri Apr 15 20:04:01 EDT 2022


BASEPATH="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." >/dev/null 2>&1 && pwd )"
echo "BASEPATH: ${BASEPATH}"

UM="cargo run"
TARGET="${BASEPATH}/testing"

for i in "$TARGET"/*.um; do
    name=`basename $i`
    infile=${name%.um}.0
    outfile=${name%.um}.1

    if [[ ! -e "$infile" ]]; then
    # run valrind, capture output
        out=`"$UM" "$i"`
    else
        out=`"$UM" "$i" < "$infile"`
    fi
    # if there is output, save it and diff it against the .1 file
    if [ -e "$outfile" ]; then
        echo -n "$out" > "$name".out
        diff "${name}.out" "$outfile"
        if [[ "$?" == 1 ]]; then
            echo "DIFFERENCE DETECTED from ${name}"
            # exit 1
        fi
    fi
done