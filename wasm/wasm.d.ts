/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} value
* @returns {Uint8Array}
*/
export function run(value: Uint8Array): Uint8Array;
/**
* @param {Uint8Array} value
* @returns {Resolution}
*/
export function get_dimensions(value: Uint8Array): Resolution;
/**
*/
export class Resolution {
  free(): void;
/**
* @returns {number}
*/
  height: number;
/**
* @returns {number}
*/
  width: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly run: (a: number, b: number) => number;
  readonly get_dimensions: (a: number, b: number) => number;
  readonly __wbg_resolution_free: (a: number) => void;
  readonly __wbg_get_resolution_width: (a: number) => number;
  readonly __wbg_set_resolution_width: (a: number, b: number) => void;
  readonly __wbg_get_resolution_height: (a: number) => number;
  readonly __wbg_set_resolution_height: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
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
