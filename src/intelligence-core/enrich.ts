/* eslint-disable @typescript-eslint/no-explicit-any */
export function enrichFingerprint(fingerprint: any) {
  const nav = typeof navigator !== 'undefined' ? navigator : null

  const win = typeof window !== 'undefined' ? window : null

  return {
    visitorId: fingerprint.visitorId,

    fingerprint: fingerprint.components,

    browser: {
      language: nav?.language ?? 'unknown',

      timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,

      platform: nav?.platform ?? 'unknown',
    },

    screen: {
      width: win?.screen.width ?? 0,

      height: win?.screen.height ?? 0,
    },

    page: {
      url: win?.location.href ?? '',

      referrer: typeof document !== 'undefined' ? document.referrer : '',
    },

    timestamp: Date.now(),
  }
}
