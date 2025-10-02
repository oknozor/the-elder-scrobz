#!/usr/bin/env bash

now=$(date +%s)
one_year=$((365*24*60*60))
MOCK_DATA=$1
OUTPUT=$2

while IFS= read -r line; do
  # pick a random timestamp within the last 12 months
  rand_offset=$((RANDOM * RANDOM % one_year))
  new_ts=$((now - rand_offset))

  # update listened_at field in compact JSON (JSONL safe)
  echo "$line" | jq -c --argjson ts "$new_ts" '.listened_at = $ts'
done < "$MOCK_DATA" > $OUTPUT
