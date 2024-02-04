#!/bin/sh

content=$(cat 11-01.jq)

cat ../input/11 | jq -R "$content"
