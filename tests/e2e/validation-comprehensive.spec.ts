import { test, expect } from "@playwright/test";

test.describe("Comprehensive Form Validation", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/complex-form.html");
    await page.waitForSelector('[data-testid="complex-form"]', {
      timeout: 10000,
    });
  });

  test("should validate all field types correctly", async ({ page }) => {
    // Test text field validation
    const nameField = page.locator('[data-testid="name-input"]');
    await nameField.fill("A"); // Too short
    await nameField.blur();
    await expect(page.locator('[data-testid="name-error"]')).toBeVisible();

    await nameField.fill("Valid Name");
    await nameField.blur();
    await expect(page.locator('[data-testid="name-error"]')).not.toBeVisible();

    // Test email validation
    const emailField = page.locator('[data-testid="email-input"]');
    await emailField.fill("invalid-email");
    await emailField.blur();
    await expect(page.locator('[data-testid="email-error"]')).toBeVisible();

    await emailField.fill("valid@example.com");
    await emailField.blur();
    await expect(page.locator('[data-testid="email-error"]')).not.toBeVisible();

    // Test number validation
    const ageField = page.locator('[data-testid="age-input"]');
    await ageField.fill("-1"); // Below minimum
    await ageField.blur();
    await expect(page.locator('[data-testid="age-error"]')).toBeVisible();

    await ageField.fill("25");
    await ageField.blur();
    await expect(page.locator('[data-testid="age-error"]')).not.toBeVisible();
  });

  test("should handle real-time validation", async ({ page }) => {
    const emailField = page.locator('[data-testid="email-input"]');

    // Type invalid email character by character
    await emailField.type("i");
    await page.waitForTimeout(100);
    // Should not show error yet (debounced)

    await emailField.type("nvalid");
    await page.waitForTimeout(300);
    // Should show error after debounce
    await expect(page.locator('[data-testid="email-error"]')).toBeVisible();

    // Complete to valid email
    await emailField.type("@example.com");
    await page.waitForTimeout(300);
    await expect(page.locator('[data-testid="email-error"]')).not.toBeVisible();
  });

  test("should validate cross-field dependencies", async ({ page }) => {
    // Enable conditional field
    const triggerCheckbox = page.locator(
      '[data-testid="enable-conditional-checkbox"]',
    );
    await triggerCheckbox.check();

    // Dependent field should now be required
    const conditionalField = page.locator('[data-testid="conditional-input"]');
    await expect(conditionalField).toBeVisible();
    await expect(conditionalField).toHaveAttribute("required");

    // Try to submit without filling dependent field
    await page.click('[data-testid="submit-button"]');
    await expect(
      page.locator('[data-testid="conditional-error"]'),
    ).toBeVisible();

    // Fill dependent field
    await conditionalField.fill("Required value");
    await expect(
      page.locator('[data-testid="conditional-error"]'),
    ).not.toBeVisible();
  });

  test("should handle async validation", async ({ page }) => {
    const usernameField = page.locator('[data-testid="username-input"]');

    // Type username that needs async validation
    await usernameField.fill("existinguser");
    await usernameField.blur();

    // Should show loading indicator
    await expect(
      page.locator('[data-testid="username-validating"]'),
    ).toBeVisible();

    // Wait for async validation to complete
    await page.waitForSelector('[data-testid="username-validating"]', {
      state: "hidden",
      timeout: 5000,
    });

    // Should show error if username exists
    await expect(page.locator('[data-testid="username-error"]')).toBeVisible();

    // Try with available username
    await usernameField.fill("availableuser");
    await usernameField.blur();
    await page.waitForSelector('[data-testid="username-validating"]', {
      state: "hidden",
      timeout: 5000,
    });
    await expect(
      page.locator('[data-testid="username-error"]'),
    ).not.toBeVisible();
  });

  test("should validate file upload fields", async ({ page }) => {
    const fileInput = page.locator('[data-testid="avatar-upload"]');

    // Test invalid file type
    await fileInput.setInputFiles({
      name: "test.txt",
      mimeType: "text/plain",
      buffer: Buffer.from("test content"),
    });

    await expect(page.locator('[data-testid="avatar-error"]')).toBeVisible();
    await expect(page.locator('[data-testid="avatar-error"]')).toContainText(
      "file type",
    );

    // Test valid file type
    await fileInput.setInputFiles({
      name: "test.jpg",
      mimeType: "image/jpeg",
      buffer: Buffer.from("fake image content"),
    });

    await expect(
      page.locator('[data-testid="avatar-error"]'),
    ).not.toBeVisible();
    await expect(page.locator('[data-testid="avatar-preview"]')).toBeVisible();
  });

  test("should validate array fields", async ({ page }) => {
    // Add items to array field
    const addTagButton = page.locator('[data-testid="add-tag-button"]');
    const tagInput = page.locator('[data-testid="tag-input"]');

    // Add first tag
    await tagInput.fill("javascript");
    await addTagButton.click();
    await expect(page.locator('[data-testid="tag-item-0"]')).toBeVisible();

    // Add second tag
    await tagInput.fill("rust");
    await addTagButton.click();
    await expect(page.locator('[data-testid="tag-item-1"]')).toBeVisible();

    // Try to add duplicate tag
    await tagInput.fill("javascript");
    await addTagButton.click();
    await expect(page.locator('[data-testid="tags-error"]')).toBeVisible();
    await expect(page.locator('[data-testid="tags-error"]')).toContainText(
      "duplicate",
    );

    // Remove a tag
    await page.click('[data-testid="remove-tag-0"]');
    await expect(page.locator('[data-testid="tag-item-0"]')).not.toBeVisible();

    // Should now allow adding the previously duplicate tag
    await tagInput.fill("javascript");
    await addTagButton.click();
    await expect(page.locator('[data-testid="tags-error"]')).not.toBeVisible();
  });

  test("should validate complex nested forms", async ({ page }) => {
    // Test nested address form
    const addAddressButton = page.locator('[data-testid="add-address-button"]');
    await addAddressButton.click();

    const addressForm = page.locator('[data-testid="address-form-0"]');
    await expect(addressForm).toBeVisible();

    // Fill invalid address data
    await addressForm.locator('[data-testid="street-input"]').fill(""); // Required
    await addressForm.locator('[data-testid="city-input"]').fill(""); // Required
    await addressForm.locator('[data-testid="zip-input"]').fill("invalid"); // Invalid format

    // Try to submit
    await page.click('[data-testid="submit-button"]');

    // Should show nested validation errors
    await expect(
      addressForm.locator('[data-testid="street-error"]'),
    ).toBeVisible();
    await expect(
      addressForm.locator('[data-testid="city-error"]'),
    ).toBeVisible();
    await expect(
      addressForm.locator('[data-testid="zip-error"]'),
    ).toBeVisible();

    // Fill valid data
    await addressForm
      .locator('[data-testid="street-input"]')
      .fill("123 Main St");
    await addressForm.locator('[data-testid="city-input"]').fill("Anytown");
    await addressForm.locator('[data-testid="zip-input"]').fill("12345");

    // Errors should clear
    await expect(
      addressForm.locator('[data-testid="street-error"]'),
    ).not.toBeVisible();
    await expect(
      addressForm.locator('[data-testid="city-error"]'),
    ).not.toBeVisible();
    await expect(
      addressForm.locator('[data-testid="zip-error"]'),
    ).not.toBeVisible();
  });

  test("should handle validation error recovery", async ({ page }) => {
    // Fill form with multiple errors
    await page.fill('[data-testid="name-input"]', "A"); // Too short
    await page.fill('[data-testid="email-input"]', "invalid"); // Invalid format
    await page.fill('[data-testid="age-input"]', "-1"); // Below minimum

    // Submit to trigger all validations
    await page.click('[data-testid="submit-button"]');

    // Should show all errors
    await expect(page.locator('[data-testid="name-error"]')).toBeVisible();
    await expect(page.locator('[data-testid="email-error"]')).toBeVisible();
    await expect(page.locator('[data-testid="age-error"]')).toBeVisible();

    // Fix errors one by one and verify they clear
    await page.fill('[data-testid="name-input"]', "Valid Name");
    await page.locator('[data-testid="name-input"]').blur();
    await expect(page.locator('[data-testid="name-error"]')).not.toBeVisible();

    await page.fill('[data-testid="email-input"]', "valid@example.com");
    await page.locator('[data-testid="email-input"]').blur();
    await expect(page.locator('[data-testid="email-error"]')).not.toBeVisible();

    await page.fill('[data-testid="age-input"]', "25");
    await page.locator('[data-testid="age-input"]').blur();
    await expect(page.locator('[data-testid="age-error"]')).not.toBeVisible();

    // Form should now be submittable
    await page.click('[data-testid="submit-button"]');
    await expect(page.locator('[data-testid="success-message"]')).toBeVisible();
  });

  test("should persist validation state during navigation", async ({
    page,
  }) => {
    // Fill form partially with errors
    await page.fill('[data-testid="name-input"]', "A"); // Invalid
    await page.fill('[data-testid="email-input"]', "valid@example.com"); // Valid

    await page.locator('[data-testid="name-input"]').blur();
    await expect(page.locator('[data-testid="name-error"]')).toBeVisible();

    // Navigate away and back
    await page.goto("/basic-form.html");
    await page.goBack();

    // Validation state should be preserved
    await expect(page.locator('[data-testid="name-input"]')).toHaveValue("A");
    await expect(page.locator('[data-testid="email-input"]')).toHaveValue(
      "valid@example.com",
    );
    await expect(page.locator('[data-testid="name-error"]')).toBeVisible();
  });

  test("should handle validation with dynamic field changes", async ({
    page,
  }) => {
    // Select different field type from dropdown
    const fieldTypeSelect = page.locator('[data-testid="field-type-select"]');
    await fieldTypeSelect.selectOption("email");

    // Dynamic field should change to email input
    const dynamicField = page.locator('[data-testid="dynamic-field"]');
    await expect(dynamicField).toHaveAttribute("type", "email");

    // Test email validation
    await dynamicField.fill("invalid-email");
    await dynamicField.blur();
    await expect(
      page.locator('[data-testid="dynamic-field-error"]'),
    ).toBeVisible();

    // Change field type again
    await fieldTypeSelect.selectOption("number");
    await expect(dynamicField).toHaveAttribute("type", "number");

    // Previous error should be cleared
    await expect(
      page.locator('[data-testid="dynamic-field-error"]'),
    ).not.toBeVisible();

    // Test number validation
    await dynamicField.fill("-10"); // Below minimum
    await dynamicField.blur();
    await expect(
      page.locator('[data-testid="dynamic-field-error"]'),
    ).toBeVisible();
  });
});
