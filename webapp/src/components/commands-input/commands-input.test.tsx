import { vi } from 'vitest'
import userEvent from '@testing-library/user-event'
import { render, screen } from '@/utils/testing'
import CommandsInput from '@/components/commands-input'

// :: ---

describe('CommandsInput component', () => {
  it('renders without errors', () => {
    render(<CommandsInput commands='' onCommandsChange={vi.fn()} />)
  })

  it('emits changed values to the provided handler function', () => {
    const changeHandler = vi.fn()
    render(<CommandsInput commands='' onCommandsChange={changeHandler} />)

    const inputArea = screen.getByRole('textbox')

    // :: We're only testing a single keypress because we're not actually propagating
    //    the state while this component is under test.
    userEvent.type(inputArea, 'P')
    expect(changeHandler).toHaveBeenCalledWith('P')
  })
})
