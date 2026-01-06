#!/bin/bash

# Lighthouse Audit Script
# Automates Lighthouse audits to ensure consistent 100% scores across all categories

set -e

# Default URL
URL="${1:-http://localhost:8080}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Create reports directory
REPORTS_DIR="./lighthouse-reports"
mkdir -p "$REPORTS_DIR"

# Paths to test
PATHS=("/" "/app" "/#app")
PRESETS=("mobile" "desktop")

# Track results
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
FAILED_DETAILS=()

echo "🔍 Starting Lighthouse Audit for $URL"
echo "=========================================="

# Function to extract score from JSON
extract_score() {
    local json_file="$1"
    local category="$2"
    node -e "
        const fs = require('fs');
        const data = JSON.parse(fs.readFileSync('$json_file'));
        console.log(Math.round(data.categories['$category'].score * 100));
    " 2>/dev/null || echo "0"
}

# Function to run audit
run_audit() {
    local path="$1"
    local preset="$2"
    local url="${URL}${path}"
    
    # Sanitize path for filename
    local safe_path=$(echo "$path" | tr '/' '_' | tr '#' '_')
    
    # Generate timestamp
    local timestamp=$(date +%Y%m%d_%H%M%S)
    
    # Report filenames
    local json_report="${REPORTS_DIR}/${safe_path}_${preset}_${timestamp}.report.json"
    local html_report="${REPORTS_DIR}/${safe_path}_${preset}_${timestamp}.report.html"
    
    echo ""
    echo "📊 Testing: $url (${preset})"
    
    # Run Lighthouse
    if ! lighthouse "$url" \
        --preset="$preset" \
        --output=json,html \
        --output-path="$json_report" \
        --quiet \
        --chrome-flags="--headless" 2>/dev/null; then
        echo -e "${RED}❌ Failed to run Lighthouse for $url (${preset})${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        FAILED_DETAILS+=("$url ($preset): Failed to run audit")
        return 1
    fi
    
    # Rename HTML report
    if [ -f "${json_report%.json}.html" ]; then
        mv "${json_report%.json}.html" "$html_report"
    fi
    
    # Extract scores
    local performance=$(extract_score "$json_report" "performance")
    local accessibility=$(extract_score "$json_report" "accessibility")
    local best_practices=$(extract_score "$json_report" "best-practices")
    local seo=$(extract_score "$json_report" "seo")
    
    # Check if all scores are 100
    local all_pass=true
    if [ "$performance" -lt 100 ]; then
        all_pass=false
        echo -e "  ${RED}Performance: ${performance}% (required: 100%)${NC}"
    else
        echo -e "  ${GREEN}Performance: ${performance}%${NC}"
    fi
    
    if [ "$accessibility" -lt 100 ]; then
        all_pass=false
        echo -e "  ${RED}Accessibility: ${accessibility}% (required: 100%)${NC}"
    else
        echo -e "  ${GREEN}Accessibility: ${accessibility}%${NC}"
    fi
    
    if [ "$best_practices" -lt 100 ]; then
        all_pass=false
        echo -e "  ${RED}Best Practices: ${best_practices}% (required: 100%)${NC}"
    else
        echo -e "  ${GREEN}Best Practices: ${best_practices}%${NC}"
    fi
    
    if [ "$seo" -lt 100 ]; then
        all_pass=false
        echo -e "  ${RED}SEO: ${seo}% (required: 100%)${NC}"
    else
        echo -e "  ${GREEN}SEO: ${seo}%${NC}"
    fi
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    if [ "$all_pass" = true ]; then
        echo -e "  ${GREEN}✅ All scores meet 100% requirement${NC}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "  ${RED}❌ Some scores below 100%${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        FAILED_DETAILS+=("$url ($preset): Performance=${performance}%, Accessibility=${accessibility}%, Best Practices=${best_practices}%, SEO=${seo}%")
    fi
    
    echo "  📄 Reports: $json_report, $html_report"
}

# Run audits for all combinations
for preset in "${PRESETS[@]}"; do
    for path in "${PATHS[@]}"; do
        run_audit "$path" "$preset"
    done
done

# Summary
echo ""
echo "=========================================="
echo "📊 Audit Summary"
echo "=========================================="
echo "Total Tests: $TOTAL_TESTS"
echo -e "${GREEN}Passed: $PASSED_TESTS${NC}"
if [ $FAILED_TESTS -gt 0 ]; then
    echo -e "${RED}Failed: $FAILED_TESTS${NC}"
    echo ""
    echo "Failed Tests:"
    for detail in "${FAILED_DETAILS[@]}"; do
        echo -e "  ${RED}❌ $detail${NC}"
    done
    exit 1
else
    echo -e "${GREEN}Failed: 0${NC}"
    echo ""
    echo -e "${GREEN}✅ All tests passed! All Lighthouse scores are 100%${NC}"
    exit 0
fi

