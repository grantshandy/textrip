/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} value
* @returns {Uint8Array}
*/
export function run(value: Uint8Array): Uint8Array;
/**
* @param {Uint8Array} value
* @returns {string}
*/
export function image_width(value: Uint8Array): string;
/**
* @param {Uint8Array} value
* @returns {string}
*/
export function image_height(value: Uint8Array): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly run: (a: number, b: number) => number;
  readonly image_width: (a: number, b: number, c: number) => void;
  readonly image_height: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
