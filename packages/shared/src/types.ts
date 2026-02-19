/**
 * Idea Engine - Shared types
 * Used by frontend and Rust backend via IPC
 */

export interface IdeaBundle {
  id: string;
  provider: 'openai' | 'anthropic' | 'gemini';
  model: string;
  ideas: Idea[];
  stepPlan: Step[];
  risks: Risk[];
  dependencies: string[];
  effort: EffortEstimate;
  nextActions: NextAction[];
  rawResponse?: string;
  createdAt: string;
}

export interface Idea {
  title: string;
  description: string;
  rationale?: string;
}

export interface Step {
  order: number;
  action: string;
  details?: string;
}

export interface Risk {
  description: string;
  severity: 'low' | 'medium' | 'high';
  mitigation?: string;
}

export interface EffortEstimate {
  time: string;
  cost?: string;
  complexity?: 'low' | 'medium' | 'high';
}

export interface NextAction {
  action: string;
  priority: 'immediate' | 'short' | 'medium' | 'long';
}

export interface ScoreCard {
  novelty: number;
  feasibility: number;
  cost: number;
  time: number;
  risk: number;
  clarity: number;
  total: number;
}

export type FeedbackType = 'helpful' | 'not_helpful' | 'follow_up_needed';

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  ideaBundles?: IdeaBundle[];
  feedback?: FeedbackType;
  createdAt: string;
}

export interface Chat {
  id: string;
  title: string;
  messages: ChatMessage[];
  templateId?: string;
  createdAt: string;
  updatedAt: string;
}

export interface PromptRecipe {
  id: string;
  name: string;
  systemPrompt: string;
  userPromptTemplate: string;
  rubric: Partial<ScoreCard>;
  fewShotExamples?: string[];
  createdAt: string;
}

export interface EvalResult {
  recipeId: string;
  problemId: string;
  bundleId: string;
  scoreCard: ScoreCard;
  delta?: Partial<ScoreCard>;
}
