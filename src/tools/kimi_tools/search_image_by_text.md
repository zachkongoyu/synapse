# search_image_by_text

Search for images matching a text query.

## When to Use
- User explicitly asks for images
- Answering requires visual reference (what does X look like)
- Describing colors, shapes, landmarks, species, notable figures

## Query Tips
- Add context for better results: "Marie Curie portrait photo", "Möbius strip 3D illustration"

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `queries` | array of strings | Search queries (all searched in parallel) |
| `total_count` | integer | Number of images to return (1-10, default 10) |
| `need_download` | boolean | Whether to download images |
| `download_dir` | string | Directory to save images (absolute path) |

## Example
```json
{
  "queries": ["golden gate bridge sunset"],
  "total_count": 5
}