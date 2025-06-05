import { quansync, type QuansyncFn } from 'quansync'
import { metadataAsync, metadataSync, type Metadata } from 'cacache-napi'
export type { Metadata } from 'cacache-napi'

/**
 * Defines the 'cacheMetadata' constant, which is a quansync instance for handling both synchronous and asynchronous metadata operations.
 *
 * @example
 *
 * ## Synchronous Usage
 *
 * ```ts
 * import { cacheMetadata } from "cacache-napi/metadata"
 *
 * cacheMetadata.sync(path/to/directory, "key");
 * ```
 *
 * ## Asynchronous Usage
 *
 * ```ts
 * import { cacheMetadata } from "cacache-napi/metadata"
 *
 * await cacheMetadata.async(path/to/directory, "key");
 * ```
 */
export const cacheMetadata: QuansyncFn<Metadata | null, [cache: string, key: string]> = quansync({
  sync: metadataSync,
  async: metadataAsync,
})
