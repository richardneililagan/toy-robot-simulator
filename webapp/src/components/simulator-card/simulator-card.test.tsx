import { render } from '@/utils/testing'

import SimulatorCard from '@/components/simulator-card'

// :: ---

describe('SimulatorCard component', () => {
  it('renders without errors', () => {
    render(<SimulatorCard />)
  })

  // :: TODO really need to find a maintainable way to test WASM in this environment.
})
