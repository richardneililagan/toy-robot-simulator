import { render, screen } from '@/utils/testing'
import SimulatorResults from '@/components/simulator-results'
import type { CommandResult } from '@/components/simulator-results'

// :: ---

describe('SimulatorResults component', () => {
  it('renders without errors', () => {
    render(<SimulatorResults results={[]} />)
  })

  it('renders all result messages provided to it', () => {
    const results: CommandResult[] = [
      {
        message: 'Hello world.',
        type: 'ok',
      },
      {
        message: 'Elden Ring.',
        type: 'ok',
      },
      {
        message: 'Invalid command.',
        type: 'error',
      },
    ]

    render(<SimulatorResults results={results} />)

    expect(screen.getByText('Hello world.')).toBeInTheDocument()
    expect(screen.getByText('Elden Ring.')).toBeInTheDocument()
    expect(screen.getByText('Invalid command.')).toBeInTheDocument()
  })
})
