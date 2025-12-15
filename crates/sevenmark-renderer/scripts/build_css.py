#!/usr/bin/env python3
"""Combines all SevenMark CSS component files into a single stylesheet."""

from pathlib import Path

CSS_DIR = Path(__file__).parent.parent / "assets" / "css"
OUTPUT_FILE = Path(__file__).parent.parent / "assets" / "sevenmark.css"

def build_css():
    css_files = sorted(CSS_DIR.glob("*.css"))

    # base.css should come first
    base = CSS_DIR / "base.css"
    if base in css_files:
        css_files.remove(base)
        css_files.insert(0, base)

    parts = []
    for filepath in css_files:
        content = filepath.read_text(encoding="utf-8").strip()
        parts.append(content)
        print(f"Added: {filepath.name}")

    combined = "\n\n".join(parts)
    OUTPUT_FILE.write_text(combined, encoding="utf-8")
    print(f"\nCombined CSS written to {OUTPUT_FILE}")

if __name__ == "__main__":
    build_css()
