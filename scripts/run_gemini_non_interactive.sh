#!/bin/bash

gemini --model gemini-2.5-flash --checkpointing=true --include-directories ~/storage/github --debug --telemetry --telemetry-log-prompts -p "Summarize the purpose and key features of the 'notel' project located in vendor/telemetry-server/notel based on its README.md file."
