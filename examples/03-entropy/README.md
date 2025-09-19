# Entropy Reduction

## Problem

Code entropy measures duplication and repetitive patterns. High entropy indicates copy-paste programming where similar code structures are repeated instead of being abstracted into reusable functions.

## Issues in Bad Example

- **Copy-paste patterns**: Multiple functions with identical structure
- **String concatenation duplication**: Same pattern repeated across functions
- **Maintenance burden**: Changes require updating multiple locations
- **High entropy score**: PMAT detects the repetitive patterns

## Solution in Good Example

- **DRY principle**: Don't Repeat Yourself - extract common patterns
- **Generic helper functions**: Reusable code for similar operations
- **Reduced duplication**: Single implementation for common logic
- **Lower entropy**: Less repetitive code patterns

## PMAT Analysis

PMAT will detect high entropy in the bad example:

```bash
# Check bad example (high entropy from duplication)
pmat tdg examples/03-entropy/bad --format table

# Check good example (low entropy, DRY code)
pmat tdg examples/03-entropy/good --format table
```

## Key Takeaways

1. Extract common patterns into helper functions
2. Use generic functions to handle similar operations
3. Avoid copy-paste programming
4. Refactor when you notice repetitive code patterns
5. Target entropy score < 30 for maintainable code