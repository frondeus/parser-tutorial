#!/bin/bash

echo ""
echo ""

for file in $(find . -type f -name "*.new"); do
  ACTUAL="$file"
  EXPECTED="${file%%.new}"

  echo "Accepting: $ACTUAL";
  echo "-----"

  diff -y -N "$EXPECTED" "$ACTUAL" | colordiff

  echo ""
  echo ""
  echo "-----"
  read -p "[A]ccept, [R]reject or [S]kip" -n 1 -r
  echo

  if [[ $REPLY =~ ^[Aa]$ ]]
  then
    mv -- "$ACTUAL" "$EXPECTED"
  elif [[ $REPLY =~ ^[Rr]$ ]]
  then
    rm -- "$ACTUAL"
  elif [[ $REPLY =~ ^[Ss]$ ]]
  then
    echo "Skipping"
  fi
done
echo "All processed"
