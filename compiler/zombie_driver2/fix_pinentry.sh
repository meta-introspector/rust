#!/bin/bash
# Add pinentry disable to all shell scripts

for script in *.sh; do
    if [[ -f "$script" ]]; then
        # Check if already has pinentry disable
        if ! grep -q "GPG_TTY=" "$script"; then
            # Find the line after shebang
            if head -1 "$script" | grep -q "^#!/bin/bash"; then
                # Create temp file with pinentry disable added
                {
                    head -1 "$script"
                    echo ""
                    echo "# Disable GNU pinentry"
                    echo 'export GPG_TTY=""'
                    echo 'export PINENTRY_USER_DATA="USE_CURSES=0"'
                    echo ""
                    tail -n +2 "$script"
                } > "${script}.tmp"
                mv "${script}.tmp" "$script"
                echo "Updated: $script"
            fi
        fi
    fi
done
