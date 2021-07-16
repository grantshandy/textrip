/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} image_bytes
* @param {Coords} c1
* @param {Coords} c2
* @param {Coords} c3
* @param {Coords} c4
* @returns {Uint8Array}
*/
export function warp_image(image_bytes: Uint8Array, c1: Coords, c2: Coords, c3: Coords, c4: Coords): Uint8Array;
/**
* @param {Uint8Array} value
* @returns {Resolution}
*/
export function get_dimensions(value: Uint8Array): Resolution;
/**
*/
export class Coords {
  free(): void;
/**
* @param {number} x
* @param {number} y
*/
  constructor(x: number, y: number);
/**
* @returns {number}
*/
  x: number;
/**
* @returns {number}
*/
  y: number;
}
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
  readonly warp_image: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly get_dimensions: (a: number, b: number) => number;
  readonly __wbg_coords_free: (a: number) => void;
  readonly __wbg_get_coords_x: (a: number) => number;
  readonly __wbg_set_coords_x: (a: number, b: number) => void;
  readonly __wbg_get_coords_y: (a: number) => number;
  readonly __wbg_set_coords_y: (a: number, b: number) => void;
  readonly __wbg_resolution_free: (a: number) => void;
  readonly coords_new: (a: number, b: number) => number;
  readonly __wbg_set_resolution_height: (a: number, b: number) => void;
  readonly __wbg_get_resolution_width: (a: number) => number;
  readonly __wbg_set_resolution_width: (a: number, b: number) => void;
  readonly __wbg_get_resolution_height: (a: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
