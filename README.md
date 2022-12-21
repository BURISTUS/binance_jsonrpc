# Binance jsonrpc.

## Project structure 

 * `src` - Project folder.
 * `scripts` - Starting scripts folder.
 * `docs` - Task description.
 * `test` - Project tests.

## Project build
To build a project, you must first run script inside `./scripts` folder.  
To do it from project root directory use `./scripts/init_db.sh`.  
After that, you can run the project using `cargo run`.

## Project testing
To run tests, just run the `cargo test` command.  

## Request examples.  

```localhost api path: Post http://127.0.0.1:8000/api```

To create a buy limit order.

```
{
	"id": "0",
	"jsonrpc": "2.0",
	"method": "create_order",
	"params": {
		"symbol": "XRPUSDT", -> pair
		"condition_price": "0.35", -> /* when the order price is lower than 0.35, the order will be posted  */
		"quantity": "40",
		"side": "BUY",
		"price": "0.341",
		"time_in_force": "GTC",
		"order_type": "LIMIT"
	}
}
```

To create a sell limit order.

```
{
	"id": "0",
	"jsonrpc": "2.0",
	"method": "create_order",
	"params": {
		"symbol": "XRPUSDT", -> pair
		"condition_price": "0.30", -> /* when the order price is greater than 0.30, the order will be posted  */
		"quantity": "40",
		"side": "SELL",
		"price": "0.341",
		"time_in_force": "GTC",
		"order_type": "LIMIT"
	}
}
```

To сreate a buy order at the market price.  

```
{
	"id": "0",
	"jsonrpc": "2.0",
	"method": "create_order",
	"params": {
		"symbol": "XRPUSDT",
		"condition_price": "0.35",
		"quantity": "40",
		"side": "BUY",
		"order_type": "MARKET"
	}
}
```

To сreate a sell order at the market price.  

```
{
	"id": "0",
	"jsonrpc": "2.0",
	"method": "create_order",
	"params": {
		"symbol": "XRPUSDT",
		"condition_price": "0.3",
		"quantity": "40",
		"side": "SELL",
		"order_type": "MARKET"
	}
}
```

To get order info

```
{
	"id": "0",
	"jsonrpc": "2.0",
	"method": "get_order",
	"params": {
		"order_id": 412800
	}
}
```

To delete active order

```
{
	"id": "0",
	"jsonrpc": "2.0",
	"method": "delete_order",
	"params": {
		"order_id": 412800
	}
}
```

## Telegram notifications  
To send messages using your bot and your channel  
add your `bot_url` and `chat_id` in `configuration.yaml`  
```
telegram: 
  bot_url: "https://api.telegram.org/bot_url"
  chat_id: "chat_id"
```

## Binance api  
To change binance api key and secret key
add your `api_key` and `secret_key` in `configuration.yaml`  
```
binance:
  url: "https://testnet.binance.vision"
  wss_url: "wss://testnet.binance.vision/stream?streams="
  api_key: "your api key"
  secret_key: "your secret key"
```