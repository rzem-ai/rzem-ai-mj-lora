/**
 * TypeScript types matching the LoRA Training Dataset JSON Schema
 */

export type Priority = 'high' | 'medium' | 'low';

export interface StyleAnalysis {
  primary_style: string;
  era_influence: string;
  color_palette: string[];
  key_characteristics: string[];
  best_subjects: string[];
  avoid_subjects: string[];
}

export interface TrainingRecommendations {
  recommended_dataset_size: number;
  optimal_subject_distribution: Record<string, number>;
}

export interface PermutationBatch {
  batch_number: number;
  batch_name: string;
  category: string;
  image_count: number;
  prompt: string;
  priority: Priority;
  notes?: string;
}

export interface PromptGuidelines {
  keep_simple: boolean;
  avoid_style_keywords: string[];
  recommended_additions: string[];
}

export interface DatasetSpecification {
  sref_code: string;
  style_analysis: StyleAnalysis;
  training_recommendations: TrainingRecommendations;
  permutation_batches: PermutationBatch[];
  prompt_guidelines: PromptGuidelines;
}

export interface ProjectData {
  images: string[];  // base64 encoded
  imagePaths: string[];  // original file paths
  srefCode: string;
  specification: DatasetSpecification | null;
  lastModified: number;
}

export interface ValidationResult {
  isValid: boolean;
  errors: string[];
  warnings: string[];
}

export interface BatchValidation extends ValidationResult {
  calculatedCount: number;
  hasSrefCode: boolean;
  hasValidSyntax: boolean;
}
