# web_open_url Tool Documentation

## Purpose
Opens a specific URL and displays its content, allowing direct access and analysis of web pages.

## Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `urls` | array of strings | **Yes** | URLs to fetch |

## When to Use

- When user provides a valid web URL and wants (or implies wanting) to:
  - Access content
  - Read content
  - Summarize content
  - Analyze content

## Usage Notes

- Accepts multiple URLs in parallel
- Returns page content for direct analysis
- Use in conjunction with `web_search` when specific sources are identified
- Useful for fact-checking specific claims from search results

## Example Usage

**User**: "What does this article say? [https://example.com/article]"

**Tool Call**:
```json
{
  "urls": ["https://example.com/article"]
}
```