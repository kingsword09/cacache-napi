use cacache::Integrity;
use napi::Error;
use napi_derive::napi;

#[napi]
/// Copy a file from the cache to a new location
///
/// @param cache The cache directory
/// @param key The key of the file to copy
/// @param to The destination path
/// @returns void
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::copy_sync('/path/to/cache', 'key', '/path/to/destination')
/// ```
pub fn copy_sync(cache: String, key: String, to: String) -> Result<u64, Error> {
  cacache::copy_sync(cache, key, to).map_err(|err| Error::from_reason(err.to_string()))
}

#[napi]
/// Copy a file from the cache to a new location
///
/// @param cache The cache directory
/// @param sri The SRI of the file to copy
/// @param to The destination path
/// @returns void
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::copy_hash_sync('/path/to/cache', 'sha512-abcdef', '/path/to/destination')
/// ```
pub fn copy_hash_sync(cache: String, sri: String, to: String) -> Result<u64, Error> {
  let sri = sri
    .parse::<Integrity>()
    .map_err(|err| Error::from_reason(err.to_string()))?;
  cacache::copy_hash_sync(cache, &sri, to).map_err(|err| Error::from_reason(err.to_string()))
}

#[napi]
/// Copy a file from the cache to a new location
///
/// @param cache The cache directory
/// @param key The key of the file to copy
/// @param to The destination path
/// @returns void
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
/// 
/// cacache::copy_unchecked_sync('/path/to/cache', 'key', '/path/to/destination')
/// ```
pub fn copy_unchecked_sync(cache: String, key: String, to: String) -> Result<u64, Error> {
  cacache::copy_unchecked_sync(cache, key, to).map_err(|err| Error::from_reason(err.to_string()))
}

#[napi]
/// Copy a file from the cache to a new location
///
/// @param cache The cache directory
/// @param sri The SRI of the file to copy
/// @param to The destination path
/// @returns void
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::copy_hash_unchecked_sync('/path/to/cache','sha512-abcdef', '/path/to/destination')  
/// ```
pub fn copy_hash_unchecked_sync(cache: String, sri: String, to: String) -> Result<u64, Error> {
  let sri = sri
    .parse::<Integrity>()
    .map_err(|err| Error::from_reason(err.to_string()))?;
  cacache::copy_hash_unchecked_sync(cache, &sri, to)
    .map_err(|err| Error::from_reason(err.to_string()))
}

#[napi]
/// Copy a file from the cache to a new location asynchronously
///
/// @param cache The cache directory
/// @param key The key of the file to copy
/// @param to The destination path
/// @returns void
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::copy_async('/path/to/cache', 'key', '/path/to/destination').await
/// ```
pub async fn copy_async(cache: String, key: String, to: String) -> Result<u64, Error> {
  cacache::copy(cache, key, to)
    .await
    .map_err(|err| Error::from_reason(err.to_string()))
}

#[napi]
/// Copy a file from the cache to a new location asynchronously
///
/// @param cache The cache directory
/// @param sri The SRI of the file to copy
/// @param to The destination path
/// @returns void
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::copy_hash_async('/path/to/cache','sha512-abcdef', '/path/to/destination').await
/// ```
pub async fn copy_hash_async(cache: String, sri: String, to: String) -> Result<u64, Error> {
  let sri = sri
    .parse::<Integrity>()
    .map_err(|err| Error::from_reason(err.to_string()))?;
  cacache::copy_hash(cache, &sri, to)
    .await
    .map_err(|err| Error::from_reason(err.to_string()))
}

#[napi]
/// Copy a file from the cache to a new location asynchronously
///
/// @param cache The cache directory
/// @param key The key of the file to copy
/// @param to The destination path
/// @returns void
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::copy_unchecked_async('/path/to/cache', 'key', '/path/to/destination').await
/// ```
pub async fn copy_unchecked_async(cache: String, key: String, to: String) -> Result<u64, Error> {
  cacache::copy_unchecked(cache, key, to)
    .await
    .map_err(|err| Error::from_reason(err.to_string()))
}

#[napi]
/// Copy a file from the cache to a new location asynchronously
///
/// @param cache The cache directory
/// @param sri The SRI of the file to copy
/// @param to The destination path
/// @returns void
/// @throws Error
/// @example
/// ```no_run
/// use cacache;
///
/// cacache::copy_hash_unchecked_async('/path/to/cache','sha512-abcdef', '/path/to/destination').await
/// ```
pub async fn copy_hash_unchecked_async(
  cache: String,
  sri: String,
  to: String,
) -> Result<u64, Error> {
  let sri = sri
    .parse::<Integrity>()
    .map_err(|err| Error::from_reason(err.to_string()))?;
  cacache::copy_hash_unchecked(cache, &sri, to)
    .await
    .map_err(|err| Error::from_reason(err.to_string()))
}
