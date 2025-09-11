#!/bin/bash

# Test runner script for Leptos Forms RS
# This script runs different types of tests based on the command line arguments

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS] [TEST_TYPE]"
    echo ""
    echo "OPTIONS:"
    echo "  -h, --help     Show this help message"
    echo "  -v, --verbose  Enable verbose output"
    echo "  -c, --clean    Clean build artifacts before running tests"
    echo ""
    echo "TEST_TYPE:"
    echo "  unit           Run unit tests (default)"
    echo "  integration    Run integration tests"
    echo "  contracts      Run API contract tests"
    echo "  e2e            Run end-to-end tests"
    echo "  benchmarks     Run performance benchmarks"
    echo "  all            Run all tests"
    echo ""
    echo "Examples:"
    echo "  $0                    # Run unit tests"
    echo "  $0 unit              # Run unit tests"
    echo "  $0 integration       # Run integration tests"
    echo "  $0 e2e               # Run end-to-end tests"
    echo "  $0 all               # Run all tests"
    echo "  $0 -c all            # Clean and run all tests"
}

# Default values
VERBOSE=false
CLEAN=false
TEST_TYPE="unit"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_usage
            exit 0
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -c|--clean)
            CLEAN=true
            shift
            ;;
        unit|integration|contracts|e2e|benchmarks|all)
            TEST_TYPE="$1"
            shift
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Function to run cargo command with proper flags
run_cargo() {
    local cmd="$1"
    local args="$2"

    if [ "$VERBOSE" = true ]; then
        cargo $cmd $args
    else
        cargo $cmd $args --quiet
    fi
}

# Function to clean build artifacts
clean_build() {
    print_status "Cleaning build artifacts..."
    cargo clean
    print_success "Build artifacts cleaned"
}

# Function to run unit tests
run_unit_tests() {
    print_status "Running unit tests..."
    cd tests/unit
    run_cargo test
    cd ../..
    print_success "Unit tests completed"
}

# Function to run integration tests
run_integration_tests() {
    print_status "Running integration tests..."
    cd tests/integration
    run_cargo test
    cd ../..
    print_success "Integration tests completed"
}

# Function to run contract tests
run_contract_tests() {
    print_status "Running API contract tests..."
    cd tests/contracts
    run_cargo test
    cd ../..
    print_success "Contract tests completed"
}

# Function to run end-to-end tests
run_e2e_tests() {
    print_status "Running end-to-end tests..."
    cd tests/e2e
    if [ -f "package.json" ]; then
        if [ "$VERBOSE" = true ]; then
            npm test
        else
            npm test --silent
        fi
    else
        print_warning "No package.json found in e2e directory, skipping e2e tests"
    fi
    cd ../..
    print_success "End-to-end tests completed"
}

# Function to run benchmarks
run_benchmarks() {
    print_status "Running performance benchmarks..."
    cd tests/benchmarks
    run_cargo bench
    cd ../..
    print_success "Benchmarks completed"
}

# Main execution
main() {
    print_status "Starting test execution..."
    print_status "Test type: $TEST_TYPE"
    print_status "Verbose: $VERBOSE"
    print_status "Clean: $CLEAN"

    # Clean if requested
    if [ "$CLEAN" = true ]; then
        clean_build
    fi

    # Run tests based on type
    case $TEST_TYPE in
        unit)
            run_unit_tests
            ;;
        integration)
            run_integration_tests
            ;;
        contracts)
            run_contract_tests
            ;;
        e2e)
            run_e2e_tests
            ;;
        benchmarks)
            run_benchmarks
            ;;
        all)
            print_status "Running all test types..."
            run_unit_tests
            run_integration_tests
            run_contract_tests
            run_e2e_tests
            run_benchmarks
            print_success "All tests completed"
            ;;
        *)
            print_error "Unknown test type: $TEST_TYPE"
            exit 1
            ;;
    esac

    print_success "Test execution completed successfully!"
}

# Run main function
main "$@"
