import { quansync, type QuansyncFn } from 'quansync'
import {
  copyAsync,
  copyHashAsync,
  copyHashSync,
  copyHashUncheckedAsync,
  copyHashUncheckedSync,
  copySync,
  copyUncheckedAsync,
  copyUncheckedSync,
} from 'cacache-napi'

/**
 * Defines the 'copy' constant, which is a quansync instance for handling both synchronous and asynchronous copying operations.
 *
 * @example
 *
 * ## Synchronous Usage
 *
 * ```ts
 * import { copy } from "cacache-napi/copy"
 *
 * copy.sync(path/to/directory, "key", path/to/directory);
 * ```
 *
 * ## Asynchronous Usage
 *
 * ```ts
 * import { copy } from "cacache-napi/copy"
 *
 * await copy.async(path/to/directory, "key", path/to/directory);
 * ```
 */
export const copy: QuansyncFn<bigint, [cache: string, key: string, to: string]> = quansync({
  sync: copySync,
  async: copyAsync,
})

/**
 * Defines the 'copyHash' constant, which is a quansync instance for handling both synchronous and asynchronous copying operations with integrity checks.
 *
 * @example
 *
 * ## Synchronous Usage
 *
 * ```ts
 * import { copyHash } from "cacache-napi/copy"
 *
 * copyHash.sync(path/to/directory, "sri", path/to/directory);
 * ```
 *
 * ## Asynchronous Usage
 *
 * ```ts
 * import { copyHash } from "cacache-napi/copy"
 *
 * await copyHash.async(path/to/directory, "sri", path/to/directory);
 * ```
 */
export const copyHash: QuansyncFn<bigint, [cache: string, sri: string, to: string]> = quansync({
  sync: copyHashSync,
  async: copyHashAsync,
})

/**
 * Defines the 'copyHashUnchecked' constant, which is a quansync instance for handling both synchronous and asynchronous copying operations without integrity checks.
 *
 * @example
 *
 * ## Synchronous Usage
 *
 * ```ts
 * import { copyHashUnchecked } from "cacache-napi/copy"
 *
 * copyHashUnchecked.sync(path/to/directory, "sri", path/to/directory);
 * ```
 *
 * ## Asynchronous Usage
 *
 * ```ts
 * import { copyHashUnchecked } from "cacache-napi/copy"
 *
 * await copyHashUnchecked.async(path/to/directory, "sri", path/to/directory);
 * ```
 */
export const copyHashUnchecked: QuansyncFn<bigint, [cache: string, sri: string, to: string]> = quansync({
  sync: copyHashUncheckedSync,
  async: copyHashUncheckedAsync,
})

/**
 * Defines the 'copyUnchecked' constant, which is a quansync instance for handling both synchronous and asynchronous copying operations without integrity checks.
 *
 * @example
 *
 * ## Synchronous Usage
 *
 * ```ts
 * import { copyUnchecked } from "cacache-napi/copy"
 *
 * copyUnchecked.sync(path/to/directory, "key", path/to/directory);
 * ```
 *
 * ## Asynchronous Usage
 *
 * ```ts
 * import { copyUnchecked } from "cacache-napi/copy"
 *
 * await copyUnchecked.async(path/to/directory, "key", path/to/directory);
 * ```
 */
export const copyUnchecked: QuansyncFn<bigint, [cache: string, key: string, to: string]> = quansync({
  sync: copyUncheckedSync,
  async: copyUncheckedAsync,
})
