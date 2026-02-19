import { describe, it, expect } from 'vitest';
import { AIResponseSchema, IdeaBundleSchema } from './schemas';

describe('schemas', () => {
  it('validates AIResponse', () => {
    const valid = {
      ideas: [{ title: 'A', description: 'B' }],
      stepPlan: [{ order: 1, action: 'Do X' }],
      risks: [{ description: 'R', severity: 'low' as const }],
      dependencies: [],
      effort: { time: '1 day' },
      nextActions: [{ action: 'Start', priority: 'immediate' as const }],
    };
    const parsed = AIResponseSchema.parse(valid);
    expect(parsed.ideas).toHaveLength(1);
    expect(parsed.stepPlan).toHaveLength(1);
    expect(parsed.nextActions).toHaveLength(1);
  });

  it('validates IdeaBundle', () => {
    const valid = {
      id: 'x',
      provider: 'openai' as const,
      model: 'gpt-4',
      ideas: [],
      stepPlan: [],
      risks: [],
      dependencies: [],
      effort: { time: '1h' },
      nextActions: [],
      createdAt: '123',
    };
    expect(IdeaBundleSchema.parse(valid)).toEqual(valid);
  });
});
