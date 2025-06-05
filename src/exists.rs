use cacache::Integrity;
use napi::Error;
use napi_derive::napi;

#[napi]
/// Check if an entry exists in the cache
///
/// @param cache The cache directory
/// @param sri The SRI of the entry
/// @returns boolean
/// @throws Error
/// @example
/// ```no_run
/// use cacache::exists_sync;
///
/// exists_sync('/path/to/cache', 'sha512-abc123')
/// ```
pub fn exists_sync(cache: String, sri: String) -> Result<bool, Error> {
  let sri = sri
    .parse::<Integrity>()
    .map_err(|err| Error::from_reason(err.to_string()))?;
  Ok(cacache::exists_sync(cache, &sri))
}

#[napi]
/// Check if an entry exists in the cache asynchronously
///
/// @param cache The cache directory
/// @param sri The SRI of the entry
/// @returns boolean
/// @throws Error
/// @example
/// ```no_run
/// use cacache::exists_async;
///
/// exists_async('/path/to/cache','sha512-abc123').await
/// ```
pub async fn exists_async(cache: String, sri: String) -> Result<bool, Error> {
  let sri = sri
    .parse::<Integrity>()
    .map_err(|err| Error::from_reason(err.to_string()))?;
  Ok(cacache::exists(cache, &sri).await)
}
