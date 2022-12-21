-- Add migration script here

CREATE TYPE Side as ENUM('Buy', 'Sell');
CREATE TYPE TimeInForce as ENUM('Gtc', 'Ioc', 'Fok');
CREATE TYPE OrderType as ENUM('Limit', 'Market', 'StopLoss', 'StopLossLimit', 'TakeProfit', 'TakeProfitLimit', 'LimitMaker');
CREATE TYPE OrderStatus as ENUM('New', 'PartiallyFilled', 'Filled', 'Canceled', 'PendingCancel', 'Rejected', 'Expired');

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    symbol TEXT NOT NULL,
    condition_price NUMERIC NOT NULL,
    quantity NUMERIC NOT NULL,
    price NUMERIC,
    order_id INTEGER,
    side Side NOT NULL,
    time_in_force TimeInForce DEFAULT 'Gtc',
    order_type OrderType NOT NULL,
    order_status OrderStatus NOT NULL DEFAULT 'New',
    posted_at timestamptz NOT NULL
);