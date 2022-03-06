import { FC, useEffect, useCallback, useState } from 'react'

import useRobot from '@/hooks/use-robot'

import CommandsInput from '@/components/commands-input'
import SimulatorResults from '@/components/simulator-results'
import type { CommandResult } from '@/components/simulator-results'

// :: ---

type SimulatorCardProps = {
  //
}

/**
 * Represents the primary control on the page, with a commands input and a results
 * list view, together with a simple actions button strip.
 */
const SimulatorCard: FC<SimulatorCardProps> = () => {
  const [robot, resetRobot] = useRobot()
  const [results, setResults] = useState<CommandResult[]>([])
  const [commands, setCommands] = useState<string>()

  useEffect(() => {
    // :: We just wanna make sure that we have a ready instance of a robot prepared
    //    to do the simulations when we need it, since the actual robot instance
    //    becomes available a few ticks after calling resetRobot().
    resetRobot()
  }, [])

  const resetSimulation = useCallback(() => {
    resetRobot()
    setResults([])
  }, [resetRobot, setResults])

  const evaluateCommands = useCallback(() => {
    // :: We don't reset the results array on evaluation, because the Robot
    //    maintains it's state until explicitly reset.
    const commandStatements = commands?.split('\n')
    for (const statement of commandStatements || []) {
      // :: `Robot.evaluate_command/1` returns a string | null on success,
      //    and throws an error when the corresponding operation in WASM results in
      //    a Rust `Result::Err` result.
      //
      //    Just kinda weird that the generated return type for this method is `any`.
      try {
        const result = robot?.evaluate_command(statement)
        console.debug(`[${statement}]: OK. ${result || 'Null result.'}`)

        // :: We really only wanna add successful results with a corresponding message,
        //    although if we wanted, we could catalog all responses for each of
        //    the parsed command statements for clarity to the user.
        if (result) {
          setResults((results) => [...results, { type: 'ok', message: result }])
        }
      } catch (error) {
        console.warn(`[${statement}]: DROPPED. ${error}`)
      }
    }
  }, [robot, commands])

  return (
    <section className='p-4 bg-white rounded shadow flex flex-col gap-4'>
      <div className='h-96 flex flex-row gap-2'>
        <CommandsInput commands={commands} onCommandsChange={setCommands} />
        <SimulatorResults results={results} />
      </div>

      <footer className='flex flex-row justify-between gap-2'>
        <button
          className='px-4 py-2 border-none rounded bg-cyan-500 text-white w-1/2'
          onClick={evaluateCommands}
        >
          Evaluate
        </button>

        <button
          className='px-4 py-2 border rounded border-pink-500 text-pink-500 w-1/2'
          onClick={resetSimulation}
        >
          Reset Robot
        </button>
      </footer>
    </section>
  )
}

export default SimulatorCard
