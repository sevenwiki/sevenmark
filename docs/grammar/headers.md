# Headers

<div v-pre>

SevenMark supports Markdown-style headers using the hash (`#`) symbol.

## Header Levels

SevenMark supports 6 levels of headers:

```sevenmark
# Header Level 1
## Header Level 2  
### Header Level 3
#### Header Level 4
##### Header Level 5
###### Header Level 6
```

## Collapsible Headers

Add an exclamation mark (`!`) immediately after the `#` symbols to make a header collapsible:

```sevenmark
##! Collapsible Header
This content will be hidden when the header is collapsed.
Users can click the header to expand/collapse this section.

###! Another Collapsible Section
More collapsible content here.
```

## Headers with Formatting

Headers can contain inline formatting:

```sevenmark
# **Bold** Header with *Italic* Text
## Header with ~~Strikethrough~~ and __Underline__
### Header with ^^Superscript^^ and ,,Subscript,,
```

## Headers in Complex Documents

Headers work alongside other SevenMark elements:

```sevenmark
# Main Section

This is the introduction.

##! Subsection with List

{{{#list #1
[[First item]]
[[Second item with **bold** text]]
}}}

###! Code Example

{{{#code #lang="rust"
fn main() {
    println!("Hello from collapsible section!");
}
}}}

## Another Section

{{{#table
[[[[Feature]] [[Description]]]]
[[[[Collapsible]] [[Headers can be collapsed]]]]
[[[[Formatted]] [[Headers support **formatting**]]]]
}}}
```

## Header Hierarchy

Proper header nesting creates a logical document structure:

```sevenmark
# Chapter 1: Introduction
## 1.1 Overview
### 1.1.1 Purpose
### 1.1.2 Scope
## 1.2 Getting Started
### 1.2.1 Installation
#### 1.2.1.1 System Requirements
#### 1.2.1.2 Download
### 1.2.2 Configuration

# Chapter 2: Usage
## 2.1 Basic Features
## 2.2 Advanced Features
```

## Best Practices

### Use Consistent Hierarchy

```sevenmark
# Main Topic (Level 1)
## Subtopic (Level 2)
### Details (Level 3)
#### Specific Points (Level 4)
```

### Combine with Collapsible Sections

```sevenmark
# Documentation Overview

##! Quick Start
{{{#list #1
[[Install the software]]
[[Run the setup wizard]]
[[Start using basic features]]
}}}

##! Advanced Configuration
{{{#fold
[[Summary: Advanced settings for power users]]
[[
Detailed configuration options:

{{{#code #lang="yaml"
config:
  advanced_mode: true
  custom_settings:
    feature_x: enabled
    feature_y: disabled
}}}
]]
}}}
```

</div>