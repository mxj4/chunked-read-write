use wasm_bindgen_test::wasm_bindgen_test;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use chunked_read_write::io::http::HttpClient;
use futures::io::AsyncReadExt;

#[wasm_bindgen_test]
async fn test_get_file_size_success() {
    let url = "https://moonlit-tapioca-d58c4b.netlify.app/ne_50m_land.parquet";
    let size = HttpClient::get_file_size(url)
        .await
        .expect("Should get file size");
    assert!(size > 0, "File size should be positive, got {}", size);
}

#[wasm_bindgen_test]
async fn test_fetch_range_success() {
    let url = "https://moonlit-tapioca-d58c4b.netlify.app/ne_50m_land.parquet";
    let start = 434426;
    let end = 434429;
    let mut reader = HttpClient::fetch_range(url, start, end)
        .await
        .expect("HTTP fetch should succeed");
    let mut buf = vec![0u8; 4];
    reader
        .read_exact(&mut buf)
        .await
        .expect("Should read 4 bytes");
    assert_eq!(&buf[..], b"PAR1");
}

#[wasm_bindgen_test]
async fn test_fetch_range_not_found() {
    let url = "https://moonlit-tapioca-d58c4b.netlify.app/nfile_not_exist";
    // range out of bound
    let start = 0;
    let end = 200;
    let result = HttpClient::fetch_range(url, start, end).await;
    assert!(result.is_err());
}

// CDNs are buggy and do not always return 416 for out-of-bounds range requests
// #[wasm_bindgen_test]
// async fn test_fetch_range_out_of_bounds() {
//     let url = "https://moonlit-tapioca-d58c4b.netlify.app/ne_50m_land.parquet";
//     // range out of bounds
//     let start = 434430;
//     let end = 434431;
//     let result = HttpClient::fetch_range(url, start, end).await;
//     assert!(result.is_err());
// }
