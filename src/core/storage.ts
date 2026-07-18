export class Storage {
  static get<T>(key: string): T | null {
    if (typeof sessionStorage === 'undefined') {
      return null
    }

    const value = sessionStorage.getItem(key)

    if (!value) {
      return null
    }

    try {
      return JSON.parse(value) as T
    } catch {
      return null
    }
  }

  static set(key: string, value: unknown): void {
    if (typeof sessionStorage === 'undefined') {
      return
    }

    sessionStorage.setItem(key, JSON.stringify(value))
  }

  static remove(key: string): void {
    if (typeof sessionStorage === 'undefined') {
      return
    }

    sessionStorage.removeItem(key)
  }

  static has(key: string): boolean {
    if (typeof sessionStorage === 'undefined') {
      return false
    }

    return sessionStorage.getItem(key) !== null
  }

  static clear(): void {
    if (typeof sessionStorage === 'undefined') {
      return
    }

    sessionStorage.clear()
  }
}
