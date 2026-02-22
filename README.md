# Copilot Streaming CLI (Rust)

This small CLI posts JSON to a streaming endpoint and prints chunks as they arrive. It's intended as a minimal starter for connecting to streaming APIs (e.g., GitHub Copilot Chat or other streaming endpoints).

Usage:

1. Build:

```bash
cargo build --release
```

2. Run (provide endpoint and API key via env or arg):

```bash
# using env var
export COPILOT_API_KEY="YOUR_KEY"
./target/release/copilot_stream_cli --endpoint https://your.streaming.endpoint/path

# or pass key directly
./target/release/copilot_stream_cli --endpoint https://your.streaming.endpoint/path --key YOUR_KEY
```

You can also pass a JSON body with `--body '{"input":"hello","stream":true}'`.

Notes:
- This CLI prints raw chunks from the HTTP response. Many streaming APIs use newline-delimited JSON or SSE; you may need to parse per-API formatting.
- Adjust the default request body to match the API you are calling.
