#!/usr/bin/env bash

# transpose-file.bash
# author: Seong Yong-ju ( @sei40kr )

awk '
{
  for (i = 1; i <= NF; ++i) {
    table[NR, i] = $i
  }
}

max_cols < NF { max_cols = NF }

END {
  for (i = 1; i <= max_cols; ++i) {
    line = table[1, i]

    for (j = 2; j <= NR; ++j) {
      line = line " " table[j, i]
    }

    print line
  }
}
' <file.txt
