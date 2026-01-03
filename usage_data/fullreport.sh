cat *.json | cut -d  -f7-  | sort | uniq -c | sort -rn | head
