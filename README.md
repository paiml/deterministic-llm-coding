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
   git clone https://github.com/paiml/deterministic-llm-coding
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

### 4. Provability Improvement
- Eliminating panic potential
- Safe error handling
- Target: 100% provable code

## 🔧 How PMAT Helps

PMAT analyzes your code using:
- **AST (Abstract Syntax Tree)** parsing
- **Complexity metrics** (McCabe cyclomatic)
- **Pattern detection** (entropy analysis)
- **Comment parsing** (SATD detection)
- **Provability analysis** (panic detection)

## 📁 Repository Structure

```
deterministic-llm-coding/
├── examples/
│   ├── 01-complexity/    # Nested code → Simple functions
│   ├── 02-satd/          # TODO comments → Clean implementation
│   ├── 03-entropy/       # Copy-paste → DRY principles
│   └── 05-provability/   # Panic-prone → Safe error handling
├── scripts/              # Automation and learning tools
├── Makefile             # Build and analysis commands
└── pmat.toml           # PMAT configuration
```

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

## 🛠️ Available Commands

- `make check` - Analyze all bad examples
- `make fix` - Show all good examples
- `make learn` - Interactive tutorial
- `make compare` - Before/after comparison
- `make build` - Build all examples
- `make install-pmat` - Install PMAT tool

## 🤝 Contributing

Add your own examples! Each example needs:
1. A "bad" version with issues
2. A "good" version with fixes
3. A README explaining the problem
4. Cargo.toml for compilation

## 📖 Resources

- [PMAT Documentation](https://github.com/paiml/pmat)
- [Deterministic LLM Course](https://linkedin.com/learning/deterministic-llm)
- [Toyota Way Principles](https://docs.pmat.dev/toyota-way)

## 📝 License

MIT - Use these examples to improve your code quality!
