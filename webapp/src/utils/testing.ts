/**
 * @see https://github.com/vitest-dev/vitest/blob/main/examples/react-testing-lib/src/utils/test-utils.tsx
 */
import { render } from '@testing-library/react'

// :: ---

const customRender = (ui: React.ReactElement, options = {}) => {
  return render(ui, {
    wrapper: ({ children }) => children,
    ...options,
  })
}

export * from '@testing-library/react'
export { default as userEvent } from '@testing-library/user-event'

// :: This overrides the render export from `@testing-library/react`.
export { customRender as render }
