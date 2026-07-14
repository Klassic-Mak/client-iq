export interface DeviceIdentity {
  visitorId: string
  confidence?: {
    score: number
  }
  fingerprint: any
  browser: {
    language: string
    timezone: string
    platform: string
  }
  screen: {
    width: number
    height: number
  }
  page: {
    url: string
    referrer: string
  }
  timestamp: number
}
