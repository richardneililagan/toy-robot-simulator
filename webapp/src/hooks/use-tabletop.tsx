import { useCallback } from 'react'

// :: Both of these won't be present in package.json; they'll be linked into the app
//    dynamically during the build process. Types + intellisense for these items are
//    made available as a side-effect of running `make link`.
import init, { Tabletop } from '@toy-robot-simulator/rules_engine'
// :: ?url is required so that the bundler doesn't attempt to mangle the WASM contents.
import wasm from '@toy-robot-simulator/rules_engine/rules_engine_bg.wasm?url'

// :: ---

const _compileModuleTask = init(wasm)

/**
 * @returns A factory function for instantiating Tabletops.
 */
const useTabletop = () => {
  // :: Note that this factory function does NOT take into account any previously
  //    created Tabletops. It is the responsibility of the caller to release assets
  //    once they're completely done. (Generally by calling `.free()` on the instance.)
  const tabletopFactory = useCallback(
    async (width: number, height: number) => {
      await _compileModuleTask

      return new Tabletop(width, height)
    },
    [_compileModuleTask]
  )

  return tabletopFactory
}

export default useTabletop
