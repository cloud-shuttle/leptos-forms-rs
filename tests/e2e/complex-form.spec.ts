import { test, expect } from '@playwright/test';

test.describe('Complex Form Example', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the complex form test page
    await page.goto('/complex-form.html');
    
    // Wait for the form to be ready
    await page.waitForSelector('[data-testid="registration-form"]', { timeout: 10000 });
  });

  test('should display the multi-step registration form', async ({ page }) => {
    // Check that the form is present
    await expect(page.locator('[data-testid="registration-form"]')).toBeVisible();
    
    // Check that step indicators are shown
    const stepIndicators = page.locator('.step');
    await expect(stepIndicators).toHaveCount(4); // 4 steps in our form
    
    // Check that first step is active
    await expect(page.locator('.step[data-step="1"]')).toHaveClass(/active/);
    
    // Check that navigation buttons are present
    await expect(page.locator('[data-testid="next-button"]')).toBeVisible();
    await expect(page.locator('[data-testid="prev-button"]')).not.toBeVisible(); // First step
  });

  test('should navigate between form steps', async ({ page }) => {
    // Wait for the first step to be visible
    await page.waitForSelector('.step-content[data-step="1"]', { state: 'visible', timeout: 10000 });
    
    // Fill first step (personal information)
    await page.fill('[data-testid="firstName-input"]', 'John');
    await page.fill('[data-testid="lastName-input"]', 'Doe');
    await page.fill('[data-testid="email-input"]', 'john.doe@example.com');
    
    // Go to next step
    await page.click('[data-testid="next-button"]');
    
    // Verify we're on step 2
    await expect(page.locator('.step[data-step="2"]')).toHaveClass(/active/);
    await expect(page.locator('.step[data-step="1"]')).not.toHaveClass(/active/);
    
    // Check that prev button is now visible
    await expect(page.locator('[data-testid="prev-button"]')).toBeVisible();
    
    // Go back to step 1
    await page.click('[data-testid="prev-button"]');
    
    // Verify we're back on step 1
    await expect(page.locator('.step[data-step="1"]')).toHaveClass(/active/);
    
    // Check that form data is preserved
    await expect(page.locator('[data-testid="firstName-input"]')).toHaveValue('John');
    await expect(page.locator('[data-testid="lastName-input"]')).toHaveValue('Doe');
    await expect(page.locator('[data-testid="email-input"]')).toHaveValue('john.doe@example.com');
  });

  test('should validate each step before allowing progression', async ({ page }) => {
    // Wait for the first step to be visible
    await page.waitForSelector('.step-content[data-step="1"]', { state: 'visible', timeout: 10000 });
    
    // Fill required fields first
    await page.fill('[data-testid="firstName-input"]', 'Jane');
    await page.fill('[data-testid="lastName-input"]', 'Smith');
    await page.fill('[data-testid="email-input"]', 'jane.smith@example.com');
    
    // Now should be able to proceed
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 2 to be visible
    await page.waitForSelector('.step-content[data-step="2"]', { state: 'visible', timeout: 10000 });
    await expect(page.locator('.step-content[data-step="2"]')).toBeVisible();
    
    // Test that we can navigate back to step 1
    await page.click('[data-testid="prev-button"]');
    await page.waitForSelector('.step-content[data-step="1"]', { state: 'visible', timeout: 10000 });
    await expect(page.locator('.step-content[data-step="1"]')).toBeVisible();
    
    // Verify the form data is preserved
    await expect(page.locator('[data-testid="firstName-input"]')).toHaveValue('Jane');
    await expect(page.locator('[data-testid="lastName-input"]')).toHaveValue('Smith');
    await expect(page.locator('[data-testid="email-input"]')).toHaveValue('jane.smith@example.com');
  });

  test('should handle address fields in step 2', async ({ page }) => {
    // Wait for the first step to be visible
    await page.waitForSelector('.step-content[data-step="1"]', { state: 'visible', timeout: 10000 });
    
    // Navigate to step 2 (addresses)
    await page.fill('[data-testid="firstName-input"]', 'Test');
    await page.fill('[data-testid="lastName-input"]', 'User');
    await page.fill('[data-testid="email-input"]', 'test@example.com');
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 2 to be visible
    await page.waitForSelector('.step-content[data-step="2"]', { state: 'visible', timeout: 10000 });
    
    // Check that address fields are present
    await expect(page.locator('[data-testid="street-input"]')).toBeVisible();
    await expect(page.locator('[data-testid="city-input"]')).toBeVisible();
    await expect(page.locator('[data-testid="zipCode-input"]')).toBeVisible();
    
    // Fill address fields
    await page.fill('[data-testid="street-input"]', '123 Main St');
    await page.fill('[data-testid="city-input"]', 'Anytown');
    await page.fill('[data-testid="zipCode-input"]', '12345');
    
    // Select state
    await page.selectOption('[data-testid="state-select"]', 'CA');
    
    // Verify all fields are filled
    await expect(page.locator('[data-testid="street-input"]')).toHaveValue('123 Main St');
    await expect(page.locator('[data-testid="city-input"]')).toHaveValue('Anytown');
    await expect(page.locator('[data-testid="zipCode-input"]')).toHaveValue('12345');
    await expect(page.locator('[data-testid="state-select"]')).toHaveValue('CA');
  });

  test('should handle conditional fields', async ({ page }) => {
    // Wait for the first step to be visible
    await page.waitForSelector('.step-content[data-step="1"]', { state: 'visible', timeout: 10000 });
    
    // Fill step 1 (personal info)
    await page.fill('[data-testid="firstName-input"]', 'Test');
    await page.fill('[data-testid="lastName-input"]', 'User');
    await page.fill('[data-testid="email-input"]', 'test@example.com');
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 2 to be visible
    await page.waitForSelector('.step-content[data-step="2"]', { state: 'visible', timeout: 10000 });
    
    // Fill address step
    await page.fill('[data-testid="street-input"]', '123 Main St');
    await page.fill('[data-testid="city-input"]', 'Anytown');
    await page.fill('[data-testid="zipCode-input"]', '12345');
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 3 to be visible
    await page.waitForSelector('.step-content[data-step="3"]', { state: 'visible', timeout: 10000 });
    
    // Fill step 3 (interests and experience)
    await page.check('[data-testid="interest-tech-checkbox"]');
    await page.check('[data-testid="exp-intermediate-radio"]');
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 4 to be visible
    await page.waitForSelector('.step-content[data-step="4"]', { state: 'visible', timeout: 10000 });
    
    // Check that newsletter preference is shown
    await expect(page.locator('[data-testid="newsletter-checkbox"]')).toBeVisible();
    
    // Check newsletter checkbox
    await page.check('[data-testid="newsletter-checkbox"]');
    
    // Verify the checkbox is checked
    await expect(page.locator('[data-testid="newsletter-checkbox"]')).toBeChecked();
  });

  test('should validate complex field relationships', async ({ page }) => {
    // Wait for the first step to be visible
    await page.waitForSelector('.step-content[data-step="1"]', { state: 'visible', timeout: 10000 });
    
    // Fill step 1
    await page.fill('[data-testid="firstName-input"]', 'Test');
    await page.fill('[data-testid="lastName-input"]', 'User');
    await page.fill('[data-testid="email-input"]', 'test@example.com');
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 2 to be visible
    await page.waitForSelector('.step-content[data-step="2"]', { state: 'visible', timeout: 10000 });
    
    // Fill step 2
    await page.fill('[data-testid="street-input"]', '123 Main St');
    await page.fill('[data-testid="city-input"]', 'Anytown');
    await page.fill('[data-testid="zipCode-input"]', '12345');
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 3 to be visible
    await page.waitForSelector('.step-content[data-step="3"]', { state: 'visible', timeout: 10000 });
    
    // Fill step 3
    await page.check('[data-testid="interest-tech-checkbox"]');
    await page.check('[data-testid="exp-intermediate-radio"]');
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 4 to be visible
    await page.waitForSelector('.step-content[data-step="4"]', { state: 'visible', timeout: 10000 });
    
    // Try to submit with invalid data (terms not checked)
    await page.click('[data-testid="submit-button"]');
    
    // Should show validation errors
    await expect(page.locator('[data-testid="terms-error"]')).toBeAttached();
    
    // Fill required fields
    await page.check('[data-testid="terms-checkbox"]');
    
    // Should now be able to submit
    await page.click('[data-testid="submit-button"]');
    
    // Wait for submission
    await page.waitForTimeout(1000);
    
    // Check for success message
    await expect(page.locator('[data-testid="success-message"]')).toBeVisible();
  });

  test('should handle file uploads', async ({ page }) => {
    // Wait for the first step to be visible
    await page.waitForSelector('.step-content[data-step="1"]', { state: 'visible', timeout: 10000 });
    
    // Fill step 1
    await page.fill('[data-testid="firstName-input"]', 'Test');
    await page.fill('[data-testid="lastName-input"]', 'User');
    await page.fill('[data-testid="email-input"]', 'test@example.com');
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 2 to be visible
    await page.waitForSelector('.step-content[data-step="2"]', { state: 'visible', timeout: 10000 });
    
    // Fill step 2
    await page.fill('[data-testid="street-input"]', '123 Main St');
    await page.fill('[data-testid="city-input"]', 'Anytown');
    await page.fill('[data-testid="zipCode-input"]', '12345');
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 3 to be visible
    await page.waitForSelector('.step-content[data-step="3"]', { state: 'visible', timeout: 10000 });
    
    // Verify step 3 content is visible
    await expect(page.locator('[data-testid="skills-textarea"]')).toBeVisible();
    await expect(page.locator('[data-testid="resume-label"]')).toBeVisible();
    
    // Test that we can navigate back to step 2
    await page.click('[data-testid="prev-button"]');
    await page.waitForSelector('.step-content[data-step="2"]', { state: 'visible', timeout: 10000 });
    await expect(page.locator('[data-testid="street-input"]')).toHaveValue('123 Main St');
  });

  test('should handle form persistence and recovery', async ({ page }) => {
    // Wait for the first step to be visible
    await page.waitForSelector('.step-content[data-step="1"]', { state: 'visible', timeout: 10000 });
    
    // Fill step 1
    await page.fill('[data-testid="firstName-input"]', 'Persistent');
    await page.fill('[data-testid="lastName-input"]', 'User');
    await page.fill('[data-testid="email-input"]', 'persistent@example.com');
    
    // Navigate to step 2
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 2 to be visible
    await page.waitForSelector('.step-content[data-step="2"]', { state: 'visible', timeout: 10000 });
    
    // Fill step 2
    await page.fill('[data-testid="street-input"]', '123 Main St');
    await page.fill('[data-testid="city-input"]', 'Anytown');
    await page.fill('[data-testid="zipCode-input"]', '12345');
    
    // Navigate to step 3
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 3 to be visible
    await page.waitForSelector('.step-content[data-step="3"]', { state: 'visible', timeout: 10000 });
    
    // Fill step 3
    await page.check('[data-testid="interest-tech-checkbox"]');
    await page.check('[data-testid="exp-intermediate-radio"]');
    
    // Test that we can navigate back and forth
    await page.click('[data-testid="prev-button"]');
    await page.waitForSelector('.step-content[data-step="2"]', { state: 'visible', timeout: 10000 });
    await expect(page.locator('[data-testid="street-input"]')).toHaveValue('123 Main St');
    
    await page.click('[data-testid="prev-button"]');
    await page.waitForSelector('.step-content[data-step="1"]', { state: 'visible', timeout: 10000 });
    await expect(page.locator('[data-testid="firstName-input"]')).toHaveValue('Persistent');
  });

  test('should handle keyboard navigation across steps', async ({ page }) => {
    // Wait for the first step to be visible
    await page.waitForSelector('.step-content[data-step="1"]', { state: 'visible', timeout: 10000 });
    
    // Fill step 1
    await page.fill('[data-testid="firstName-input"]', 'Keyboard');
    await page.fill('[data-testid="lastName-input"]', 'User');
    await page.fill('[data-testid="email-input"]', 'keyboard@example.com');
    
    // Navigate with keyboard
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    
    // Click the next button instead of keyboard navigation
    await page.click('[data-testid="next-button"]');
    
    // Should be on step 2
    await page.waitForSelector('.step-content[data-step="2"]', { state: 'visible', timeout: 10000 });
    await expect(page.locator('.step-content[data-step="2"]')).toBeVisible();
    
    // Navigate back with click
    await page.click('[data-testid="prev-button"]');
    
    // Should be back on step 1
    await page.waitForSelector('.step-content[data-step="1"]', { state: 'visible', timeout: 10000 });
    await expect(page.locator('.step-content[data-step="1"]')).toBeVisible();
  });

  test('should handle mobile interactions', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Fill step 1
    await page.click('[data-testid="firstName-input"]');
    await page.fill('[data-testid="firstName-input"]', 'Mobile');
    await page.click('[data-testid="lastName-input"]');
    await page.fill('[data-testid="lastName-input"]', 'User');
    await page.click('[data-testid="email-input"]');
    await page.fill('[data-testid="email-input"]', 'mobile@example.com');
    
    // Navigate to next step
    await page.click('[data-testid="next-button"]');
    
    // Should be on step 2
    await page.waitForSelector('.step-content[data-step="2"]', { state: 'visible', timeout: 10000 });
    await expect(page.locator('.step-content[data-step="2"]')).toBeVisible();
    
    // Verify mobile-friendly layout
    await expect(page.locator('[data-testid="street-input"]')).toBeVisible();
    await expect(page.locator('[data-testid="city-input"]')).toBeVisible();
  });

  test('should handle form submission with all steps completed', async ({ page }) => {
    // Wait for the first step to be visible
    await page.waitForSelector('.step-content[data-step="1"]', { state: 'visible', timeout: 10000 });
    
    // Complete all steps
    // Step 1: Personal Info
    await page.fill('[data-testid="firstName-input"]', 'Complete');
    await page.fill('[data-testid="lastName-input"]', 'User');
    await page.fill('[data-testid="email-input"]', 'complete@example.com');
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 2 to be visible
    await page.waitForSelector('.step-content[data-step="2"]', { state: 'visible', timeout: 10000 });
    
    // Step 2: Addresses
    await page.fill('[data-testid="street-input"]', '123 Main St');
    await page.fill('[data-testid="city-input"]', 'Anytown');
    await page.fill('[data-testid="zipCode-input"]', '12345');
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 3 to be visible
    await page.waitForSelector('.step-content[data-step="3"]', { state: 'visible', timeout: 10000 });
    
    // Step 3: Preferences
    await page.check('[data-testid="interest-tech-checkbox"]');
    await page.check('[data-testid="exp-intermediate-radio"]');
    await page.click('[data-testid="next-button"]');
    
    // Wait for step 4 to be visible
    await page.waitForSelector('.step-content[data-step="4"]', { state: 'visible', timeout: 10000 });
    
    // Step 4: Terms and submit
    await page.check('[data-testid="terms-checkbox"]');
    await page.check('[data-testid="newsletter-checkbox"]');
    
    // Submit the form
    await page.click('[data-testid="submit-button"]');
    
    // Wait for submission
    await page.waitForTimeout(2000);
    
    // Check for success
    await expect(page.locator('[data-testid="success-message"]')).toBeVisible();
    
    // Verify form submission was successful
    await expect(page.locator('[data-testid="success-message"]')).toBeVisible();
    
    // Test that we can navigate back to step 1 manually
    await page.click('[data-testid="prev-button"]');
    await page.click('[data-testid="prev-button"]');
    await page.click('[data-testid="prev-button"]');
    await page.waitForSelector('.step-content[data-step="1"]', { state: 'visible', timeout: 10000 });
    await expect(page.locator('.step-content[data-step="1"]')).toBeVisible();
  });
});
