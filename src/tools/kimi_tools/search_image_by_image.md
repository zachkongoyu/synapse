# search_image_by_image

Search for visually similar images using an image URL or local path.

## When to Use
- User uploads an image and asks to find similar ones
- Tracing the original source of an image
- Finding higher resolution versions

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `image_url` | string | URL of the image or local absolute file path |
| `total_count` | integer | Number of images to return (1-10, default 10) |
| `need_download` | boolean | Whether to download images |
| `download_dir` | string | Directory to save images (absolute path) |

## Example
```json
{
  "image_url": "https://example.com/image.jpg",
  "total_count": 5
}