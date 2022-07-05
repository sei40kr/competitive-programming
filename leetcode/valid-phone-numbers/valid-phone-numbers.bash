#!/usr/bin/env bash

# valid-phone-numbers.bash
# author: Seong Yong-ju ( @sei40kr )

grep --color=never -Pe '^(?:\(\d{3}\)\s|\d{3}-)\d{3}-\d{4}$' <file.txt

