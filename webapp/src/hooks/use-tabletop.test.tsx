import { vi } from 'vitest'
import { renderHook, act } from '@testing-library/react-hooks'
import useTabletop from '@/hooks/use-tabletop'

// :: ---

vi.mock('@toy-robot-simulator/rules_engine', () => {
  return async () => ({
    Tabletop: () => ({}),
  })
})

vi.mock('@toy-robot-simulator/rules_engine/rules_engine_bg.wasm?url', () => ({}))

describe('useTabletop hook', () => {
  it('creates a Tabletop factory function', () => {
    const { result } = renderHook(() => useTabletop())
    expect(result).toBeInstanceOf(Function)
  })
})
