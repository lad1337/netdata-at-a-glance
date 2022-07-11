#!/bin/sh
set -e

echo "Starting test"
until $(curl --output /dev/null --silent --fail $ND_URI); do
    printf '.'
    sleep 5
done
echo "Success accessing NU_URI, starting netdata-at-a-glance"

#./usr/local/bin/netdata-at-a-glance
netdata-at-a-glance
