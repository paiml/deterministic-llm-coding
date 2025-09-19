#!/bin/bash
# Check all bad examples and generate report

echo "PMAT Code Quality Report"
echo "========================"
echo ""

TOTAL_ISSUES=0

for category in complexity satd entropy churn provability dead-code documentation; do
    echo "Checking $category..."

    if [ -d "examples/0*-$category/bad" ]; then
        ISSUES=$(pmat tdg "examples/0*-$category/bad" --format json 2>/dev/null | jq '.violations | length' 2>/dev/null || echo "0")
        TOTAL_ISSUES=$((TOTAL_ISSUES + ISSUES))

        echo "  ❌ Issues found: $ISSUES"
        pmat tdg "examples/0*-$category/bad" --format table 2>/dev/null || echo "  (PMAT analysis failed - tool may not be installed)"
    else
        echo "  📁 Directory not found: examples/0*-$category/bad"
    fi
    echo ""
done

echo "Total Issues: $TOTAL_ISSUES"
echo ""
echo "Run 'make fix' to see solutions!"