# Complexity Management

## Problem

Complex nested functions with high cyclomatic complexity are difficult for AI assistants to understand and maintain. The `bad` example has a complexity score of 15+, making it hard to follow the logic flow.

## Issues in Bad Example

- **Deep nesting**: Multiple levels of if-else statements
- **High complexity**: Single function handles too many paths
- **Poor readability**: Hard to understand the logic flow
- **Maintenance burden**: Difficult to modify or extend

## Solution in Good Example

- **Function decomposition**: Break complex function into smaller, focused functions
- **Pattern matching**: Use `match` instead of nested if-else
- **Single responsibility**: Each function has one clear purpose
- **Low complexity**: Each function has complexity ≤ 5

## PMAT Analysis

Run PMAT to see the difference:

```bash
# Check bad example
pmat tdg examples/01-complexity/bad --format table

# Check good example
pmat tdg examples/01-complexity/good --format table
```

## Key Takeaways

1. Keep function complexity under 10 (ideally under 5)
2. Use early returns to reduce nesting
3. Extract helper functions for specific tasks
4. Use pattern matching for cleaner code flows