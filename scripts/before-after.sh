#!/bin/bash
# Show before/after comparison for all examples

echo "📊 Before/After PMAT Analysis Comparison"
echo "========================================"
echo ""

CATEGORIES=("complexity" "satd" "entropy" "provability")

for category in "${CATEGORIES[@]}"; do
    echo "🔍 Category: $category"
    echo "------------------------"

    BAD_DIR="examples/*-$category/bad"
    GOOD_DIR="examples/*-$category/good"

    echo ""
    echo "❌ BEFORE (Bad Example):"
    if command -v pmat >/dev/null 2>&1; then
        pmat tdg $BAD_DIR --format table 2>/dev/null || echo "  (PMAT analysis failed)"
    else
        echo "  ⚠️  PMAT not installed. Install with 'cargo install pmat'"
    fi

    echo ""
    echo "✅ AFTER (Good Example):"
    if command -v pmat >/dev/null 2>&1; then
        pmat tdg $GOOD_DIR --format table 2>/dev/null || echo "  (PMAT analysis failed)"
    else
        echo "  ⚠️  PMAT not installed. Install with 'cargo install pmat'"
    fi

    echo ""
    echo "=================================================="
    echo ""
done

echo "🎯 Summary:"
echo "The good examples should show:"
echo "  • Lower complexity scores"
echo "  • Zero SATD violations"
echo "  • Reduced entropy"
echo "  • Higher provability"
echo ""
echo "For detailed explanations, see the README.md in each example directory."