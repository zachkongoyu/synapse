# memory_space_edits

Manage contents stored in 'memory_space' for persistence across conversations.

## Operations

| Command | When to Use | Required Params |
|---------|-------------|-----------------|
| `add` | User says "remember that...", "note that...", "don't forget" | `operate`, `content` |
| `remove` | User says "forget...", "delete...", "remove..." | `operate`, `id` |
| `replace` | User corrects previously stored information | `operate`, `id`, `content` |

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `operate` | enum | add, remove, or replace |
| `id` | string | Target memory ID (required for remove/replace) |
| `content` | string | Memory content (required for add/replace) |

## Content Guidelines
- Must start with "User" / "用户" or user's name
- Use same language as current conversation
- Must be a complete declarative statement

## Restrictions (Never store without explicit request)
- Race, ethnicity, religion
- Criminal related information
- Precise location data (addresses, coordinates)
- Political affiliations or opinions
- Health/medical information
- Information about minors (<18 years old)

## Examples

### Add
```json
{
  "operate": "add",
  "content": "User's name is Sam and works as a designer"
}
```

### Remove
```json
{
  "operate": "remove",
  "id": "1"
}
```

### Replace
```json
{
  "operate": "replace",
  "id": "1",
  "content": "User works as a developer at Moonshot"
}
```

### Important Notes
- NEVER expose actual memory_id to user
- Must confirm before removing all memories
- Memory is user-controlled and NOT used for model training
- Can be disabled in Settings → Personalization → Memory space