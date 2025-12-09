/**
 * Parsoid Parser Benchmark Script
 * Compare with SevenMark parser performance.
 */

'use strict';

const fs = require('fs');
const parsoid = require('parsoid');

async function benchmarkParse(content, iterations) {
    const documentLen = Buffer.byteLength(content, 'utf8');

    // Warmup
    for (let i = 0; i < 3; i++) {
        try {
            await parsoid.parse({
                input: content,
                mode: 'wt2html',
                parsoidOptions: {
                    loadWMF: false,
                    useBatchAPI: false,
                },
                envOptions: {
                    domain: 'en.wikipedia.org',
                    pageName: 'Test',
                },
                body_only: true,
            });
        } catch (e) {
            // Ignore warmup errors
        }
    }

    // Benchmark
    const startTime = process.hrtime.bigint();
    let nodeCount = 0;

    for (let i = 0; i < iterations; i++) {
        try {
            const result = await parsoid.parse({
                input: content,
                mode: 'wt2html',
                parsoidOptions: {
                    loadWMF: false,
                    useBatchAPI: false,
                },
                envOptions: {
                    domain: 'en.wikipedia.org',
                    pageName: 'Test',
                },
                body_only: true,
            });
            if (result && result.html) {
                nodeCount = result.html.length;
            }
        } catch (e) {
            console.error('Parse error:', e.message);
        }
    }

    const endTime = process.hrtime.bigint();
    const totalDurationMs = Number(endTime - startTime) / 1e6;
    const avgDurationMs = totalDurationMs / iterations;
    const throughputKbS = (documentLen / 1024) / (avgDurationMs / 1000);

    return {
        documentBytes: documentLen,
        iterations,
        totalDurationS: totalDurationMs / 1000,
        avgDurationMs,
        throughputKbS,
        outputLen: nodeCount,
    };
}

async function main() {
    let content;
    try {
        content = fs.readFileSync('ToParse_mediawiki.txt', 'utf8');
    } catch (e) {
        console.error('Error: ToParse_mediawiki.txt not found');
        process.exit(1);
    }

    console.log('='.repeat(60));
    console.log('Parsoid Parser Benchmark (Node.js)');
    console.log('='.repeat(60));
    console.log(`Input: ${Buffer.byteLength(content, 'utf8')} bytes`);
    console.log();

    // Run benchmark with fewer iterations (Parsoid is slow)
    const results = await benchmarkParse(content, 10);

    console.log(`Parsed in ${results.avgDurationMs.toFixed(3)} ms (avg)`);
    console.log(`Total time for ${results.iterations} iterations: ${results.totalDurationS.toFixed(3)} s`);
    console.log();
    console.log(`Performance: ${results.throughputKbS.toFixed(2)} KB/s`);
    console.log('='.repeat(60));

    // 10x content size test
    console.log();
    console.log('Testing with 10x content size...');
    const largeContent = content.repeat(10);
    const largeResults = await benchmarkParse(largeContent, 3);

    console.log(`Input: ${largeResults.documentBytes} bytes`);
    console.log(`Parsed in ${largeResults.avgDurationMs.toFixed(3)} ms (avg)`);
    console.log(`Performance: ${largeResults.throughputKbS.toFixed(2)} KB/s`);
}

main().catch(console.error);