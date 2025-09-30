# Serena MCP Integration

## Overview
This project uses Serena MCP (Model Context Protocol) server for intelligent code analysis and editing with LSP-like capabilities. Serena provides semantic code understanding beyond basic text manipulation.

## Available Serena Tools

### File Operations
- `mcp__serena__list_dir` - List directories (with recursion and gitignore support)
- `mcp__serena__find_file` - Find files by pattern/mask

### Code Search & Analysis
- `mcp__serena__search_for_pattern` - Flexible regex pattern search with context
- `mcp__serena__get_symbols_overview` - Get top-level symbols in a file (classes, functions, structs)
- `mcp__serena__find_symbol` - Find symbols by name path (supports depth, filtering by kind)
- `mcp__serena__find_referencing_symbols` - Find all references to a symbol

### Symbol-Based Code Editing
- `mcp__serena__replace_symbol_body` - Replace entire symbol body (function, struct, etc.)
- `mcp__serena__insert_after_symbol` - Insert code after a symbol
- `mcp__serena__insert_before_symbol` - Insert code before a symbol (e.g., imports)

### Memory System
- `mcp__serena__write_memory` - Save project knowledge for future sessions
- `mcp__serena__read_memory` - Read saved project knowledge
- `mcp__serena__list_memories` - List all available memory files
- `mcp__serena__delete_memory` - Remove outdated memory files

### Project Onboarding
- `mcp__serena__check_onboarding_performed` - Check if project has been analyzed
- `mcp__serena__onboarding` - Initial project analysis and memory creation

### Reflection Tools
- `mcp__serena__think_about_collected_information` - Reflect after search operations
- `mcp__serena__think_about_task_adherence` - Verify task alignment before edits
- `mcp__serena__think_about_whether_you_are_done` - Final completion check

## Symbol Kind Reference (LSP)
Used in `find_symbol` for filtering:
- 5 = Class
- 6 = Method  
- 12 = Function
- 13 = Variable
- 23 = Struct
- 10 = Enum
- 11 = Interface/Trait
- 26 = Type Parameter

## Best Practices

### Efficient Code Reading
1. **Start with overview**: Use `get_symbols_overview` to understand file structure
2. **Target specific symbols**: Use `find_symbol` with name paths (e.g., `TableElement/new`)
3. **Only read bodies when needed**: Set `include_body=false` until you need implementation
4. **Use depth parameter**: Get nested symbols (methods of a class) with `depth=1`

### Name Path Matching
- Simple name: `method` - matches any symbol named "method"
- Relative path: `Class/method` - matches method in Class (any nesting level)
- Absolute path: `/Class/method` - matches only top-level Class

### Symbol-Based Editing
- **Replace entire symbols**: Use `replace_symbol_body` for functions, structs, impls
- **Add new symbols**: Use `insert_after_symbol` or `insert_before_symbol`
- **Add imports**: Use `insert_before_symbol` with first top-level symbol
- **Find where to edit**: Use `find_referencing_symbols` to update callers

### Pattern Search
- Use for finding code when you don't know exact symbol names
- Supports regex with context lines before/after matches
- Can restrict to code files or search all files
- Use glob patterns to filter by file type

## SevenMark-Specific Usage

### Finding Parser Code
```
# Find all parser functions
find_symbol: name_path="*_parser", include_kinds=[12]

# Find specific element parser
find_symbol: name_path="table_parser" relative_path="src/sevenmark/parser/brace/table/"

# Get table parser module structure
get_symbols_overview: relative_path="src/sevenmark/parser/brace/table/mod.rs"
```

### Finding AST Elements
```
# Get all element types
get_symbols_overview: relative_path="src/sevenmark/ast.rs"

# Find specific element with its fields
find_symbol: name_path="TableElement" depth=1 include_body=true
```

### Editing Patterns
```
# Replace parser function
replace_symbol_body: name_path="element_parser" body="new implementation"

# Add new element to enum
find_symbol: name_path="SevenMarkElement"
insert_before_symbol: name_path="SevenMarkElement/LastVariant" body="NewVariant(NewElement),"
```

## Memory Files Created
- `project_overview.md` - Project purpose, tech stack, performance
- `codebase_structure.md` - Directory layout, module organization
- `suggested_commands.md` - All development commands
- `code_style_and_conventions.md` - Naming, patterns, parser structure
- `task_completion_checklist.md` - What to do when finishing work
- `serena_mcp_integration.md` - This file