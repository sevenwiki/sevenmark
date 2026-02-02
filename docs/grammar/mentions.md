# Mentions

<div v-pre>

Mentions allow you to reference users and discussions/documents within your content. When rendered, mentions are resolved to display the actual user name or document title.

## User Mentions

Use `<@uuid>` syntax to mention a user:

```sevenmark
<@550e8400-e29b-41d4-a716-446655440000>
```

The UUID must be a valid UUID v4 format: `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx` (8-4-4-4-12 hexadecimal characters).

### Examples

```sevenmark
This feature was implemented by <@550e8400-e29b-41d4-a716-446655440000>.

Please contact <@6ba7b810-9dad-11d1-80b4-00c04fd430c8> for more information.

Thanks to <@f47ac10b-58cc-4372-a567-0e02b2c3d479> and <@7c9e6679-7425-40de-944b-e07fc1f90ae7> for their contributions!
```

## Discussion/Document Mentions

Use `<#uuid>` syntax to mention a discussion or document:

```sevenmark
<#550e8400-e29b-41d4-a716-446655440000>
```

### Examples

```sevenmark
See <#550e8400-e29b-41d4-a716-446655440000> for the full specification.

This is related to <#6ba7b810-9dad-11d1-80b4-00c04fd430c8>.

For more details, check <#f47ac10b-58cc-4372-a567-0e02b2c3d479> and <#7c9e6679-7425-40de-944b-e07fc1f90ae7>.
```

## Mentions in Context

Mentions can be used inline within any text:

```sevenmark
# Meeting Notes

Attendees: <@550e8400-e29b-41d4-a716-446655440000>, <@6ba7b810-9dad-11d1-80b4-00c04fd430c8>

## Agenda

1. Review <#f47ac10b-58cc-4372-a567-0e02b2c3d479>
2. Discuss implementation with <@7c9e6679-7425-40de-944b-e07fc1f90ae7>
3. Update <#a1b2c3d4-e5f6-7890-abcd-ef1234567890>
```

### In Lists

```sevenmark
{{{#list #1
[[Assigned to <@550e8400-e29b-41d4-a716-446655440000>]]
[[Reference: <#6ba7b810-9dad-11d1-80b4-00c04fd430c8>]]
[[Reviewed by <@f47ac10b-58cc-4372-a567-0e02b2c3d479>]]
}}}
```

### In Tables

```sevenmark
{{{#table
[[[[Task]] [[Assignee]] [[Related Doc]]]]
[[[[Feature A]] [[<@550e8400-e29b-41d4-a716-446655440000>]] [[<#6ba7b810-9dad-11d1-80b4-00c04fd430c8>]]]]
[[[[Bug Fix]] [[<@f47ac10b-58cc-4372-a567-0e02b2c3d479>]] [[<#7c9e6679-7425-40de-944b-e07fc1f90ae7>]]]]
}}}
```

## Rendering Behavior

When the document is processed:

- **User mentions** (`<@uuid>`) are rendered as user profile links with the user's display name
- **Discussion mentions** (`<#uuid>`) are rendered as document/discussion links with the document title

If the referenced UUID is not found in the system, the mention will be displayed as an invalid reference indicator.

## Technical Notes

- Mentions use angle bracket syntax: `<@...>` for users, `<#...>` for discussions
- UUID format must be exact: 8-4-4-4-12 hexadecimal digits with hyphens
- Mentions are resolved during the rendering phase, not parsing
- The rendered HTML includes a `data-uuid` attribute for client-side interaction
- Invalid UUIDs will cause a parse error

## Use Cases

### Collaboration

```sevenmark
<@550e8400-e29b-41d4-a716-446655440000> Please review <#6ba7b810-9dad-11d1-80b4-00c04fd430c8> when you have time.
```

### Attribution

```sevenmark
Original author: <@550e8400-e29b-41d4-a716-446655440000>
Based on: <#6ba7b810-9dad-11d1-80b4-00c04fd430c8>
```

### Cross-referencing

```sevenmark
This document extends the concepts from <#550e8400-e29b-41d4-a716-446655440000>.
For the implementation details, see <#6ba7b810-9dad-11d1-80b4-00c04fd430c8>.
```

</div>
