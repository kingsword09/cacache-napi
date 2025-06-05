use napi_derive::napi;

#[napi]
/// Clear the cache
///
/// @param cache The cache directory
/// @returns void
/// @throws Error
/// @example
/// ```no_run
/// use cacache::clear_sync;
///
/// clear_sync('/path/to/cache')
/// ```
pub fn clear_sync(cache: String) {
  _ = cacache::clear_sync(cache);
}

#[napi]
/// Clear the cache asynchronously
///
/// @param cache The cache directory
/// @returns void
/// @throws Error
/// @example
/// ```no_run
/// use cacache::clear_async;
///
/// clear_async('/path/to/cache').await
/// ```
pub async fn clear_async(cache: String) {
  _ = cacache::clear(cache);
}
