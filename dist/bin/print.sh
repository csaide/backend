#!/bin/bash

# Copyright (c) 2019 Christian Saide <supernomad>
# Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

COLUMNS=$(tput cols)
LEN=$(echo "${1}" | wc -c)

printf "=%.s" $(seq 1 $(( (${COLUMNS} / 2) - ((${LEN} / 2) + 1) )))
printf " %s " "${1}"
printf "=%.s" $(seq 1 $(( (${COLUMNS} / 2) - ((${LEN} / 2) + 1) )))
printf "\n"
