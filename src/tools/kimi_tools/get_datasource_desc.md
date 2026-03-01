# get_datasource_desc

Returns detailed information and API details about a chosen data source.

## When to Use
- Before querying financial, economic, or academic data
- Understanding available APIs and their parameters
- Exploring what data is available from a specific source

## Supported Data Sources

| Source | Domain |
|--------|--------|
| `yahoo_finance` | Financial markets, stocks, forex, ETFs, crypto |
| `binance_crypto` | Cryptocurrency data |
| `world_bank_open_data` | Economic, social, environmental indicators |
| `arxiv` | Scientific papers and preprints |
| `google_scholar` | Academic literature and citations |

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `data_source_name` | enum | Name of the data source |

## Example
```json
{
  "data_source_name": "yahoo_finance"
}