# End-to-End Testing with Playwright

This directory contains comprehensive end-to-end tests for the Leptos Forms library using Playwright.

## 🚀 Quick Start

### Prerequisites

- Node.js 20+ and pnpm installed
- Project dependencies installed (`pnpm install`)
- Playwright browsers installed (`pnpm run test:e2e:install`)

### Running Tests

```bash
# Run all E2E tests
pnpm run test:e2e

# Run tests with UI mode (interactive)
pnpm run test:e2e:ui

# Run tests in headed mode (see browser)
pnpm run test:e2e:headed

# Run tests in debug mode
pnpm run test:e2e:debug

# Show test report
pnpm run test:e2e:show-report
```

## 📁 Test Structure

```
tests/e2e/
├── basic-form.spec.ts          # Basic form example tests
├── complex-form.spec.ts        # Complex multi-step form tests
├── form-components.spec.ts     # Individual component tests
├── utils/
│   ├── test-helpers.ts        # Test utility functions
│   └── test-helpers.spec.ts   # Tests for utility functions
├── global-setup.ts             # Global test setup
├── global-teardown.ts          # Global test cleanup
└── README.md                   # This file
```

## 🧪 Test Categories

### 1. Basic Form Tests (`basic-form.spec.ts`)

- Form rendering and field display
- Validation error handling
- Form submission and success
- Field focus and blur events
- Keyboard navigation
- Accessibility features
- Mobile responsiveness
- Form state persistence

### 2. Complex Form Tests (`complex-form.spec.ts`)

- Multi-step form navigation
- Field arrays (dynamic fields)
- Conditional field rendering
- Complex validation rules
- File uploads
- Form persistence across steps
- Cross-step data validation

### 3. Component Tests (`form-components.spec.ts`)

- Individual component rendering
- Component-specific functionality
- State management
- Event handling
- Accessibility compliance
- Mobile interactions

## 🛠️ Test Utilities

The `utils/test-helpers.ts` file provides common utility functions:

### Form Interaction Helpers

- `fillField()` - Fill and verify field values
- `checkCheckbox()` / `uncheckCheckbox()` - Handle checkboxes
- `selectOption()` - Handle dropdown selections
- `uploadFile()` - Handle file uploads
- `submitForm()` - Submit forms with proper waiting

### Navigation Helpers

- `goToNextStep()` / `goToPreviousStep()` - Multi-step form navigation
- `navigateWithTab()` / `navigateWithShiftTab()` - Keyboard navigation
- `pressEnter()` / `pressEscape()` - Common key presses

### Validation Helpers

- `expectValidationErrors()` - Verify error count
- `expectNoValidationErrors()` - Verify no errors
- `expectFieldValue()` - Verify field values
- `expectVisible()` / `expectNotVisible()` - Visibility checks

### Viewport Helpers

- `setMobileViewport()` - Set mobile dimensions
- `setDesktopViewport()` - Set desktop dimensions

### Common Selectors

The `FormSelectors` constant provides consistent selectors:

```typescript
import { FormSelectors } from "./utils/test-helpers";

// Use consistent selectors
await page.click(FormSelectors.SUBMIT_BUTTON);
await page.fill(FormSelectors.TEXT_INPUT, "value");
```

### Test Data

The `TestData` constant provides common test values:

```typescript
import { TestData } from "./utils/test-helpers";

// Use predefined test data
for (const email of TestData.VALID_EMAILS) {
  await page.fill(FormSelectors.EMAIL_INPUT, email);
}
```

## 🔧 Configuration

### Playwright Config (`playwright.config.ts`)

- **Test Directory**: `./tests/e2e`
- **Browsers**: Chrome, Firefox, Safari, Mobile Chrome, Mobile Safari
- **Base URL**: `http://localhost:3000`
- **Web Server**: Automatically starts dev server before tests
- **Reporters**: HTML, JSON, JUnit
- **Screenshots**: On failure
- **Videos**: On failure
- **Traces**: On first retry

### Global Setup/Teardown

- **Setup**: Verifies application is ready for testing
- **Teardown**: Cleans up test environment

## 📱 Browser Support

Tests run against multiple browsers and viewports:

### Desktop Browsers

- **Chromium** - Chrome/Edge compatibility
- **Firefox** - Firefox-specific behavior
- **WebKit** - Safari compatibility

### Mobile Browsers

- **Mobile Chrome** - Android Chrome
- **Mobile Safari** - iOS Safari

### Viewport Sizes

- **Desktop**: 1280x720
- **Mobile**: 375x667 (iPhone 12)
- **Tablet**: 768x1024 (iPad)

## 🎯 Test Patterns

### Page Object Model

Each test file focuses on a specific page or component:

```typescript
test.describe("Basic Form Example", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await page.waitForSelector("form");
  });

  // Tests here...
});
```

### Data-Driven Testing

Use test data arrays for comprehensive coverage:

```typescript
for (const email of TestData.VALID_EMAILS) {
  test(`should accept valid email: ${email}`, async ({ page }) => {
    await page.fill('[name="email"]', email);
    await page.click('[type="submit"]');
    await expect(page.locator(".error")).not.toBeVisible();
  });
}
```

### Accessibility Testing

Verify ARIA labels, keyboard navigation, and screen reader support:

```typescript
test("should be accessible with screen readers", async ({ page }) => {
  await expect(page.locator("form")).toHaveAttribute("aria-label", /form/i);
  await expect(page.locator('input[name="username"]')).toHaveAttribute(
    "aria-label",
    /username/i,
  );
});
```

## 🚨 Common Issues & Solutions

### Form Not Ready

```typescript
// Wait for form to be fully loaded
await page.waitForSelector("form", { timeout: 10000 });
await page.waitForLoadState("networkidle");
```

### Validation Timing

```typescript
// Wait for validation to complete
await page.waitForTimeout(100);
await expect(page.locator(".error-message")).toBeVisible();
```

### Mobile Testing

```typescript
// Set mobile viewport before testing
await page.setViewportSize({ width: 375, height: 667 });
```

### File Uploads

```typescript
// Create test file and upload
const testFilePath = "tests/e2e/test-file.txt";
await page.locator('input[type="file"]').setInputFiles(testFilePath);
```

## 📊 Test Reports

After running tests, view detailed reports:

```bash
# Open HTML report
pnpm run test:e2e:show-report

# View test results
open test-results/
```

Reports include:

- Test execution timeline
- Screenshots on failure
- Video recordings
- Trace files for debugging
- Performance metrics

## 🔍 Debugging

### Debug Mode

```bash
pnpm run test:e2e:debug
```

### Code Generation

```bash
pnpm run test:e2e:codegen
```

### Screenshots

```typescript
// Take screenshots during tests
await TestHelpers.takeScreenshot(page, "form-state");
```

### Logging

```typescript
// Log page state for debugging
await TestHelpers.logPageState(page);
```

## 🚀 CI/CD Integration

Tests are automatically included in CI pipeline:

```bash
pnpm run ci  # Runs lint, unit tests, WASM tests, and E2E tests
```

## 📚 Best Practices

1. **Use Test Helpers**: Leverage utility functions for common operations
2. **Consistent Selectors**: Use `data-testid` attributes for reliable selection
3. **Wait for Elements**: Always wait for elements before interacting
4. **Test Accessibility**: Include accessibility checks in every test
5. **Mobile First**: Test mobile interactions and responsive behavior
6. **Error Handling**: Test both success and failure scenarios
7. **Performance**: Monitor test execution time and optimize slow tests

## 🤝 Contributing

When adding new tests:

1. Follow the existing test structure
2. Use the provided test utilities
3. Include accessibility testing
4. Test both desktop and mobile
5. Add appropriate error handling
6. Document any new test patterns

## 📖 Additional Resources

- [Playwright Documentation](https://playwright.dev/)
- [Playwright Testing Best Practices](https://playwright.dev/docs/best-practices)
- [Accessibility Testing Guide](https://playwright.dev/docs/accessibility-testing)
- [Mobile Testing Guide](https://playwright.dev/docs/mobile)
