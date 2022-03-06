import { useCallback, useState } from 'react'

import init, { Tabletop, Robot } from '@toy-robot-simulator/rules_engine'
import wasm from '@toy-robot-simulator/rules_engine/rules_engine_bg.wasm?url'

// :: ---

export type UseRobotResult = [Robot | undefined, () => Promise<void>]

const TABLETOP_WIDTH = 5
const TABLETOP_HEIGHT = 5

const _compileModuleTask = init(wasm)
const _initTabletopTask = _compileModuleTask.then(
  () => new Tabletop(TABLETOP_WIDTH, TABLETOP_HEIGHT)
)

const useRobot = () => {
  const [robot, setRobot] = useState<Robot>()

  const createRobot = useCallback(async () => {
    const tabletop = await _initTabletopTask

    // :: TODO check if memory is properly dealloc'ed?
    setRobot(Robot.create(tabletop))
  }, [_initTabletopTask])

  const result: UseRobotResult = [robot, createRobot]
  return result
}

export default useRobot
