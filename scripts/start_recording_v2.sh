#!/bin/bash

TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
asciinema rec ../docs/asciicast_$TIMESTAMP.cast
