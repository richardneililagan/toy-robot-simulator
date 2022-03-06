/**
 * A heuristic for testing availability of WebAssembly functionalities in
 * the current browser environment.
 *
 * @see https://stackoverflow.com/a/47880734
 */
export const isWasmSupported = (): boolean => {
  try {
    if (typeof WebAssembly === 'object' && typeof WebAssembly.instantiate === 'function') {
      const module = new WebAssembly.Module(
        Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00)
      )

      if (module instanceof WebAssembly.Module)
        return new WebAssembly.Instance(module) instanceof WebAssembly.Instance
    } //
  } catch {
    //
  }

  return false
}
