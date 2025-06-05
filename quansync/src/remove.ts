import { quansync, type QuansyncFn } from 'quansync'
import { removeAsync, removeSync, removeHashAsync, removeHashSync } from 'cacache-napi'
/**
 * Defines the 'remove' constant, which is a quansync instance for handling both synchronous and asynchronous removal operations.
 *
 * @example
 *
 * ## Synchronous Usage
 *
 * ```ts
 * import { remove } from "cacache-napi/remove"
 *
 * remove.sync(path/to/directory, "key");
 * ```
 *
 * ## Asynchronous Usage
 *
 * ```ts
 * import { remove } from "cacache-napi/remove"
 *
 * await remove.async(path/to/directory, "key");
 * ```
 */
export const remove: QuansyncFn<void, [cache: string, key: string]> = quansync({
  sync: removeSync,
  async: removeAsync,
})

/**
 * Defines the 'removeHash' constant, which is a quansync instance for handling both synchronous and asynchronous removal operations with integrity checks.
 *
 * @example
 *
 * ## Synchronous Usage
 *
 * ```ts
 * import { removeHash } from "cacache-napi/remove"
 *
 * removeHash.sync(path/to/directory, "sri");
 * ```
 *
 * ## Asynchronous Usage
 *
 * ```ts
 * import { removeHash } from "cacache-napi/remove"
 *
 * await removeHash.async(path/to/directory, "sri");
 * ```
 */
export const removeHash: QuansyncFn<void, [cache: string, sri: string]> = quansync({
  sync: removeHashSync,
  async: removeHashAsync,
})
