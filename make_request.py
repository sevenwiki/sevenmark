#!/usr/bin/env python3
import json
import os

# Read ToParse.txt
with open("ToParse.txt", "r", encoding="utf-8") as f:
    content = f.read()

# Create JSON request
request_body = {
    "content": content
}

# Print JSON (for piping to curl)
print(json.dumps(request_body, ensure_ascii=False))

# Save to load_test folder
load_test_path = os.path.join("misc", "load_test", "request.json")
with open(load_test_path, "w", encoding="utf-8") as f:
    json.dump(request_body, f, ensure_ascii=False, indent=2)

print(f"\n✓ Saved to {load_test_path}", file=__import__('sys').stderr)