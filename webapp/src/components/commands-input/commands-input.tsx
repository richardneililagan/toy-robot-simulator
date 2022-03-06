import { FC } from 'react'

// :: ---

type CommandsInputProps = {
  commands: string | undefined
  onCommandsChange: (value: string) => void
}

const CommandsInput: FC<CommandsInputProps> = ({ commands, onCommandsChange }) => {
  return (
    <textarea
      className='w-96 h-full p-4 font-mono border border-slate-200'
      placeholder='Input commands here.'
      value={commands}
      onChange={({ target }) => onCommandsChange(target.value)}
    ></textarea>
  )
}

export default CommandsInput
