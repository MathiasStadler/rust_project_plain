# FROM HERE
# https://citizen428.net/blog/bash-error-handling-with-trap/


#/bin/bash
set -Eeuo pipefail

trap "{ echo 'Bye!' ; exit 0; }" SIGINT
while true ; do sleep 1 ; done