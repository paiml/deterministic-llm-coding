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
    if ls $BAD_DIR/*.rs >/dev/null 2>&1; then
        ls $BAD_DIR/*.rs | head -1 | xargs cat
    else
        echo "  📁 Bad example not found for $category"
    fi

    echo ""
    echo "2️⃣  Running PMAT analysis..."
    if command -v pmat >/dev/null 2>&1; then
        pmat tdg $BAD_DIR --format table 2>/dev/null || echo "  (PMAT analysis failed)"
    else
        echo "  ⚠️  PMAT not installed. Run 'make install-pmat' first."
    fi

    echo ""
    echo "Press Enter to see the fix..."
    read

    echo "3️⃣  Here's the fixed version:"
    if ls $GOOD_DIR/*.rs >/dev/null 2>&1; then
        ls $GOOD_DIR/*.rs | head -1 | xargs cat
    else
        echo "  📁 Good example not found for $category"
    fi

    echo ""
    echo "4️⃣  Verifying the fix..."
    if command -v pmat >/dev/null 2>&1; then
        pmat tdg $GOOD_DIR --format table 2>/dev/null || echo "  (PMAT analysis failed)"
    else
        echo "  ⚠️  PMAT not installed. Run 'make install-pmat' first."
    fi

    echo ""
    echo "✅ Lesson complete!"
    echo "Press Enter for next lesson..."
    read
done

echo "🎉 Congratulations! You've completed all lessons!"