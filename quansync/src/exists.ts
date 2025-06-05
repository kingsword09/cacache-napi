import { quansync, type QuansyncFn } from 'quansync'
import { existsAsync, existsSync } from 'cacache-napi'

/**
 * Defines the 'exists' constant, which is a quansync instance for handling both synchronous and asynchronous existence checks.
 *
 * @example
 *
 * ## Synchronous Usage
 *
 * ```ts
 * import { exists } from "cacache-napi/exists"
 *
 * exists.sync(path/to/directory, "sri");
 * ```
 *
 * ## Asynchronous Usage
 *
 * ```ts
 * import { exists } from "cacache-napi/exists"
 *
 * await exists.async(path/to/directory, "sri");
 * ```
 */
export const exists: QuansyncFn<boolean, [cache: string, sri: string]> = quansync({
  sync: existsSync,
  async: existsAsync,
})
