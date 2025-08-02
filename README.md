# How to use solana geyser plugin `Redis Edition`

## After commpiling the `geyser` plugin:
```console
git clone https://github.com/SolanaCore/solana-geyser-plugin.git
cd solana-geyser-plugin/plugin
cargo build --release
solana-test-validator --geyser-plugin-config geyser.json
```

## The configuration file `config.json` format should look like this
```json
// config.json
{
  "redis_url": "redis://default:password@redis.railway.internal:6379",
  "target_program_ids": [
    "BPFLoaderUpgradeab1e11111111111111111111111",
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    "MetaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
    "So11111111111111111111111111111111111111111"
  ]
}
```

## The geyser plugin file `geyser.json` format would look like this
```json
{ "name": "solan-geyser-plugin", "libpath": "target/release/libplugin.so"}
 ```