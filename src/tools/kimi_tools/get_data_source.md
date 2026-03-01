# get_data_source

Gets a response with data preview and file from a specific data source API.

## When to Use
- After understanding the API structure via `get_datasource_desc`
- Retrieving actual financial, economic, or academic data
- Performing data analysis on structured datasets

## Important Notes
- Do not request entire time series for sparse, non-consecutive data points
- Required parameters must be provided
- For `world_bank_open_data`, batch multiple indicators in one call
- For `arxiv` and `google_scholar`, use max 8 words or 'OR' connectors, limit to 6 rounds

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `data_source_name` | enum | yahoo_finance, arxiv, world_bank_open_data, binance_crypto, google_scholar |
| `api_name` | string | Specific API to call |
| `params` | object | API parameters |

## Example
```json
{
  "data_source_name": "yahoo_finance",
  "api_name": "get_historical_stock_prices",
  "params": {
    "ticker": "AAPL",
    "period": "1y",
    "interval": "1d"
  }
}