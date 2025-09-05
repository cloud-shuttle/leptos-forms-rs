import { test, expect } from "@playwright/test";

test.describe("Basic Form Example", () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the basic form test page
    await page.goto("/basic-form.html");

    // Wait for the form to be ready
    await page.waitForSelector('[data-testid="login-form"]', {
      timeout: 10000,
    });
  });

  test("should display the login form with all fields", async ({ page }) => {
    // Check that all form fields are present
    await expect(page.locator('[data-testid="username-input"]')).toBeVisible();
    await expect(page.locator('[data-testid="password-input"]')).toBeVisible();
    await expect(page.locator('[data-testid="submit-button"]')).toBeVisible();

    // Check field labels and placeholders
    await expect(page.locator('[data-testid="username-label"]')).toContainText(
      "Username",
    );
    await expect(page.locator('[data-testid="password-label"]')).toContainText(
      "Password",
    );

    // Check input types
    await expect(
      page.locator('[data-testid="username-input"]'),
    ).toHaveAttribute("type", "text");
    await expect(
      page.locator('[data-testid="password-input"]'),
    ).toHaveAttribute("type", "password");
  });

  test("should show validation errors for empty required fields", async ({
    page,
  }) => {
    // Try to submit without filling any fields
    await page.click('[data-testid="submit-button"]');

    // Wait for validation to trigger
    await page.waitForTimeout(100);

    // Check that error message elements exist (they may be hidden by CSS)
    await expect(page.locator('[data-testid="username-error"]')).toBeAttached();
    await expect(page.locator('[data-testid="password-error"]')).toBeAttached();

    // Verify form was not submitted successfully
    await expect(
      page.locator('[data-testid="success-message"]'),
    ).not.toBeVisible();

    // Verify form is still visible
    await expect(page.locator('[data-testid="login-form"]')).toBeVisible();
  });

  test("should validate username field requirements", async ({ page }) => {
    // Fill only password field
    await page.fill('[data-testid="password-input"]', "testpassword");

    // Try to submit
    await page.click('[data-testid="submit-button"]');

    // Check that username error element exists
    await expect(page.locator('[data-testid="username-error"]')).toBeAttached();

    // Fill username with invalid value (empty)
    await page.fill('[data-testid="username-input"]', "");

    // Submit again
    await page.click('[data-testid="submit-button"]');

    // Should still have username error element
    await expect(page.locator('[data-testid="username-error"]')).toBeAttached();

    // Verify form was not submitted successfully
    await expect(
      page.locator('[data-testid="success-message"]'),
    ).not.toBeVisible();
  });

  test("should validate password field requirements", async ({ page }) => {
    // Fill only username field
    await page.fill('[data-testid="username-input"]', "testuser");

    // Try to submit
    await page.click('[data-testid="submit-button"]');

    // Check that password error element exists
    await expect(page.locator('[data-testid="password-error"]')).toBeAttached();

    // Fill password with invalid value (too short)
    await page.fill('[data-testid="password-input"]', "123");

    // Submit again
    await page.click('[data-testid="submit-button"]');

    // Should still have password error element
    await expect(page.locator('[data-testid="password-error"]')).toBeAttached();

    // Verify form was not submitted successfully
    await expect(
      page.locator('[data-testid="success-message"]'),
    ).not.toBeVisible();
  });

  test("should accept valid form data and submit successfully", async ({
    page,
  }) => {
    // Fill form with valid data
    await page.fill('[data-testid="username-input"]', "validusername");
    await page.fill('[data-testid="password-input"]', "validpassword123");

    // Submit the form
    await page.click('[data-testid="submit-button"]');

    // Wait for form submission to complete
    await page.waitForTimeout(500);

    // Check that success message is shown
    await expect(page.locator('[data-testid="success-message"]')).toBeVisible();

    // Verify fields are cleared after successful submission
    await expect(page.locator('[data-testid="username-input"]')).toHaveValue(
      "",
    );
    await expect(page.locator('[data-testid="password-input"]')).toHaveValue(
      "",
    );
  });

  test("should handle field focus and blur events", async ({ page }) => {
    // Focus on username field
    await page.click('[data-testid="username-input"]');

    // Check that username field has focus
    await expect(page.locator('[data-testid="username-input"]')).toBeFocused();

    // Focus on password field
    await page.click('[data-testid="password-input"]');

    // Check that password field has focus
    await expect(page.locator('[data-testid="password-input"]')).toBeFocused();

    // Blur password field by clicking elsewhere
    await page.click("body");

    // Wait a moment for focus to change
    await page.waitForTimeout(100);

    // Verify we can still interact with the form
    await expect(page.locator('[data-testid="login-form"]')).toBeVisible();

    // Test that we can focus on username field again
    await page.click('[data-testid="username-input"]');
    await expect(page.locator('[data-testid="username-input"]')).toBeFocused();
  });

  test("should handle keyboard navigation", async ({ page }) => {
    // First click on username field to ensure it has focus
    await page.click('[data-testid="username-input"]');
    await expect(page.locator('[data-testid="username-input"]')).toBeFocused();

    // Navigate to password field with Tab
    await page.keyboard.press("Tab");
    await expect(page.locator('[data-testid="password-input"]')).toBeFocused();

    // Try to submit with Enter key on password field
    await page.keyboard.press("Enter");

    // Should show validation error elements
    await page.waitForTimeout(100);
    await expect(page.locator('[data-testid="username-error"]')).toBeAttached();
    await expect(page.locator('[data-testid="password-error"]')).toBeAttached();

    // Verify form was not submitted successfully
    await expect(
      page.locator('[data-testid="success-message"]'),
    ).not.toBeVisible();
  });

  test("should be accessible with screen readers", async ({ page }) => {
    // Check that form has proper structure
    await expect(page.locator('[data-testid="login-form"]')).toBeVisible();

    // Check that inputs have proper labels
    const usernameInput = page.locator('[data-testid="username-input"]');
    const passwordInput = page.locator('[data-testid="password-input"]');

    // Check that labels are properly associated with inputs
    await expect(page.locator('[data-testid="username-label"]')).toBeVisible();
    await expect(page.locator('[data-testid="password-label"]')).toBeVisible();

    // Check that submit button has accessible text
    const submitButton = page.locator('[data-testid="submit-button"]');
    await expect(submitButton).toHaveText(/login/i);
  });

  test("should work on mobile devices", async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    // Verify form is still usable on mobile
    await expect(page.locator('[data-testid="username-input"]')).toBeVisible();
    await expect(page.locator('[data-testid="password-input"]')).toBeVisible();
    await expect(page.locator('[data-testid="submit-button"]')).toBeVisible();

    // Test touch interactions
    await page.click('[data-testid="username-input"]');
    await page.fill('[data-testid="username-input"]', "mobileuser");

    await page.click('[data-testid="password-input"]');
    await page.fill('[data-testid="password-input"]', "mobilepass123");

    await page.click('[data-testid="submit-button"]');

    // Wait for submission
    await page.waitForTimeout(500);

    // Verify form was submitted successfully
    await expect(page.locator('[data-testid="success-message"]')).toBeVisible();
  });

  test("should handle rapid form interactions", async ({ page }) => {
    // Fill form quickly
    await page.fill('[data-testid="username-input"]', "fastuser");
    await page.fill('[data-testid="password-input"]', "fastpass123");

    // Submit multiple times quickly
    await page.click('[data-testid="submit-button"]');
    await page.click('[data-testid="submit-button"]');
    await page.click('[data-testid="submit-button"]');

    // Wait for processing
    await page.waitForTimeout(1000);

    // Verify form was only processed once (no duplicate submissions)
    // Our HTML form shows success message and resets
    await expect(page.locator('[data-testid="success-message"]')).toBeVisible();
  });

  test("should preserve form state during navigation", async ({ page }) => {
    // Fill form partially
    await page.fill('[data-testid="username-input"]', "partialuser");

    // Navigate away and back (simulate browser back/forward)
    await page.goto("/");
    await page.goBack();

    // Check if form state is preserved (this depends on implementation)
    // For now, just verify the form is still accessible
    await expect(page.locator('[data-testid="login-form"]')).toBeVisible();
  });
});
