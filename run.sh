#!/bin/bash
sudo setcap cap_net_raw=eip "$1"
"$@" &
pid=$!
trap "kill $pid" INT TERM
wait $pid
