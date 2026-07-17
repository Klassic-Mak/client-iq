import { enrichFingerprint } from './enrich'
import { calculateRisk } from './risk'
import { normalizeDevice } from './normalize'

export function processFingerprint(fingerprint: any) {
  const enriched = enrichFingerprint(fingerprint)

  const normalized = normalizeDevice(enriched)
  const risk = calculateRisk(normalized)

  return {
    ...normalized,
    risk,
  }
}
