import { vi } from 'vitest'
import { render, screen } from '@/utils/testing'

import { isWasmSupported } from '@/utils/compatibility'
import AppRoot from '@/components/approot'

// :: ---

// const isWasmSupported = vi.fn()
vi.mock('@/utils/compatibility', () => ({
  isWasmSupported: vi.fn().mockReturnValue(true),
}))

describe('AppRoot component', () => {
  it('renders without errors', () => {
    render(<AppRoot />)
  })

  it('displays a warning when WebAssembly is not supported in the environment', () => {
    vi.mocked(isWasmSupported).mockReturnValueOnce(false)
    render(<AppRoot />)

    expect(
      screen.getByText(
        'This application requires WebAssembly support, and your browser does not seem to support it.'
      )
    ).toBeInTheDocument()
  })

  it('displays the simulator card components when WebAssembly is supported in the environment', () => {
    vi.mocked(isWasmSupported).mockReturnValueOnce(true) // just making sure :p
    render(<AppRoot />)

    // :: The commands input control is visible.
    expect(screen.getByRole('textbox')).toBeInTheDocument()
  })
})
