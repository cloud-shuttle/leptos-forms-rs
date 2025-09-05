# CI/CD Pipeline Documentation - Leptos Forms

**Project**: Leptos Forms Library
**Version**: 1.0
**Date**: 2025-01-02
**Status**: Implementation Guide

## 1. Pipeline Overview

### 1.1 Pipeline Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Source Control (GitHub)                  │
├─────────────────────────────────────────────────────────────┤
│  Push/PR → Trigger Webhooks → GitHub Actions Runners       │
├─────────────────────────────────────────────────────────────┤
│                  Continuous Integration                     │
│  ┌─────────────┬─────────────┬─────────────┬─────────────┐ │
│  │   Build     │    Test     │   Quality   │   Security  │ │
│  │   Pipeline  │   Pipeline  │   Pipeline  │   Pipeline  │ │
│  └─────────────┴─────────────┴─────────────┴─────────────┘ │
├─────────────────────────────────────────────────────────────┤
│                  Continuous Deployment                     │
│  ┌─────────────┬─────────────┬─────────────┬─────────────┐ │
│  │   Package   │   Release   │    Docs     │   Monitor   │ │
│  │  Pipeline   │   Pipeline  │   Pipeline  │   Pipeline  │ │
│  └─────────────┴─────────────┴─────────────┴─────────────┘ │
├─────────────────────────────────────────────────────────────┤
│                      Target Environments                   │
│  crates.io │ docs.rs │ GitHub Releases │ Documentation   │
└─────────────────────────────────────────────────────────────┘
```

### 1.2 Pipeline Triggers

| Trigger      | Pipeline          | Purpose                                    |
| ------------ | ----------------- | ------------------------------------------ |
| Push to main | Full CI/CD        | Integration testing, automated release     |
| Pull Request | CI only           | Code validation, quality gates             |
| Release tag  | Release pipeline  | Package publishing, documentation          |
| Scheduled    | Nightly build     | Dependency updates, performance regression |
| Manual       | Specific workflow | Ad-hoc testing, emergency fixes            |

### 1.3 Environment Strategy

| Environment | Purpose                | Deployment              | Access           |
| ----------- | ---------------------- | ----------------------- | ---------------- |
| Development | Feature development    | Manual                  | Developers       |
| Staging     | Pre-production testing | Automatic (main branch) | QA, Stakeholders |
| Production  | Live releases          | Automatic (tags)        | Public           |

## 2. GitHub Actions Workflows

### 2.1 Main CI Workflow

```yaml
# .github/workflows/ci.yml
name: Continuous Integration

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]
  schedule:
    - cron: "0 2 * * *" # Daily at 2 AM UTC

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  # Job 1: Code Quality Checks
  quality:
    name: Code Quality
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Check formatting
        run: cargo fmt --all -- --check

      - name: Run Clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Check documentation
        run: cargo doc --all-features --no-deps --document-private-items

      - name: Audit dependencies
        run: |
          cargo install cargo-audit
          cargo audit

  # Job 2: Build Matrix
  build:
    name: Build (${{ matrix.os }}, ${{ matrix.rust }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta, nightly]
        include:
          - os: ubuntu-latest
            rust: 1.70.0 # MSRV
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust ${{ matrix.rust }}
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}

      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-${{ matrix.rust }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Build library
        run: cargo build --verbose --all-features

      - name: Build without default features
        run: cargo build --verbose --no-default-features

      - name: Build examples
        run: cargo build --verbose --examples

  # Job 3: Test Suite
  test:
    name: Test Suite (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Run unit tests
        run: cargo test --all-features --lib

      - name: Run integration tests
        run: cargo test --all-features --test integration_tests

      - name: Run documentation tests
        run: cargo test --all-features --doc

  # Job 4: WASM Build
  wasm:
    name: WASM Build and Test
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-wasm-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Build WASM package
        run: wasm-pack build --target web --out-dir pkg

      - name: Test WASM in Node.js
        run: wasm-pack test --node

      - name: Test WASM in browser (headless)
        run: wasm-pack test --headless --chrome

      - name: Check WASM bundle size
        run: |
          du -h pkg/*.wasm
          if [ $(du -b pkg/*.wasm | cut -f1) -gt 153600 ]; then  # 150KB limit
            echo "WASM bundle too large!"
            exit 1
          fi

  # Job 5: Coverage Analysis
  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Install cargo-tarpaulin
        run: cargo install cargo-tarpaulin

      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-coverage-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Generate coverage report
        run: |
          cargo tarpaulin \
            --all-features \
            --exclude-files 'tests/*' \
            --exclude-files 'examples/*' \
            --exclude-files 'benches/*' \
            --timeout 300 \
            --out Html Lcov \
            --output-dir coverage

      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v3
        with:
          files: coverage/lcov.info
          fail_ci_if_error: true

      - name: Check coverage threshold
        run: |
          COVERAGE=$(grep -oP 'lines......: \K[0-9.]+' coverage/lcov.info | tail -1)
          echo "Coverage: ${COVERAGE}%"
          if (( $(echo "$COVERAGE < 90.0" | bc -l) )); then
            echo "Coverage ${COVERAGE}% is below minimum 90%"
            exit 1
          fi

      - name: Archive coverage report
        uses: actions/upload-artifact@v3
        with:
          name: coverage-report
          path: coverage/

  # Job 6: Performance Benchmarks
  benchmarks:
    name: Performance Benchmarks
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-bench-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Run benchmarks
        run: cargo bench --all-features

      - name: Store benchmark results
        uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: "cargo"
          output-file-path: target/criterion/report/index.html
          github-token: ${{ secrets.GITHUB_TOKEN }}
          auto-push: true
          comment-on-alert: true
          alert-threshold: "110%" # Alert if performance degrades by 10%

  # Job 7: Security Scan
  security:
    name: Security Scan
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Run cargo-audit
        uses: actions-rs/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}

      - name: Run cargo-deny
        run: |
          cargo install cargo-deny
          cargo deny check all

      - name: Scan for secrets
        uses: trufflesecurity/trufflehog@v3.63.2
        with:
          path: ./
          base: main
          head: HEAD
```

### 2.2 E2E Testing Workflow

```yaml
# .github/workflows/e2e.yml
name: End-to-End Testing

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  schedule:
    - cron: "0 4 * * *" # Daily at 4 AM UTC

jobs:
  e2e-tests:
    name: E2E Tests (${{ matrix.browser }})
    runs-on: ubuntu-latest
    strategy:
      matrix:
        browser: [chromium, firefox, webkit]
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: "18"

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Install dependencies
        run: |
          npm install -g wasm-pack
          npm install playwright @playwright/test

      - name: Install Playwright browsers
        run: npx playwright install ${{ matrix.browser }}

      - name: Build examples for E2E testing
        run: |
          cd examples/contact-form
          wasm-pack build --target web
          npm install
          npm run build

      - name: Start development server
        run: |
          cd examples/contact-form
          npm run serve &
          sleep 5  # Wait for server to start

      - name: Run E2E tests
        run: npx playwright test --browser=${{ matrix.browser }}

      - name: Upload test results
        uses: actions/upload-artifact@v3
        if: failure()
        with:
          name: e2e-results-${{ matrix.browser }}
          path: test-results/

  cross-browser-compatibility:
    name: Cross-Browser Compatibility
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        browser: [chrome, firefox, safari, edge]
        exclude:
          - os: ubuntu-latest
            browser: safari
          - os: ubuntu-latest
            browser: edge
          - os: windows-latest
            browser: safari
          - os: macos-latest
            browser: edge
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Run cross-browser tests
        uses: ./.github/actions/browser-test
        with:
          os: ${{ matrix.os }}
          browser: ${{ matrix.browser }}
```

### 2.3 Release Pipeline

```yaml
# .github/workflows/release.yml
name: Release Pipeline

on:
  push:
    tags:
      - "v*.*.*"

env:
  CARGO_TERM_COLOR: always

jobs:
  # Job 1: Pre-release validation
  validate:
    name: Pre-release Validation
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Validate version consistency
        run: |
          TAG_VERSION=${GITHUB_REF#refs/tags/v}
          CARGO_VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
          if [ "$TAG_VERSION" != "$CARGO_VERSION" ]; then
            echo "Version mismatch: tag=$TAG_VERSION, Cargo.toml=$CARGO_VERSION"
            exit 1
          fi

      - name: Check changelog
        run: |
          TAG_VERSION=${GITHUB_REF#refs/tags/v}
          if ! grep -q "## \[$TAG_VERSION\]" CHANGELOG.md; then
            echo "Version $TAG_VERSION not found in CHANGELOG.md"
            exit 1
          fi

      - name: Run full test suite
        run: |
          cargo test --all-features
          cargo test --all-features --examples

  # Job 2: Build release artifacts
  build-release:
    name: Build Release Artifacts
    runs-on: ${{ matrix.os }}
    needs: validate
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Build optimized WASM
        run: |
          wasm-pack build --release --target web --out-dir pkg

      - name: Optimize WASM binary
        run: |
          cargo install wasm-opt
          wasm-opt -Oz pkg/leptos_forms_bg.wasm -o pkg/leptos_forms_bg.wasm

      - name: Create release archive
        run: |
          tar -czf leptos-forms-${{ matrix.os }}.tar.gz pkg/

      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: release-${{ matrix.os }}
          path: leptos-forms-${{ matrix.os }}.tar.gz

  # Job 3: Publish to crates.io
  publish:
    name: Publish to crates.io
    runs-on: ubuntu-latest
    needs: [validate, build-release]
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Publish leptos-forms-macro
        run: |
          cd leptos-forms-macro
          cargo publish --token ${{ secrets.CRATES_IO_TOKEN }}

      - name: Wait for macro crate to propagate
        run: sleep 60

      - name: Publish leptos-forms
        run: |
          cargo publish --token ${{ secrets.CRATES_IO_TOKEN }}

  # Job 4: Create GitHub release
  github-release:
    name: Create GitHub Release
    runs-on: ubuntu-latest
    needs: [publish, build-release]
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Download all artifacts
        uses: actions/download-artifact@v3

      - name: Extract changelog entry
        id: changelog
        run: |
          TAG_VERSION=${GITHUB_REF#refs/tags/v}
          awk "/^## \[$TAG_VERSION\]/,/^## \[/{if(/^## \[/ && !/^## \[$TAG_VERSION\]/) exit; if(!/^## \[$TAG_VERSION\]/) print}" CHANGELOG.md > release-notes.md

      - name: Create GitHub release
        uses: softprops/action-gh-release@v1
        with:
          body_path: release-notes.md
          files: |
            release-*/leptos-forms-*.tar.gz
          draft: false
          prerelease: false
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

  # Job 5: Update documentation
  docs:
    name: Update Documentation
    runs-on: ubuntu-latest
    needs: publish
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          token: ${{ secrets.DOCS_TOKEN }}

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Build documentation
        run: |
          cargo doc --all-features --no-deps

      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.DOCS_TOKEN }}
          publish_dir: target/doc
          cname: leptos-forms.dev

  # Job 6: Notify stakeholders
  notify:
    name: Notify Stakeholders
    runs-on: ubuntu-latest
    needs: [github-release, docs]
    steps:
      - name: Send Discord notification
        uses: Ilshidur/action-discord@master
        env:
          DISCORD_WEBHOOK: ${{ secrets.DISCORD_WEBHOOK }}
        with:
          args: |
            🎉 Leptos Forms ${{ github.ref_name }} has been released!
            📦 Available on crates.io: https://crates.io/crates/leptos-forms
            📖 Documentation: https://leptos-forms.dev
            🔗 Release Notes: ${{ github.server_url }}/${{ github.repository }}/releases/tag/${{ github.ref_name }}

      - name: Tweet announcement
        uses: ethomson/send-tweet-action@v1
        with:
          status: |
            🚀 Leptos Forms ${{ github.ref_name }} is now available!

            ✨ Type-safe reactive forms for @leptos_rs
            🎯 <15KB bundle size
            ⚡ <1ms field updates
            🛡️ Compile-time validation

            Get started: https://leptos-forms.dev
            #RustLang #WebDev #Leptos
        env:
          TWITTER_CONSUMER_API_KEY: ${{ secrets.TWITTER_API_KEY }}
          TWITTER_CONSUMER_API_SECRET: ${{ secrets.TWITTER_API_SECRET }}
          TWITTER_ACCESS_TOKEN: ${{ secrets.TWITTER_ACCESS_TOKEN }}
          TWITTER_ACCESS_TOKEN_SECRET: ${{ secrets.TWITTER_ACCESS_SECRET }}
```

## 3. Quality Gates and Validation

### 3.1 Automated Quality Checks

#### Code Quality Gate

```bash
#!/bin/bash
# scripts/quality-gate.sh

set -e

echo "🔍 Running quality checks..."

# Formatting check
echo "📐 Checking code formatting..."
cargo fmt --all -- --check

# Linting
echo "📋 Running Clippy lints..."
cargo clippy --all-targets --all-features -- -D warnings

# Documentation
echo "📚 Checking documentation..."
cargo doc --all-features --no-deps --document-private-items

# Tests
echo "🧪 Running test suite..."
cargo test --all-features

# Coverage
echo "📊 Checking test coverage..."
cargo tarpaulin --all-features --out Stdout | grep "Coverage Results" | awk '{
  coverage = $3;
  gsub(/%/, "", coverage);
  if (coverage < 90) {
    print "❌ Coverage " coverage "% is below minimum 90%";
    exit 1;
  } else {
    print "✅ Coverage " coverage "% meets minimum requirement";
  }
}'

# Security audit
echo "🛡️ Running security audit..."
cargo audit

# Bundle size check
echo "📦 Checking bundle size..."
wasm-pack build --release --target web --out-dir temp-pkg
SIZE=$(wc -c < temp-pkg/leptos_forms_bg.wasm)
MAX_SIZE=153600  # 150KB
if [ $SIZE -gt $MAX_SIZE ]; then
    echo "❌ Bundle size ${SIZE} bytes exceeds maximum ${MAX_SIZE} bytes"
    exit 1
else
    echo "✅ Bundle size ${SIZE} bytes within limit"
fi
rm -rf temp-pkg

echo "✅ All quality checks passed!"
```

#### Performance Gate

```bash
#!/bin/bash
# scripts/performance-gate.sh

set -e

echo "⚡ Running performance validation..."

# Run benchmarks
cargo bench --all-features > benchmark-results.txt

# Check critical performance metrics
FIELD_UPDATE_TIME=$(grep "field_update_time" benchmark-results.txt | awk '{print $2}' | sed 's/ms//')
FORM_VALIDATION_TIME=$(grep "form_validation_time" benchmark-results.txt | awk '{print $2}' | sed 's/ms//')

if (( $(echo "$FIELD_UPDATE_TIME > 1.0" | bc -l) )); then
    echo "❌ Field update time ${FIELD_UPDATE_TIME}ms exceeds 1ms target"
    exit 1
fi

if (( $(echo "$FORM_VALIDATION_TIME > 2.0" | bc -l) )); then
    echo "❌ Form validation time ${FORM_VALIDATION_TIME}ms exceeds 2ms target"
    exit 1
fi

echo "✅ Performance targets met!"
echo "  - Field updates: ${FIELD_UPDATE_TIME}ms (target: <1ms)"
echo "  - Form validation: ${FORM_VALIDATION_TIME}ms (target: <2ms)"
```

### 3.2 Release Validation

#### Pre-release Checklist

```yaml
# .github/workflows/pre-release-checklist.yml
name: Pre-release Checklist

on:
  workflow_dispatch:
    inputs:
      version:
        description: "Version to release (e.g., 1.0.0)"
        required: true

jobs:
  checklist:
    name: Release Readiness Check
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Validate release checklist
        run: |
          VERSION=${{ github.event.inputs.version }}

          # Check version consistency
          if ! grep -q "version = \"$VERSION\"" Cargo.toml; then
            echo "❌ Version $VERSION not found in Cargo.toml"
            exit 1
          fi

          # Check changelog
          if ! grep -q "## \[$VERSION\]" CHANGELOG.md; then
            echo "❌ Version $VERSION not documented in CHANGELOG.md"
            exit 1
          fi

          # Check all tests pass
          cargo test --all-features

          # Check documentation builds
          cargo doc --all-features --no-deps

          # Check examples work
          cd examples/contact-form
          wasm-pack build --target web

          # Security audit
          cargo audit

          echo "✅ Release $VERSION is ready!"

          # Create release checklist issue
          gh issue create \
            --title "Release $VERSION Checklist" \
            --body "$(cat .github/RELEASE_CHECKLIST_TEMPLATE.md)" \
            --label "release"
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## 4. Deployment Strategies

### 4.1 Staging Deployment

```yaml
# .github/workflows/deploy-staging.yml
name: Deploy to Staging

on:
  push:
    branches: [main]

jobs:
  deploy-staging:
    name: Deploy to Staging
    runs-on: ubuntu-latest
    environment: staging
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Build documentation site
        run: |
          cargo doc --all-features --no-deps
          mkdir -p staging-site
          cp -r target/doc/* staging-site/

      - name: Build examples
        run: |
          cd examples/contact-form
          wasm-pack build --target web
          npm install && npm run build
          cp -r dist/* ../../staging-site/examples/

      - name: Deploy to Netlify
        uses: nwtgck/actions-netlify@v2.0
        with:
          publish-dir: "./staging-site"
          production-deploy: false
          github-token: ${{ secrets.GITHUB_TOKEN }}
          deploy-message: "Staging deployment for commit ${{ github.sha }}"
        env:
          NETLIFY_AUTH_TOKEN: ${{ secrets.NETLIFY_AUTH_TOKEN }}
          NETLIFY_SITE_ID: ${{ secrets.NETLIFY_STAGING_SITE_ID }}
```

### 4.2 Production Deployment

```yaml
# .github/workflows/deploy-production.yml
name: Deploy to Production

on:
  release:
    types: [published]

jobs:
  deploy-production:
    name: Deploy to Production
    runs-on: ubuntu-latest
    environment: production
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Build production documentation
        run: |
          cargo doc --all-features --no-deps --release

      - name: Optimize and deploy
        run: |
          # Optimize documentation for production
          find target/doc -name "*.html" -exec htmlmin {} {} \;
          find target/doc -name "*.css" -exec csso {} -o {} \;

          # Deploy to production CDN
          aws s3 sync target/doc s3://leptos-forms-docs/ --delete
          aws cloudfront create-invalidation --distribution-id ${{ secrets.CLOUDFRONT_DISTRIBUTION_ID }} --paths "/*"
        env:
          AWS_ACCESS_KEY_ID: ${{ secrets.AWS_ACCESS_KEY_ID }}
          AWS_SECRET_ACCESS_KEY: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
```

## 5. Monitoring and Observability

### 5.1 Build Monitoring

```yaml
# .github/workflows/monitoring.yml
name: Pipeline Monitoring

on:
  schedule:
    - cron: "0 */6 * * *" # Every 6 hours

jobs:
  health-check:
    name: Pipeline Health Check
    runs-on: ubuntu-latest
    steps:
      - name: Check recent builds
        run: |
          # Get recent workflow runs
          FAILED_RUNS=$(gh api repos/${{ github.repository }}/actions/runs \
            --jq '.workflow_runs[] | select(.created_at > (now - 86400 | strftime("%Y-%m-%dT%H:%M:%SZ")) and .conclusion == "failure") | .id' \
            | wc -l)

          if [ $FAILED_RUNS -gt 5 ]; then
            echo "⚠️ High failure rate: $FAILED_RUNS failures in last 24 hours"
            # Send alert to Slack
            curl -X POST -H 'Content-type: application/json' \
              --data "{\"text\":\"🚨 Leptos Forms CI: High failure rate ($FAILED_RUNS failures in 24h)\"}" \
              ${{ secrets.SLACK_WEBHOOK_URL }}
          fi
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

      - name: Check dependency freshness
        run: |
          # Check for outdated dependencies
          cargo install cargo-outdated
          OUTDATED=$(cargo outdated --root-deps-only --format json | jq '.dependencies | length')

          if [ $OUTDATED -gt 10 ]; then
            echo "⚠️ Many outdated dependencies: $OUTDATED"
            # Create issue for dependency updates
            gh issue create \
              --title "Dependency Update Required" \
              --body "Found $OUTDATED outdated root dependencies" \
              --label "maintenance"
          fi
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### 5.2 Performance Monitoring

```yaml
# .github/workflows/performance-monitoring.yml
name: Performance Monitoring

on:
  schedule:
    - cron: "0 3 * * *" # Daily at 3 AM

jobs:
  performance-regression:
    name: Performance Regression Check
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Run performance benchmarks
        run: |
          cargo bench --all-features -- --output-format json > current-benchmarks.json

      - name: Compare with baseline
        run: |
          # Download previous benchmark results
          gh release download performance-baseline --pattern "benchmarks.json"

          # Compare performance (simplified)
          python scripts/compare-benchmarks.py baseline-benchmarks.json current-benchmarks.json

      - name: Upload current benchmarks
        if: success()
        run: |
          gh release upload performance-baseline current-benchmarks.json --clobber
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## 6. Secrets and Security

### 6.1 Required Secrets

| Secret                  | Purpose                 | Scope            |
| ----------------------- | ----------------------- | ---------------- |
| `CRATES_IO_TOKEN`       | Publishing to crates.io | Release workflow |
| `GITHUB_TOKEN`          | GitHub API access       | All workflows    |
| `CODECOV_TOKEN`         | Coverage reporting      | CI workflow      |
| `DISCORD_WEBHOOK`       | Community notifications | Release workflow |
| `NETLIFY_AUTH_TOKEN`    | Staging deployment      | Deploy workflow  |
| `AWS_ACCESS_KEY_ID`     | Production deployment   | Deploy workflow  |
| `AWS_SECRET_ACCESS_KEY` | Production deployment   | Deploy workflow  |

### 6.2 Security Best Practices

- **Principle of Least Privilege**: Each secret has minimum required permissions
- **Environment Isolation**: Separate secrets for staging and production
- **Regular Rotation**: Secrets rotated quarterly or on security events
- **Audit Trail**: All secret usage logged and monitored

## 7. Troubleshooting Guide

### 7.1 Common Issues

#### Build Failures

```bash
# Debug build issues
cargo build --verbose 2>&1 | tee build.log

# Check toolchain
rustup show

# Clean rebuild
cargo clean && cargo build
```

#### Test Failures

```bash
# Run specific test with output
cargo test test_name -- --nocapture

# Run tests in single thread
cargo test -- --test-threads=1

# Debug test with backtrace
RUST_BACKTRACE=full cargo test
```

#### WASM Issues

```bash
# Check WASM target
rustup target list --installed | grep wasm32

# Install WASM target
rustup target add wasm32-unknown-unknown

# Debug WASM build
wasm-pack build --dev --target web --verbose
```

### 7.2 Pipeline Failure Recovery

1. **Identify failure point** from GitHub Actions logs
2. **Reproduce locally** using same commands
3. **Fix issue** and test locally
4. **Push fix** and monitor pipeline
5. **Document solution** for future reference

This comprehensive CI/CD pipeline ensures high-quality releases with automated testing, security scanning, performance monitoring, and seamless deployment to multiple environments.

---

**Document Control**

- **Created**: 2025-01-02
- **Last Modified**: 2025-01-02
- **Next Review**: Monthly during implementation
- **Version**: 1.0
- **Classification**: Technical Implementation
