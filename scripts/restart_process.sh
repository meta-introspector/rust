#!/bin/bash

ascinema rec ../docs/asciicast1.cast
gemini --model gemini-2.5-flash --checkpointing=true --include-directories ~/storage/github
