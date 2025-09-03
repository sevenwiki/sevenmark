const { parse_sevenmark_to_json } = require('./pkg/sevenmark.js');

console.log('🦀 SevenMark WASM Test\n');

const testCases = [
    '**bold** and *italic*',
    '{{{#fold\n[[Summary]]\n[[Details content]]\n}}}',
    '{{{#list #1\n[[Item 1]]\n[[Item 2]]\n}}}',
    '# Header 1\n## Header 2',
    '{{{#table\n[[[[Cell 1]] [[Cell 2]]]]\n[[[[Cell 3]] [[Cell 4]]]]\n}}}',
    '~~strikethrough~~ __underline__ ^^superscript^^'
];

console.log('Testing SevenMark parser...\n');

testCases.forEach((input, index) => {
    console.log(`Test ${index + 1}:`);
    console.log(`Input: ${input.replace(/\n/g, '\\n')}`);
    
    const startTime = process.hrtime.bigint();
    const result = parse_sevenmark_to_json(input);
    const endTime = process.hrtime.bigint();
    
    const parsed = JSON.parse(result);
    const duration = Number(endTime - startTime) / 1000000; // Convert to milliseconds
    
    console.log(`Output: ${parsed.length} elements`);
    console.log(`Time: ${duration.toFixed(3)}ms`);
    console.log('AST:', JSON.stringify(parsed, null, 2));
    console.log('-'.repeat(60));
});

console.log('✅ All tests completed!');