#!/usr/bin/env bash

# word-frequency.bash
# author: Seong Yong-ju ( @sei40kr )

tr ' ' '\n' <words.txt | sort | uniq -c | sort -r | awk '$2 {print $2" "$1}'

