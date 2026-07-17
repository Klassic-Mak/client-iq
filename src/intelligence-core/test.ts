import { processFingerprint } from './index.ts'

const result = processFingerprint({
  visitorId: 'abc123',

  confidence: {
    score: 0.99,
  },

  components: {},
})

console.log(result)
