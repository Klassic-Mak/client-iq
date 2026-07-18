import { ClientIQEvent } from './events'

type Listener = (payload: unknown) => void

export class EventEmitter {
  private listeners = new Map<ClientIQEvent, Set<Listener>>()

  on(event: ClientIQEvent, listener: Listener): void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set())
    }

    this.listeners.get(event)!.add(listener)
  }

  off(event: ClientIQEvent, listener: Listener): void {
    this.listeners.get(event)?.delete(listener)
  }

  emit(event: ClientIQEvent, payload: unknown): void {
    this.listeners.get(event)?.forEach((listener) => listener(payload))
  }
}

export const emitter = new EventEmitter()
