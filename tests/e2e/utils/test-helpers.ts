import { Page, expect } from "@playwright/test";

/**
 * Utility functions for E2E testing
 */
export class TestHelpers {
  /**
   * Wait for form to be ready and interactive
   */
  static async waitForFormReady(page: Page, formSelector = "form") {
    await page.waitForSelector(formSelector, { timeout: 10000 });
    await page.waitForLoadState("networkidle");
  }

  /**
   * Fill a form field and verify the value
   */
  static async fillField(page: Page, selector: string, value: string) {
    await page.fill(selector, value);
    await expect(page.locator(selector)).toHaveValue(value);
  }

  /**
   * Check a checkbox and verify it's checked
   */
  static async checkCheckbox(page: Page, selector: string) {
    await page.check(selector);
    await expect(page.locator(selector)).toBeChecked();
  }

  /**
   * Uncheck a checkbox and verify it's unchecked
   */
  static async uncheckCheckbox(page: Page, selector: string) {
    await page.uncheck(selector);
    await expect(page.locator(selector)).not.toBeChecked();
  }

  /**
   * Select an option from a dropdown and verify the selection
   */
  static async selectOption(page: Page, selector: string, value: string) {
    await page.selectOption(selector, value);
    await expect(page.locator(selector)).toHaveValue(value);
  }

  /**
   * Upload a file and verify the upload
   */
  static async uploadFile(page: Page, selector: string, filePath: string) {
    await page.locator(selector).setInputFiles(filePath);
    // Wait a moment for the upload to process
    await page.waitForTimeout(100);
  }

  /**
   * Submit a form and wait for completion
   */
  static async submitForm(
    page: Page,
    submitSelector = 'button[type="submit"]',
  ) {
    await page.click(submitSelector);
    await page.waitForTimeout(1000); // Wait for submission to complete
  }

  /**
   * Navigate to next step in a multi-step form
   */
  static async goToNextStep(page: Page) {
    await page.click('[data-testid="next-button"]');
    await page.waitForTimeout(500); // Wait for step transition
  }

  /**
   * Navigate to previous step in a multi-step form
   */
  static async goToPreviousStep(page: Page) {
    await page.click('[data-testid="prev-button"]');
    await page.waitForTimeout(500); // Wait for step transition
  }

  /**
   * Verify that validation errors are shown
   */
  static async expectValidationErrors(page: Page, count: number) {
    const errorMessages = page.locator(
      '.error-message, [data-testid="error"], .validation-error',
    );
    await expect(errorMessages).toHaveCount(count);
  }

  /**
   * Verify that no validation errors are shown
   */
  static async expectNoValidationErrors(page: Page) {
    const errorMessages = page.locator(
      '.error-message, [data-testid="error"], .validation-error',
    );
    await expect(errorMessages).toHaveCount(0);
  }

  /**
   * Verify form field has expected value
   */
  static async expectFieldValue(
    page: Page,
    selector: string,
    expectedValue: string,
  ) {
    await expect(page.locator(selector)).toHaveValue(expectedValue);
  }

  /**
   * Verify form field is empty
   */
  static async expectFieldEmpty(page: Page, selector: string) {
    await expect(page.locator(selector)).toHaveValue("");
  }

  /**
   * Verify checkbox is checked
   */
  static async expectCheckboxChecked(page: Page, selector: string) {
    await expect(page.locator(selector)).toBeChecked();
  }

  /**
   * Verify checkbox is unchecked
   */
  static async expectCheckboxUnchecked(page: Page, selector: string) {
    await expect(page.locator(selector)).not.toBeChecked();
  }

  /**
   * Verify element is visible
   */
  static async expectVisible(page: Page, selector: string) {
    await expect(page.locator(selector)).toBeVisible();
  }

  /**
   * Verify element is not visible
   */
  static async expectNotVisible(page: Page, selector: string) {
    await expect(page.locator(selector)).not.toBeVisible();
  }

  /**
   * Verify element has focus
   */
  static async expectFocused(page: Page, selector: string) {
    await expect(page.locator(selector)).toBeFocused();
  }

  /**
   * Verify element does not have focus
   */
  static async expectNotFocused(page: Page, selector: string) {
    await expect(page.locator(selector)).not.toBeFocused();
  }

  /**
   * Verify element has specific class
   */
  static async expectHasClass(page: Page, selector: string, className: string) {
    await expect(page.locator(selector)).toHaveClass(new RegExp(className));
  }

  /**
   * Verify element does not have specific class
   */
  static async expectNotHasClass(
    page: Page,
    selector: string,
    className: string,
  ) {
    await expect(page.locator(selector)).not.toHaveClass(new RegExp(className));
  }

  /**
   * Wait for success message
   */
  static async waitForSuccess(page: Page, timeout = 5000) {
    await page.waitForSelector('[data-testid="success-message"]', { timeout });
  }

  /**
   * Wait for error message
   */
  static async waitForError(page: Page, timeout = 5000) {
    await page.waitForSelector('[data-testid="error-message"]', { timeout });
  }

  /**
   * Set mobile viewport for mobile testing
   */
  static async setMobileViewport(page: Page) {
    await page.setViewportSize({ width: 375, height: 667 });
  }

  /**
   * Set desktop viewport for desktop testing
   */
  static async setDesktopViewport(page: Page) {
    await page.setViewportSize({ width: 1280, height: 720 });
  }

  /**
   * Navigate with keyboard (Tab navigation)
   */
  static async navigateWithTab(page: Page, times = 1) {
    for (let i = 0; i < times; i++) {
      await page.keyboard.press("Tab");
    }
  }

  /**
   * Navigate backwards with keyboard (Shift+Tab navigation)
   */
  static async navigateWithShiftTab(page: Page, times = 1) {
    for (let i = 0; i < times; i++) {
      await page.keyboard.press("Shift+Tab");
    }
  }

  /**
   * Press Enter key
   */
  static async pressEnter(page: Page) {
    await page.keyboard.press("Enter");
  }

  /**
   * Press Escape key
   */
  static async pressEscape(page: Page) {
    await page.keyboard.press("Escape");
  }

  /**
   * Wait for a specific timeout
   */
  static async wait(ms: number) {
    await new Promise((resolve) => setTimeout(resolve, ms));
  }

  /**
   * Take a screenshot for debugging
   */
  static async takeScreenshot(page: Page, name: string) {
    await page.screenshot({ path: `test-results/${name}.png` });
  }

  /**
   * Log current page state for debugging
   */
  static async logPageState(page: Page) {
    const url = page.url();
    const title = await page.title();
    console.log(`Current page: ${title} (${url})`);
  }
}

/**
 * Common form field selectors
 */
export const FormSelectors = {
  // Basic form fields
  TEXT_INPUT: '[data-testid="text-input"]',
  NUMBER_INPUT: '[data-testid="number-input"]',
  CHECKBOX_INPUT: '[data-testid="checkbox-input"]',
  SELECT_INPUT: '[data-testid="select-input"]',
  TEXTAREA_INPUT: '[data-testid="textarea-input"]',
  RADIO_INPUT: '[data-testid="radio-input"]',
  FILE_INPUT: '[data-testid="file-input"]',

  // Form actions
  SUBMIT_BUTTON: '[data-testid="submit-button"]',
  RESET_BUTTON: '[data-testid="reset-button"]',
  NEXT_BUTTON: '[data-testid="next-button"]',
  PREV_BUTTON: '[data-testid="prev-button"]',

  // Validation
  VALIDATION_ERROR: '[data-testid="validation-error"]',
  ERROR_MESSAGE: '[data-testid="error-message"]',
  SUCCESS_MESSAGE: '[data-testid="success-message"]',

  // Form state
  FORM: "form",
  FORM_WIZARD: '[data-testid="form-wizard"]',
  STEP_INDICATOR: '[data-testid="step-indicator"]',

  // Field arrays
  ADD_FIELD: '[data-testid="add-field"]',
  REMOVE_FIELD: '[data-testid="remove-field"]',

  // Conditional fields
  SHOW_CONDITIONAL: '[data-testid="show-conditional"]',
  CONDITIONAL_FIELD: '[data-testid="conditional-field"]',
} as const;

/**
 * Common test data
 */
export const TestData = {
  VALID_EMAILS: [
    "test@example.com",
    "user.name@domain.co.uk",
    "user+tag@example.org",
  ],
  INVALID_EMAILS: [
    "invalid",
    "@domain.com",
    "user@",
    "user@domain",
    "user name@domain.com",
  ],
  VALID_URLS: [
    "https://example.com",
    "http://domain.org/path",
    "https://sub.domain.com:8080/path?query=1",
  ],
  INVALID_URLS: ["invalid", "ftp://example.com", "//example.com"],
} as const;
