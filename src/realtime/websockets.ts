import { io, Socket } from 'socket.io-client'
import { ClientIQEvent } from './events'
import { emitter } from './emitter'

export type LogLevel = 'debug' | 'info' | 'warn' | 'error'

export interface RealtimeLogEntry {
  level: LogLevel
  message: string
  data?: unknown
  timestamp: number
}

export interface SocketIOLogStreamOptions {
  url: string
  socketPath?: string
  eventName?: string
  namespace?: string
  auth?: Record<string, unknown>
  query?: Record<string, string>
  streamConsole?: boolean
}

export interface SocketIOLogStream {
  socket: Socket
  emitLog(level: LogLevel, message: string, data?: unknown): void
  disconnect(): void
}

const defaultLogEventName = 'clientiq.log'
const consoleMethods: LogLevel[] = ['debug', 'info', 'warn', 'error']

function serializeConsoleArgs(args: unknown[]): string {
  return args
    .map((arg) => {
      if (typeof arg === 'string') {
        return arg
      }

      try {
        return JSON.stringify(arg)
      } catch {
        return String(arg)
      }
    })
    .join(' ')
}

export function configureSocketIOLogStream(options: SocketIOLogStreamOptions): SocketIOLogStream {
  const { url, socketPath, eventName = defaultLogEventName, namespace = '', auth, query, streamConsole = false } = options
  const socket = io(`${url}${namespace}`, {
    path: socketPath,
    auth,
    query,
    transports: ['websocket'],
  })

  const emitLog = (level: LogLevel, message: string, data?: unknown): void => {
    const entry: RealtimeLogEntry = {
      level,
      message,
      data,
      timestamp: Date.now(),
    }

    socket.emit(eventName, entry)
    emitter.emit(ClientIQEvent.LOG, entry)
  }

  const originalConsoleMethods = new Map<LogLevel, (...data: unknown[]) => void>()

  if (streamConsole) {
    consoleMethods.forEach((level) => {
      originalConsoleMethods.set(level, console[level].bind(console))
      console[level] = (...args: unknown[]) => {
        originalConsoleMethods.get(level)?.(...args)
        emitLog(level, serializeConsoleArgs(args), args)
      }
    })
  }

  return {
    socket,
    emitLog,
    disconnect() {
      originalConsoleMethods.forEach((method, level) => {
        console[level] = method
      })
      originalConsoleMethods.clear()
      socket.disconnect()
    },
  }
}
