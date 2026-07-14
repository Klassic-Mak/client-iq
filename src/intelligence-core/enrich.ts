//this adds up to the fingerprint data , thus making it more detailed

import type { DeviceIdentity } from '../types'

export function enrichFingerprint(fingerprint: any): DeviceIdentity {
  return {
    visitorId: fingerprint.visitorId,

    confidence: fingerprint.confidence,

    fingerprint: fingerprint.components,
    browser: {
      language: navigator.language,
      timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
      platform: navigator.platform,
    },

    screen: {
      width: window.screen.width,
      height: window.screen.height,
    },

    page: {
      url: window.location.href,

      referrer: document.referrer,
    },
    timestamp: Date.now(),
  }
}
