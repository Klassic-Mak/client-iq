export function calculateRisk(data: any) {
  let score = 0

  if (!data.browser.language) {
    score += 10
  }

  if (data.screen.language) {
    score += 20
  }

  return {
    score,
    level: score > 50 ? 'high' : 'low',
  }
}
