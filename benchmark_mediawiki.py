#!/usr/bin/env python3
"""
MediaWiki Parser Benchmark Script
Compare with SevenMark parser performance.

Usage:
    pip install mwparserfromhell
    python benchmark_mediawiki.py
"""

import time
import sys

try:
    import mwparserfromhell
except ImportError:
    print("Error: mwparserfromhell not installed")
    print("Run: pip install mwparserfromhell")
    sys.exit(1)


def benchmark_parse(content: str, iterations: int = 100) -> dict:
    """Benchmark parsing performance."""
    document_len = len(content.encode('utf-8'))

    # Warmup
    for _ in range(3):
        mwparserfromhell.parse(content)

    # Benchmark
    start_time = time.perf_counter()
    for _ in range(iterations):
        parsed = mwparserfromhell.parse(content)
    end_time = time.perf_counter()

    total_duration = end_time - start_time
    avg_duration = total_duration / iterations
    throughput_kb_s = (document_len / 1024) / avg_duration

    return {
        "document_bytes": document_len,
        "iterations": iterations,
        "total_duration_s": total_duration,
        "avg_duration_s": avg_duration,
        "avg_duration_ms": avg_duration * 1000,
        "throughput_kb_s": throughput_kb_s,
        "nodes_count": len(parsed.nodes),
    }


def main():
    # Read MediaWiki format test file
    try:
        with open("ToParse_mediawiki.txt", "r", encoding="utf-8") as f:
            content = f.read()
    except FileNotFoundError:
        print("Error: ToParse_mediawiki.txt not found")
        sys.exit(1)

    print("=" * 60)
    print("MediaWiki Parser Benchmark (mwparserfromhell)")
    print("=" * 60)
    print(f"Input: {len(content.encode('utf-8'))} bytes")
    print()

    # Run benchmark
    results = benchmark_parse(content, iterations=100)

    print(f"Parsed {results['nodes_count']} nodes in {results['avg_duration_ms']:.3f} ms (avg)")
    print(f"Total time for {results['iterations']} iterations: {results['total_duration_s']:.3f} s")
    print()
    print(f"Performance: {results['throughput_kb_s']:.2f} KB/s")
    print("=" * 60)

    # Also test with larger content (10x)
    print()
    print("Testing with 10x content size...")
    large_content = content * 10
    large_results = benchmark_parse(large_content, iterations=10)

    print(f"Input: {large_results['document_bytes']} bytes")
    print(f"Parsed {large_results['nodes_count']} nodes in {large_results['avg_duration_ms']:.3f} ms (avg)")
    print(f"Performance: {large_results['throughput_kb_s']:.2f} KB/s")


if __name__ == "__main__":
    main()