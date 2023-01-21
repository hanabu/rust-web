#!/bin/sh

sh ./build.sh && cd legacy && python3 -m http.server 1111
