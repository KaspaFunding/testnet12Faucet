# Kaspa Testnet-12 Faucet (Rust)

A simple, lightweight faucet for Kaspa testnet-12 written in Rust. It provides a small amount of KAS to any testnet-12 address, with IP-based rate limiting.

## Features

- Sends a fixed amount of KAS per claim
- IP-based rate limiting (default: 1 claim per hour)
- Simple HTTP API (`/status`, `/claim`)
- Configurable via `faucet-config.toml`
- No external database required (in-memory rate limiting)

## Quick start

1. **Install Rust** (if you haven’t already)
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone and build**
   ```sh
   cd testnet12Faucet
   cargo build --release
   ```

3. **Run once to generate config**
   ```sh
   ./target/release/faucet
   # It will create faucet-config.toml
   ```

4. **Edit `faucet-config.toml`**
   ```toml
   kaspad_url = "127.0.0.1:16210"
   port = 3010
   faucet_private_key = "YOUR_PRIVATE_KEY_HERE"
   # amount_per_claim can be specified as:
   # - sompi (u64): 100000000
   # - or KAS decimal (string/number): "1.00000000" or 1.0
   amount_per_claim = "0.01000000"
   claim_interval_seconds = 3600      # 1 hour
   ```

5. **Run**
   ```sh
   ./target/release/faucet
   ```

## API

### GET /
Simple HTML welcome page.

### GET /status
```json
{
  "active": true,
  "faucet_address": "kaspatest:...",
  "balance_kas": "123.45678000",
  "next_claim_seconds": 3600
}
```

### POST /claim
Request body:
```json
{
  "address": "kaspatest:qz4wqx8kjzcj4fj6g5kqvcs9ckf7l2z5p9c4u8x0xvqyqvlz7x"
}
```

Success response:
```json
{
  "transaction_id": "abcd1234...",
  "amount_kas": "0.01000000",
  "next_claim_seconds": 3600
}
```

Error responses return appropriate HTTP status codes (400, 429, 500).

## Notes

- This faucet targets **testnet-12** only.
- Ensure your kaspad node is synced and reachable.
- Keep the faucet wallet funded; otherwise claims will fail.
- Rate limiting is in-memory only; restarts clear the history.

## License

MIT
