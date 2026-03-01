# ipython

Interactive Python execution environment (Jupyter Notebook-like).

## When to Use
- Numerical computation and math problems
- Data analysis of uploaded files (CSV/Excel/JSON)
- Chart and visualization generation
- Image processing and editing

## Features
- Standard Python code execution
- Data analysis (pandas, numpy, matplotlib)
- Image processing (Pillow, OpenCV)
- Bash commands with `!` prefix

## File System Paths

| Path | Purpose | Access |
|------|---------|--------|
| `/mnt/kimi/upload` | User uploaded files | Read-only |
| `/mnt/kimi/output` | Final deliverables (charts) | Read/Write |

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `code` | string | Python code to execute |
| `restart` | boolean | Reset all variables and imports |

## Example
```json
{
  "code": "import pandas as pd\ndf = pd.read_csv('/mnt/kimi/upload/data.csv')\nprint(df.head())"
}