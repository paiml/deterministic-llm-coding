# Deterministic LLM Coding - GitHub Repository Specification

## Repository: `deterministic-llm-coding`

### Purpose
Educational repository demonstrating how PMAT (Pragmatic AI Labs MCP Agent Toolkit) detects and helps fix common code quality issues in the context of deterministic LLM programming.

---

## Repository Structure

```
deterministic-llm-coding/
├── README.md                          # Main documentation
├── Cargo.toml                         # Rust workspace configuration
├── pmat.toml                          # PMAT configuration
├── Makefile                           # Automation commands
├── .github/
│   └── workflows/
│       └── quality-gates.yml         # CI/CD with PMAT checks
├── examples/
│   ├── 01-complexity/
│   │   ├── bad/
│   │   │   └── nested_nightmare.rs   # Complex nested code
│   │   ├── good/
│   │   │   └── simple_functions.rs   # Refactored version
│   │   └── README.md                 # Complexity explanation
│   ├── 02-satd/
│   │   ├── bad/
│   │   │   └── todo_everywhere.rs    # Self-admitted technical debt
│   │   ├── good/
│   │   │   └── clean_code.rs         # No TODOs, proper implementation
│   │   └── README.md                 # SATD explanation
│   ├── 03-entropy/
│   │   ├── bad/
│   │   │   └── copy_paste_code.rs    # Duplicated code patterns
│   │   ├── good/
│   │   │   └── dry_code.rs           # DRY principle applied
│   │   └── README.md                 # Entropy explanation
│   ├── 04-churn/
│   │   ├── bad/
│   │   │   └── unstable_api.rs       # Frequently changing code
│   │   ├── good/
│   │   │   └── stable_interface.rs   # Well-designed stable API
│   │   └── README.md                 # Code churn explanation
│   ├── 05-provability/
│   │   ├── bad/
│   │   │   └── unsafe_operations.rs  # Code with panic potential
│   │   ├── good/
│   │   │   └── safe_code.rs          # Proper error handling
│   │   └── README.md                 # Provability explanation
│   ├── 06-dead-code/
│   │   ├── bad/
│   │   │   └── unused_functions.rs   # Dead code examples
│   │   ├── good/
│   │   │   └── all_used.rs           # Clean, used code only
│   │   └── README.md                 # Dead code explanation
│   └── 07-documentation/
│       ├── bad/
│       │   └── no_docs.rs            # Undocumented public APIs
│       ├── good/
│       │   └── well_documented.rs    # Properly documented
│       └── README.md                 # Documentation coverage
├── scripts/
│   ├── check-all.sh                  # Run PMAT on all examples
│   ├── fix-guided.sh                 # Interactive fixing guide
│   └── before-after.sh               # Show improvements
└── docs/
    ├── GETTING_STARTED.md            # Quick start guide
    ├── PMAT_BASICS.md                # PMAT fundamentals
    └── EXERCISES.md                  # Practice exercises

```

---

## Example Code Samples

### 1. Complexity Example (`examples/01-complexity/bad/nested_nightmare.rs`)

```rust
// BAD: Cyclomatic complexity of 15+ (PMAT will flag this)
fn process_user_input(input: &str, mode: i32, flags: u32) -> Result<String, String> {
    if input.is_empty() {
        return Err("Empty input".to_string());
    } else {
        if mode == 1 {
            if flags & 0x01 != 0 {
                if input.len() > 100 {
                    if input.contains("admin") {
                        return Ok("Admin mode".to_string());
                    } else {
                        if input.contains("user") {
                            return Ok("User mode".to_string());
                        } else {
                            return Ok("Guest mode".to_string());
                        }
                    }
                } else {
                    return Err("Input too short".to_string());
                }
            } else {
                return Err("Flag not set".to_string());
            }
        } else if mode == 2 {
            if flags & 0x02 != 0 {
                return Ok("Mode 2 active".to_string());
            } else {
                return Err("Mode 2 requires flag".to_string());
            }
        } else {
            return Err("Unknown mode".to_string());
        }
    }
}
```

**Fixed Version (`examples/01-complexity/good/simple_functions.rs`):**

```rust
// GOOD: Complexity under 5 per function
fn process_user_input(input: &str, mode: i32, flags: u32) -> Result<String, String> {
    validate_input(input)?;
    
    match mode {
        1 => process_mode_one(input, flags),
        2 => process_mode_two(flags),
        _ => Err("Unknown mode".to_string()),
    }
}

fn validate_input(input: &str) -> Result<(), String> {
    if input.is_empty() {
        Err("Empty input".to_string())
    } else {
        Ok(())
    }
}

fn process_mode_one(input: &str, flags: u32) -> Result<String, String> {
    if flags & 0x01 == 0 {
        return Err("Flag not set".to_string());
    }
    
    if input.len() <= 100 {
        return Err("Input too short".to_string());
    }
    
    Ok(determine_user_type(input))
}

fn determine_user_type(input: &str) -> String {
    if input.contains("admin") {
        "Admin mode".to_string()
    } else if input.contains("user") {
        "User mode".to_string()
    } else {
        "Guest mode".to_string()
    }
}

fn process_mode_two(flags: u32) -> Result<String, String> {
    if flags & 0x02 != 0 {
        Ok("Mode 2 active".to_string())
    } else {
        Err("Mode 2 requires flag".to_string())
    }
}
```

### 2. SATD Example (`examples/02-satd/bad/todo_everywhere.rs`)

```rust
// BAD: Multiple TODOs and FIXMEs (PMAT will flag these)
fn calculate_price(quantity: f64, price_per_unit: f64) -> f64 {
    // TODO: Add tax calculation
    // FIXME: This doesn't handle discounts
    let base_price = quantity * price_per_unit;
    
    // HACK: Hardcoded discount for now
    if quantity > 100.0 {
        base_price * 0.9  // TODO: Make this configurable
    } else {
        base_price
    }
}

// TODO: Implement this function
fn apply_shipping_cost(_weight: f64) -> f64 {
    0.0  // FIXME: Actually calculate shipping
}
```

**Fixed Version (`examples/02-satd/good/clean_code.rs`):**

```rust
// GOOD: No technical debt comments, proper implementation
const BULK_DISCOUNT_THRESHOLD: f64 = 100.0;
const BULK_DISCOUNT_RATE: f64 = 0.9;
const TAX_RATE: f64 = 0.08;

fn calculate_price(quantity: f64, price_per_unit: f64, include_tax: bool) -> f64 {
    let base_price = quantity * price_per_unit;
    let discounted_price = apply_bulk_discount(base_price, quantity);
    
    if include_tax {
        discounted_price * (1.0 + TAX_RATE)
    } else {
        discounted_price
    }
}

fn apply_bulk_discount(price: f64, quantity: f64) -> f64 {
    if quantity > BULK_DISCOUNT_THRESHOLD {
        price * BULK_DISCOUNT_RATE
    } else {
        price
    }
}

fn calculate_shipping_cost(weight_kg: f64) -> f64 {
    const BASE_RATE: f64 = 5.0;
    const PER_KG_RATE: f64 = 0.5;
    
    BASE_RATE + (weight_kg * PER_KG_RATE)
}
```

### 3. Entropy Example (`examples/03-entropy/bad/copy_paste_code.rs`)

```rust
// BAD: Duplicated patterns (PMAT entropy detection will flag this)
fn process_user(name: &str, age: u32) -> String {
    let mut result = String::new();
    result.push_str("User: ");
    result.push_str(name);
    result.push_str(", Age: ");
    result.push_str(&age.to_string());
    result
}

fn process_admin(name: &str, level: u32) -> String {
    let mut result = String::new();
    result.push_str("Admin: ");
    result.push_str(name);
    result.push_str(", Level: ");
    result.push_str(&level.to_string());
    result
}

fn process_guest(name: &str, duration: u32) -> String {
    let mut result = String::new();
    result.push_str("Guest: ");
    result.push_str(name);
    result.push_str(", Duration: ");
    result.push_str(&duration.to_string());
    result
}
```

**Fixed Version (`examples/03-entropy/good/dry_code.rs`):**

```rust
// GOOD: DRY principle applied, no duplication
fn format_entity(entity_type: &str, name: &str, attribute: &str, value: u32) -> String {
    format!("{}: {}, {}: {}", entity_type, name, attribute, value)
}

fn process_user(name: &str, age: u32) -> String {
    format_entity("User", name, "Age", age)
}

fn process_admin(name: &str, level: u32) -> String {
    format_entity("Admin", name, "Level", level)
}

fn process_guest(name: &str, duration: u32) -> String {
    format_entity("Guest", name, "Duration", duration)
}
```

### 4. Provability Example (`examples/05-provability/bad/unsafe_operations.rs`)

```rust
// BAD: Can panic, low provability score
fn divide_numbers(a: i32, b: i32) -> i32 {
    a / b  // Will panic if b == 0
}

fn get_array_element(arr: &[i32], index: usize) -> i32 {
    arr[index]  // Will panic if index out of bounds
}

fn parse_number(s: &str) -> i32 {
    s.parse().unwrap()  // Will panic on invalid input
}
```

**Fixed Version (`examples/05-provability/good/safe_code.rs`):**

```rust
// GOOD: No panic potential, high provability
fn divide_numbers(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn get_array_element(arr: &[i32], index: usize) -> Option<i32> {
    arr.get(index).copied()
}

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse()
        .map_err(|e| format!("Failed to parse number: {}", e))
}
```

---

## PMAT Configuration (`pmat.toml`)

```toml
[project]
name = "deterministic-llm-coding"
version = "1.0.0"

[quality_gates]
max_complexity = 10
max_satd_count = 0
max_entropy_score = 30
min_documentation_coverage = 80
max_dead_code_percentage = 5

[checks]
enabled = [
    "complexity",
    "satd",
    "entropy",
    "dead_code",
    "documentation",
    "provability"
]

[output]
format = "json"
verbose = true
```

---

## Makefile

```makefile
.PHONY: check fix learn clean

# Check all examples with PMAT
check:
	@echo "🔍 Analyzing bad examples..."
	@for dir in examples/*/bad; do \
		echo "\n📁 Checking $$dir"; \
		pmat tdg $$dir --format table; \
	done

# Show fixes
fix:
	@echo "✅ Showing good examples..."
	@for dir in examples/*/good; do \
		echo "\n📁 Checking $$dir"; \
		pmat tdg $$dir --format table; \
	done

# Interactive learning mode
learn:
	@bash scripts/fix-guided.sh

# Before/after comparison
compare:
	@bash scripts/before-after.sh

# Install PMAT
install-pmat:
	cargo install pmat

# Setup pre-commit hooks
setup-hooks:
	pmat quality-gate hooks install

# Clean build artifacts
clean:
	cargo clean
```

---

## README.md Content

```markdown
# Deterministic LLM Coding Examples

Learn how to write deterministic, high-quality code that AI assistants can understand and maintain effectively.

## 🎯 Purpose

This repository demonstrates common code quality issues that PMAT (Pragmatic AI Labs MCP Agent Toolkit) can detect, and how to fix them. Each example is designed to be:

- **Simple**: ELI5-level complexity
- **Focused**: One issue per example
- **Practical**: Real problems you'll encounter
- **Educational**: Learn by doing

## 🚀 Quick Start

1. **Install PMAT**:
   ```bash
   cargo install pmat
   ```

2. **Clone this repo**:
   ```bash
   git clone https://github.com/yourusername/deterministic-llm-coding
   cd deterministic-llm-coding
   ```

3. **Check bad examples**:
   ```bash
   make check
   ```

4. **See the fixes**:
   ```bash
   make fix
   ```

5. **Learn interactively**:
   ```bash
   make learn
   ```

## 📚 What You'll Learn

### 1. Complexity Management
- Why nested if-statements hurt AI understanding
- How to refactor for clarity
- Target: Complexity ≤ 10

### 2. SATD (Self-Admitted Technical Debt)
- Why TODOs and FIXMEs accumulate
- How to eliminate technical debt
- Target: 0 SATD comments

### 3. Entropy Reduction
- Spotting copy-paste patterns
- Applying DRY principles
- Target: Entropy score < 30

### 4. Code Churn Prevention
- Identifying unstable interfaces
- Designing for stability
- Target: Minimal changes over time

### 5. Provability Improvement
- Eliminating panic potential
- Safe error handling
- Target: 100% provable code

### 6. Dead Code Elimination
- Finding unused functions
- Keeping codebase clean
- Target: < 5% dead code

### 7. Documentation Coverage
- Writing clear API docs
- Helping AI understand intent
- Target: > 80% coverage

## 🔧 How PMAT Helps

PMAT analyzes your code using:
- **AST (Abstract Syntax Tree)** parsing
- **Complexity metrics** (McCabe cyclomatic)
- **Pattern detection** (entropy analysis)
- **Reachability analysis** (dead code)
- **Comment parsing** (SATD detection)

## 🎓 Interactive Learning

Run the guided tutorial:
```bash
make learn
```

This will:
1. Show you a bad example
2. Run PMAT to identify issues
3. Explain what's wrong
4. Show you how to fix it
5. Verify the fix works

## 📈 Progress Tracking

Track your improvements:
```bash
pmat tdg . --format dashboard
```

## 🤝 Contributing

Add your own examples! Each example needs:
1. A "bad" version with issues
2. A "good" version with fixes
3. A README explaining the problem
4. Tests to verify the fix

## 📖 Resources

- [PMAT Documentation](https://github.com/paiml/pmat)
- [Deterministic LLM Course](https://linkedin.com/learning/deterministic-llm)
- [Toyota Way Principles](https://docs.pmat.dev/toyota-way)

## 📝 License

MIT - Use these examples to improve your code quality!
```

---

## Script Examples

### `scripts/check-all.sh`

```bash
#!/bin/bash
# Check all bad examples and generate report

echo "PMAT Code Quality Report"
echo "========================"
echo ""

TOTAL_ISSUES=0

for category in complexity satd entropy churn provability dead-code documentation; do
    echo "Checking $category..."
    
    if [ -d "examples/0*-$category/bad" ]; then
        ISSUES=$(pmat tdg "examples/0*-$category/bad" --format json | jq '.violations | length')
        TOTAL_ISSUES=$((TOTAL_ISSUES + ISSUES))
        
        echo "  ❌ Issues found: $ISSUES"
        pmat tdg "examples/0*-$category/bad" --format table
    fi
    echo ""
done

echo "Total Issues: $TOTAL_ISSUES"
echo ""
echo "Run 'make fix' to see solutions!"
```

### `scripts/fix-guided.sh`

```bash
#!/bin/bash
# Interactive guided fixing tutorial

echo "🎓 Welcome to PMAT Interactive Learning!"
echo ""

CATEGORIES=("complexity" "satd" "entropy" "provability")

for category in "${CATEGORIES[@]}"; do
    echo "📚 Lesson: $category"
    echo "===================="
    
    BAD_DIR="examples/*-$category/bad"
    GOOD_DIR="examples/*-$category/good"
    
    echo "1️⃣  Examining bad code..."
    ls $BAD_DIR/*.rs | head -1 | xargs cat
    
    echo ""
    echo "2️⃣  Running PMAT analysis..."
    pmat tdg $BAD_DIR --format table
    
    echo ""
    echo "Press Enter to see the fix..."
    read
    
    echo "3️⃣  Here's the fixed version:"
    ls $GOOD_DIR/*.rs | head -1 | xargs cat
    
    echo ""
    echo "4️⃣  Verifying the fix..."
    pmat tdg $GOOD_DIR --format table
    
    echo ""
    echo "✅ Lesson complete!"
    echo "Press Enter for next lesson..."
    read
done

echo "🎉 Congratulations! You've completed all lessons!"
```

---

## GitHub Actions Workflow (`.github/workflows/quality-gates.yml`)

```yaml
name: PMAT Quality Gates

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  quality-check:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Install PMAT
      run: cargo install pmat
      
    - name: Check Bad Examples (Should Fail)
      run: |
        for dir in examples/*/bad; do
          echo "Checking $dir should have issues..."
          pmat tdg "$dir" || true
        done
        
    - name: Check Good Examples (Should Pass)
      run: |
        for dir in examples/*/good; do
          echo "Checking $dir should pass..."
          pmat tdg "$dir" --fail-on-violation
        done
        
    - name: Generate Report
      run: |
        pmat tdg examples --format sarif > pmat-report.sarif
        
    - name: Upload SARIF
      uses: github/codeql-action/upload-sarif@v2
      with:
        sarif_file: pmat-report.sarif
```

---

## Exercise Ideas (`docs/EXERCISES.md`)

```markdown
# PMAT Exercises

## Beginner Level

### Exercise 1: Fix the Complexity
```rust
// Make this function have complexity < 5
fn validate_password(password: &str) -> bool {
    if password.len() < 8 {
        false
    } else if password.len() > 100 {
        false
    } else if !password.chars().any(|c| c.is_numeric()) {
        false
    } else if !password.chars().any(|c| c.is_alphabetic()) {
        false
    } else if !password.chars().any(|c| c.is_uppercase()) {
        false
    } else {
        true
    }
}
```

### Exercise 2: Remove the Technical Debt
Find and fix all SATD comments in the provided code.

### Exercise 3: DRY This Code
Identify the repeated patterns and refactor them.

## Intermediate Level

### Exercise 4: Improve Provability
Make this code panic-free with proper error handling.

### Exercise 5: Eliminate Dead Code
Find and remove all unused functions.

## Advanced Level

### Exercise 6: Reduce Entropy Score to < 20
Refactor a complex module to minimize code duplication.

### Exercise 7: Document for AI
Add documentation that helps AI understand your code's intent.
```

---

This specification provides everything needed to create an educational GitHub repository that demonstrates PMAT's capabilities with simple, understandable examples that users can learn from and fix.
