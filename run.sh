#!/bin/bash
sudo setcap cap_net_raw=eip "$1"
"$@" &
pid=$!
trap "kill -2 $pid" INT TERM
wait $pid
exit $?
