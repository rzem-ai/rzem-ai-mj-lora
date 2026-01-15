import type { PermutationBatch, DatasetSpecification, BatchValidation, ValidationResult } from '../types/schema';

/**
 * Parse permutation syntax and count combinations
 * Example: "{mountain, ocean} with {sunrise, sunset}" -> 2 * 2 = 4
 */
export function calculatePermutationCount(prompt: string): number {
  try {
    // Extract all {...} blocks
    const permutationBlocks = prompt.match(/\{[^}]+\}/g);

    if (!permutationBlocks || permutationBlocks.length === 0) {
      return 1; // No permutations, just one image
    }

    let totalCount = 1;

    for (const block of permutationBlocks) {
      // Remove braces and split by comma
      const options = block
        .slice(1, -1)
        .split(',')
        .map(s => s.trim())
        .filter(s => s.length > 0);

      totalCount *= options.length;
    }

    return totalCount;
  } catch (error) {
    console.error('Error calculating permutation count:', error);
    return 0;
  }
}

/**
 * Validate permutation syntax
 */
export function hasValidPermutationSyntax(prompt: string): boolean {
  try {
    // Check for balanced braces
    const openBraces = (prompt.match(/\{/g) || []).length;
    const closeBraces = (prompt.match(/\}/g) || []).length;

    if (openBraces !== closeBraces) {
      return false;
    }

    // Check that all braces contain content
    const blocks = prompt.match(/\{[^}]+\}/g);
    if (blocks) {
      for (const block of blocks) {
        const content = block.slice(1, -1).trim();
        if (!content || !content.includes(',')) {
          return false;
        }
      }
    }

    return true;
  } catch (error) {
    return false;
  }
}

/**
 * Check if prompt contains SREF code
 */
export function hasSrefCode(prompt: string): boolean {
  return /--sref\s+\d+/.test(prompt);
}

/**
 * Extract SREF code from prompt
 */
export function extractSrefCode(prompt: string): string | null {
  const match = prompt.match(/--sref\s+(\d+)/);
  return match ? match[1] : null;
}

/**
 * Validate a single batch
 */
export function validateBatch(batch: PermutationBatch, expectedSrefCode?: string): BatchValidation {
  const errors: string[] = [];
  const warnings: string[] = [];

  const calculatedCount = calculatePermutationCount(batch.prompt);
  const hasValidSyntax = hasValidPermutationSyntax(batch.prompt);
  const hasSref = hasSrefCode(batch.prompt);

  // Check image count
  if (calculatedCount !== 40) {
    errors.push(`Batch generates ${calculatedCount} images, must be exactly 40`);
  }

  // Check syntax
  if (!hasValidSyntax && batch.prompt.includes('{')) {
    errors.push('Invalid permutation syntax');
  }

  // Check SREF code
  if (!hasSref) {
    errors.push('Missing --sref code in prompt');
  } else if (expectedSrefCode) {
    const batchSref = extractSrefCode(batch.prompt);
    if (batchSref !== expectedSrefCode) {
      errors.push(`SREF code mismatch: expected ${expectedSrefCode}, found ${batchSref}`);
    }
  }

  // Check priority
  if (!['high', 'medium', 'low'].includes(batch.priority)) {
    errors.push('Priority must be "high", "medium", or "low"');
  }

  // Warnings
  if (batch.prompt.length > 200) {
    warnings.push('Prompt is quite long - consider simplifying');
  }

  // Check for style keywords that might be redundant
  const styleKeywords = ['retro', 'vintage', '70s', '80s', 'poster', 'illustration', 'watercolor', 'stylized'];
  const promptLower = batch.prompt.toLowerCase();
  const foundKeywords = styleKeywords.filter(kw => promptLower.includes(kw));
  if (foundKeywords.length > 0) {
    warnings.push(`Consider removing style keywords (${foundKeywords.join(', ')}) - SREF handles styling`);
  }

  return {
    isValid: errors.length === 0,
    errors,
    warnings,
    calculatedCount,
    hasSrefCode: hasSref,
    hasValidSyntax,
  };
}

/**
 * Validate entire dataset specification
 */
export function validateDatasetSpecification(spec: DatasetSpecification): ValidationResult {
  const errors: string[] = [];
  const warnings: string[] = [];

  // Check SREF code format
  if (!/^\d{10}$/.test(spec.sref_code)) {
    warnings.push('SREF code should be a 10-digit number');
  }

  // Check number of batches
  if (spec.permutation_batches.length < 8) {
    errors.push(`Only ${spec.permutation_batches.length} batches - minimum 8 required`);
  }

  // Check for duplicate batch numbers
  const batchNumbers = spec.permutation_batches.map(b => b.batch_number);
  const uniqueNumbers = new Set(batchNumbers);
  if (uniqueNumbers.size !== batchNumbers.length) {
    errors.push('Duplicate batch numbers found');
  }

  // Validate each batch
  spec.permutation_batches.forEach((batch) => {
    const validation = validateBatch(batch, spec.sref_code);
    if (!validation.isValid) {
      errors.push(`Batch ${batch.batch_number}: ${validation.errors.join(', ')}`);
    }
    validation.warnings.forEach(warning => {
      warnings.push(`Batch ${batch.batch_number}: ${warning}`);
    });
  });

  // Check distribution percentages
  const distribution = spec.training_recommendations.optimal_subject_distribution;
  const percentages = Object.values(distribution).map(p => parseFloat(p.replace('%', '')));
  const total = percentages.reduce((sum, p) => sum + p, 0);

  if (Math.abs(total - 100) > 5) {
    warnings.push(`Subject distribution totals ${total.toFixed(1)}% - should be close to 100%`);
  }

  // Check dataset size
  const totalImages = spec.permutation_batches.reduce((sum, b) => sum + b.image_count, 0);
  if (totalImages < 50) {
    warnings.push(`Total dataset size is ${totalImages} - minimum 50 recommended`);
  }

  if (totalImages > 200) {
    warnings.push(`Total dataset size is ${totalImages} - consider reducing for more focused training`);
  }

  return {
    isValid: errors.length === 0,
    errors,
    warnings,
  };
}

/**
 * Generate a summary of the dataset
 */
export function generateDatasetSummary(spec: DatasetSpecification) {
  const totalImages = spec.permutation_batches.reduce((sum, b) => sum + b.image_count, 0);
  const highPriorityBatches = spec.permutation_batches.filter(b => b.priority === 'high').length;
  const categories = [...new Set(spec.permutation_batches.map(b => b.category))];

  return {
    totalImages,
    totalBatches: spec.permutation_batches.length,
    highPriorityBatches,
    categories: categories.length,
    categoryList: categories,
  };
}
