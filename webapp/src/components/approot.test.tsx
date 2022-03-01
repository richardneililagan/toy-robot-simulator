import { render, screen } from '@/utils/testing'
import AppRoot from '@/components/approot'

// :: ---

describe('AppRoot component', () => {
  it('says "hello world."', () => {
    render(<AppRoot />)

    const _componentText = screen.getByText(/hello world\./)
    expect(_componentText).toBeInTheDocument()
  })
})
