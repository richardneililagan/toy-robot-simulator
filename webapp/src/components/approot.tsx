import { FC, useState } from 'react'

import { isWasmSupported } from '@/utils/compatibility'

import SimulatorCard from '@/components/simulator-card'

// :: ---

type AppRootProps = {
  //
}

const AppRoot: FC<AppRootProps> = () => {
  const [wasmSupported] = useState(isWasmSupported())

  return (
    <section className='flex flex-col gap-8 items-center'>
      <header>
        <h1 className='text-4xl text-white font-extralight'>Toy Robot Simulator</h1>
      </header>

      {wasmSupported ? (
        <SimulatorCard />
      ) : (
        <div className='text-white text-center'>
          <h2>
            This application requires WebAssembly support, and your browser does not seem to support
            it.
          </h2>
        </div>
      )}
    </section>
  )
}

export default AppRoot
