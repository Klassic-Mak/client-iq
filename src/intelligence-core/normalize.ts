//this ensures that the data is normally clean and well structured

export function normalizeDevice(data: any) {
  return {
    visitorId: String(data.visitorId),

    browser: data.browser ?? {},

    screen: data.screen ?? {},

    page: data.page ?? {},

    timestamp: Number(data.timestamp),
  }
}
