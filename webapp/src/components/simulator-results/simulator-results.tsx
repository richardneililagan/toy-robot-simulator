import { FC } from 'react'

// :: ---

export type CommandResult = {
  message?: string
  type: 'ok' | 'error'
}

type SimulatorResultsProps = {
  results: CommandResult[]
}

const SimulatorResults: FC<SimulatorResultsProps> = ({ results }) => {
  return (
    <div className='w-96 h-full p-4 overflow-auto font-mono'>
      {results.map((result, index) => (
        <p className='text-emerald-700' key={index}>
          {result.message}
        </p>
      ))}
    </div>
  )
}

export default SimulatorResults
