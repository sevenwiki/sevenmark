# Complex Examples

<div v-pre>

This section shows advanced SevenMark usage with nested structures and complex layouts.

## Nested Lists with Mixed Content

```sevenmark
{{{#list #1 #style="line-height: 1.6"
[[**Project Setup**]]
[[{{{#list #a
[[Install dependencies: {{{#code npm install }}}]]
[[Configure environment: {{{#code cp .env.example .env }}}]]
[[Run initial setup: {{{#code npm run setup }}}]]
}}}]]
[[**Development Process**]]
[[{{{#list #a
[[Write code with proper **error handling**]]
[[Add unit tests for ~~legacy~~ new functionality]]
[[Update documentation with examples]]
}}}]]
[[**Deployment**]]
[[{{{#list #i
[[Run tests: [now] - All tests must pass]]
[[Build project: {{{#code npm run build }}}]]
[[Deploy to staging environment]]
[[Verify deployment works correctly]]
}}}]]
}}}
```

## Advanced Table with Nested Elements

```sevenmark
{{{#table #style="border-collapse: collapse; width: 100%"
[[[[**Component**]] [[**Status**]] [[**Last Updated**]] [[**Actions**]]]]
[[[[Frontend]] [[✅ **Active**]] [[[now]]] [[{{{#list #1 [[Deploy]] [[Test]] [[Monitor]] }}}]]]]
[[[[Backend API]] [[⚠️ *Maintenance*]] [[2024-01-15]] [[{{{#code #lang="bash" systemctl restart api }}}]]]]
[[[[Database]] [[❌ ~~Offline~~]] [[2024-01-10]] [[[[#url="https://status.db.com" Check Status]]]]]]
}}}
```

## Collapsible Documentation Section

```sevenmark
## API Reference-

{{{#fold
[[📚 **Quick Reference** - Click to expand full documentation]]
[[
### Authentication

{{{#code #lang="bash"
curl -H "Authorization: Bearer TOKEN" \
     -H "Content-Type: application/json" \
     https://api.example.com/data
}}}

### Response Format

{{{#table
[[[[Field]] [[Type]] [[Description]]]]
[[[[id]] [[number]] [[Unique identifier]]]]
[[[[name]] [[string]] [[Resource name]]]]
[[[[created_at]] [[string]] [[ISO 8601 timestamp]]]]
}}}

### Error Handling

Common error codes:
{{{#list #1
[[**400 Bad Request** - Invalid parameters]]
[[**401 Unauthorized** - Missing or invalid token]]
[[**404 Not Found** - Resource doesn't exist]]
[[**500 Server Error** - Internal server issue]]
}}}
]]
}}}
```

## Scientific Document Example

```sevenmark
# Research Findings

## Abstract

This study examines the relationship between **temperature** and *reaction rates* 
in chemical processes.

## Methodology

{{{#list #1
[[Sample preparation at room temperature (20°C ± 2°C)]]
[[Measurement using calibrated equipment]]
[[Data collection over [age(2023-01-01)] days]]
[[Statistical analysis using specialized software]]
}}}

## Results

### Temperature vs Reaction Rate

{{{#table
[[[[Temperature (°C)]] [[Rate (mol/s)]] [[Standard Deviation]]]]
[[[[20]] [[1.2 × 10^^-3^^]] [[±0.05]]]]
[[[[40]] [[2.8 × 10^^-3^^]] [[±0.08]]]]
[[[[60]] [[5.1 × 10^^-3^^]] [[±0.12]]]]
}}}

### Mathematical Model

The relationship follows the Arrhenius equation:

{{{#tex #block
k = A \cdot e^{-\frac{E_a}{RT}}
}}}

Where:
- k = rate constant
- A = pre-exponential factor  
- E,,a,, = activation energy
- R = gas constant
- T = absolute temperature

## Conclusion

{{{#quote #style="font-style: italic; border-left: 3px solid #007acc; padding-left: 15px"
"The experimental data confirms the exponential relationship between temperature 
and reaction rate, with correlation coefficient r² = 0.998."
}}}

/* 
Multi-line comment for peer review:
- Verify statistical significance
- Check experimental controls
- Review literature citations
*/
```

## Wiki Page with All Features

```sevenmark
{{{#category Programming Languages}}}
{{{#category Documentation}}}

# Advanced SevenMark Tutorial

{{{#include #page="CommonIntro" Basic introduction content}}}

## Table of Contents-

{{{#list #I
[[**Text Formatting** - Styles and emphasis]]
[[**Structure Elements** - Lists, tables, folds]]
[[**Media Integration** - Images and links]]
[[**Advanced Features** - Math, code, comments]]
}}}

## Interactive Examples-

{{{#fold #style="background: #f8f9fa; border: 1px solid #e9ecef; border-radius: 5px"
[[🎯 **Try These Examples** - Expand to see interactive content]]
[[
### Styled Content Example

{{{ #style="color: #d63384; font-weight: bold; background: #fff3cd; padding: 10px; border-radius: 3px"
This is custom-styled content with multiple parameters applied!
}}}

### Complex Math Formula

{{{#tex #block
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
}}}

### File Reference

[[#file="example-output.json" View Sample Output]]
]]
}}}

---

*Last updated: [now] | Age of this tutorial: [age(2024-01-01)] days*
```

</div>