#!/usr/bin/env bash

nslookup google.com

exec > >(echo "hello google.com") 2>&1

exit 1
