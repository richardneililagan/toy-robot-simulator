import { useCallback } from 'react'

import init, { Tabletop, Robot } from '@toy-robot-simulator/rules_engine'
import wasm from '@toy-robot-simulator/rules_engine/rules_engine_bg.wasm?url'

// :: ---

const TABLETOP_WIDTH = 5
const TABLETOP_HEIGHT = 5

const _compileModuleTask = init(wasm)
const _initTabletopTask = _compileModuleTask.then(
  () => new Tabletop(TABLETOP_WIDTH, TABLETOP_HEIGHT)
)

const useRobotFactory = () => {
  const robotFactory = useCallback(async () => {
    const tabletop = await _initTabletopTask
    return Robot.create(tabletop)
  }, [_initTabletopTask])

  return robotFactory
}

export default useRobotFactory
