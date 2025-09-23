# Development

1. In VSCode, Reopen in Container
2. Run integration tests
    ```sh
    wasm-pack test --headless --firefox
    ```
3. Run web app
    ```sh
    dx serve
    ```

# Chunked Read from CDN

Based on HTTP range query.
We are reading a GeoParquet file, chunk size information can be derived from the footer, which is the metadata section:

Get file size:
```sh
curl -I -L https://moonlit-tapioca-d58c4b.netlify.app/ne_50m_land.parquet | grep content-length
# should print content-length: 434430
```
Verify the magic byte:
```sh
curl -H "Range: bytes=434426-434429" -L https://moonlit-tapioca-d58c4b.netlify.app/ne_50m_land.parquet
# should print PAR1
```
Get metadata footer length:
```sh
curl -H "Range: bytes=434422-434425" -L https://moonlit-tapioca-d58c4b.netlify.app/ne_50m_land.parquet | od -t u4
# This reads the 4 bytes before the magic number and converts the little-endian integer to decimal
# should print 1146
```
Get metadata:
```sh
curl -H "Range: bytes=433276-434421" -L https://moonlit-tapioca-d58c4b.netlify.app/ne_50m_land.parquet | xxd
```
