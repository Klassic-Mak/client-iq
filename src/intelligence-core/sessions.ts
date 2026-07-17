// src/intelligence/session.ts

const SESSION_KEY = '__device_sdk_session'

const SESSION_TIMEOUT = 30 * 60 * 1000 // 30 minutes

export function getSession() {
  const now = Date.now()

  const stored = sessionStorage.getItem(SESSION_KEY)

  if (!stored) {
    const session = {
      id: crypto.randomUUID(),
      startedAt: now,
      lastActivity: now,
      pageViews: 1,
      duration: 0,
      isNew: true,
    }

    sessionStorage.setItem(SESSION_KEY, JSON.stringify(session))

    return session
  }

  const session = JSON.parse(stored)

  // Expire after inactivity
  if (now - session.lastActivity > SESSION_TIMEOUT) {
    const newSession = {
      id: crypto.randomUUID(),
      startedAt: now,
      lastActivity: now,
      pageViews: 1,
      duration: 0,
      isNew: true,
    }

    sessionStorage.setItem(SESSION_KEY, JSON.stringify(newSession))

    return newSession
  }

  session.lastActivity = now
  session.pageViews += 1
  session.duration = now - session.startedAt
  session.isNew = false

  sessionStorage.setItem(SESSION_KEY, JSON.stringify(session))

  return session
}
