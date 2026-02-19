/**
 * Zod schemas for validation
 */
import { z } from 'zod';

export const IdeaSchema = z.object({
  title: z.string(),
  description: z.string(),
  rationale: z.string().optional(),
});

export const StepSchema = z.object({
  order: z.number(),
  action: z.string(),
  details: z.string().optional(),
});

export const RiskSchema = z.object({
  description: z.string(),
  severity: z.enum(['low', 'medium', 'high']),
  mitigation: z.string().optional(),
});

export const EffortEstimateSchema = z.object({
  time: z.string(),
  cost: z.string().optional(),
  complexity: z.enum(['low', 'medium', 'high']).optional(),
});

export const NextActionSchema = z.object({
  action: z.string(),
  priority: z.enum(['immediate', 'short', 'medium', 'long']),
});

export const IdeaBundleSchema = z.object({
  id: z.string(),
  provider: z.enum(['openai', 'anthropic', 'gemini']),
  model: z.string(),
  ideas: z.array(IdeaSchema),
  stepPlan: z.array(StepSchema),
  risks: z.array(RiskSchema),
  dependencies: z.array(z.string()),
  effort: EffortEstimateSchema,
  nextActions: z.array(NextActionSchema),
  rawResponse: z.string().optional(),
  createdAt: z.string(),
});

export const ScoreCardSchema = z.object({
  novelty: z.number().min(0).max(10),
  feasibility: z.number().min(0).max(10),
  cost: z.number().min(0).max(10),
  time: z.number().min(0).max(10),
  risk: z.number().min(0).max(10),
  clarity: z.number().min(0).max(10),
  total: z.number(),
});

export const AIResponseSchema = z.object({
  ideas: z.array(IdeaSchema),
  step_plan: z.array(StepSchema).optional(),
  stepPlan: z.array(StepSchema).optional(),
  risks: z.array(RiskSchema),
  dependencies: z.array(z.string()),
  effort: EffortEstimateSchema,
  next_actions: z.array(NextActionSchema).optional(),
  nextActions: z.array(NextActionSchema).optional(),
}).transform((d) => ({
  ideas: d.ideas,
  stepPlan: d.step_plan ?? d.stepPlan ?? [],
  risks: d.risks,
  dependencies: d.dependencies,
  effort: d.effort,
  nextActions: d.next_actions ?? d.nextActions ?? [],
}));

export type AIResponse = z.infer<typeof AIResponseSchema>;
