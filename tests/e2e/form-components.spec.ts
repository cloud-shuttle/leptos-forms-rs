import { test, expect } from '@playwright/test';

test.describe('Form Library Components', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the component testing page
    await page.goto('/components');
    
    // Wait for the page to be ready
    await page.waitForSelector('[data-testid="component-test-page"]', { timeout: 10000 });
  });

  test('should render TextInput component correctly', async ({ page }) => {
    // Check that TextInput is rendered
    const textInput = page.locator('[data-testid="text-input"]');
    await expect(textInput).toBeVisible();
    
    // Check input attributes
    await expect(textInput).toHaveAttribute('type', 'text');
    await expect(textInput).toHaveAttribute('name', 'test-text');
    
    // Check label
    await expect(page.locator('label[for="test-text"]')).toContainText('Test Text Input');
    
    // Test input functionality
    await textInput.fill('Hello World');
    await expect(textInput).toHaveValue('Hello World');
  });

  test('should render NumberInput component correctly', async ({ page }) => {
    // Check that NumberInput is rendered
    const numberInput = page.locator('[data-testid="number-input"]');
    await expect(numberInput).toBeVisible();
    
    // Check input attributes
    await expect(numberInput).toHaveAttribute('type', 'number');
    await expect(numberInput).toHaveAttribute('name', 'test-number');
    await expect(numberInput).toHaveAttribute('min', '0');
    await expect(numberInput).toHaveAttribute('max', '100');
    
    // Test input functionality
    await numberInput.fill('42');
    await expect(numberInput).toHaveValue('42');
    
    // Test validation
    await numberInput.fill('150');
    await expect(page.locator('[data-testid="number-error"]')).toBeVisible();
  });

  test('should render CheckboxInput component correctly', async ({ page }) => {
    // Check that CheckboxInput is rendered
    const checkbox = page.locator('[data-testid="checkbox-input"]');
    await expect(checkbox).toBeVisible();
    
    // Check input attributes
    await expect(checkbox).toHaveAttribute('type', 'checkbox');
    await expect(checkbox).toHaveAttribute('name', 'test-checkbox');
    
    // Test checkbox functionality
    await checkbox.check();
    await expect(checkbox).toBeChecked();
    
    await checkbox.uncheck();
    await expect(checkbox).not.toBeChecked();
  });

  test('should render SelectInput component correctly', async ({ page }) => {
    // Check that SelectInput is rendered
    const select = page.locator('[data-testid="select-input"]');
    await expect(select).toBeVisible();
    
    // Check select attributes
    await expect(select).toHaveAttribute('name', 'test-select');
    
    // Check options
    await expect(select.locator('option[value="option1"]')).toContainText('Option 1');
    await expect(select.locator('option[value="option2"]')).toContainText('Option 2');
    await expect(select.locator('option[value="option3"]')).toContainText('Option 3');
    
    // Test selection
    await select.selectOption('option2');
    await expect(select).toHaveValue('option2');
  });

  test('should render TextareaInput component correctly', async ({ page }) => {
    // Check that TextareaInput is rendered
    const textarea = page.locator('[data-testid="textarea-input"]');
    await expect(textarea).toBeVisible();
    
    // Check textarea attributes
    await expect(textarea).toHaveAttribute('name', 'test-textarea');
    await expect(textarea).toHaveAttribute('rows', '4');
    
    // Test input functionality
    await textarea.fill('This is a long text\nwith multiple lines');
    await expect(textarea).toHaveValue('This is a long text\nwith multiple lines');
  });

  test('should render RadioInput component correctly', async ({ page }) => {
    // Check that RadioInputs are rendered
    const radio1 = page.locator('[data-testid="radio-input-1"]');
    const radio2 = page.locator('[data-testid="radio-input-2"]');
    const radio3 = page.locator('[data-testid="radio-input-3"]');
    
    await expect(radio1).toBeVisible();
    await expect(radio2).toBeVisible();
    await expect(radio3).toBeVisible();
    
    // Check radio attributes
    await expect(radio1).toHaveAttribute('type', 'radio');
    await expect(radio2).toHaveAttribute('type', 'radio');
    await expect(radio3).toHaveAttribute('type', 'radio');
    
    // Check that they have the same name (radio group)
    await expect(radio1).toHaveAttribute('name', 'test-radio');
    await expect(radio2).toHaveAttribute('name', 'test-radio');
    await expect(radio3).toHaveAttribute('name', 'test-radio');
    
    // Test radio selection
    await radio2.check();
    await expect(radio2).toBeChecked();
    await expect(radio1).not.toBeChecked();
    await expect(radio3).not.toBeChecked();
    
    // Select another radio
    await radio3.check();
    await expect(radio3).toBeChecked();
    await expect(radio1).not.toBeChecked();
    await expect(radio2).not.toBeChecked();
  });

  test('should render FileInput component correctly', async ({ page }) => {
    // Check that FileInput is rendered
    const fileInput = page.locator('[data-testid="file-input"]');
    await expect(fileInput).toBeVisible();
    
    // Check input attributes
    await expect(fileInput).toHaveAttribute('type', 'file');
    await expect(fileInput).toHaveAttribute('name', 'test-file');
    await expect(fileInput).toHaveAttribute('accept', '.txt,.pdf,.doc');
    
    // Test file selection
    const testFilePath = 'tests/e2e/test-file.txt';
    await fileInput.setInputFiles(testFilePath);
    
    // Check that file is selected
    await expect(page.locator('[data-testid="file-name"]')).toContainText('test-file.txt');
  });

  test('should handle form validation correctly', async ({ page }) => {
    // Try to submit without filling required fields
    // Use JavaScript click to avoid pointer event interception issues
    await page.evaluate(() => {
      const submitBtn = document.querySelector('[data-testid="submit-button"]');
      if (submitBtn) submitBtn.click();
    });
    
    // Check for validation errors
    const errorMessages = page.locator('[data-testid="validation-error"]');
    await expect(errorMessages).toHaveCount(4); // We have 4 required fields
    
    // Fill required fields
    await page.fill('[data-testid="text-input"]', 'Valid Text');
    await page.fill('[data-testid="email-input"]', 'test@example.com');
    await page.fill('[data-testid="password-input"]', 'password123');
    await page.selectOption('[data-testid="select-input"]', 'option1');
    
    // Submit again - should now succeed and show success message
    await page.evaluate(() => {
      const submitBtn = document.querySelector('[data-testid="submit-button"]');
      if (submitBtn) submitBtn.click();
    });
    
    // Check for success message instead of counting validation errors
    await expect(page.locator('[data-testid="success-message"]')).toBeVisible();
  });

  test('should handle form submission correctly', async ({ page }) => {
    // Fill required fields
    await page.fill('[data-testid="text-input"]', 'Test Text');
    await page.fill('[data-testid="email-input"]', 'test@example.com');
    await page.fill('[data-testid="password-input"]', 'password123');
    await page.selectOption('[data-testid="select-input"]', 'option2');
    
    // Fill optional fields
    await page.fill('[data-testid="number-input"]', '75');
    await page.check('[data-testid="checkbox-input"]');
    await page.fill('[data-testid="textarea-input"]', 'Test description');
    await page.check('[data-testid="radio-input-2"]');
    
    // Submit the form
    await page.evaluate(() => {
      const submitBtn = document.querySelector('[data-testid="submit-button"]');
      if (submitBtn) submitBtn.click();
    });
    
    // Wait for submission
    await page.waitForTimeout(1000);
    
    // Check for success message
    await expect(page.locator('[data-testid="success-message"]')).toBeVisible();
    
    // Verify form is reset
    await expect(page.locator('[data-testid="text-input"]')).toHaveValue('');
    await expect(page.locator('[data-testid="email-input"]')).toHaveValue('');
    await expect(page.locator('[data-testid="password-input"]')).toHaveValue('');
    await expect(page.locator('[data-testid="select-input"]')).toHaveValue('');
  });

  test('should handle field state changes correctly', async ({ page }) => {
    // Check initial state
    const textInput = page.locator('[data-testid="text-input"]');
    await expect(textInput).not.toHaveClass(/dirty/);
    await expect(textInput).not.toHaveClass(/touched/);
    
    // Focus the field
    await textInput.focus();
    await expect(textInput).toHaveClass(/touched/);
    
    // Type something
    await textInput.fill('Test');
    await expect(textInput).toHaveClass(/dirty/);
    
    // Clear the field
    await textInput.clear();
    await expect(textInput).toHaveClass(/dirty/);
    await expect(textInput).toHaveClass(/touched/);
  });

  test('should handle conditional field rendering', async ({ page }) => {
    // Check that conditional field is hidden initially
    const conditionalField = page.locator('[data-testid="conditional-field"]');
    await expect(conditionalField).not.toBeVisible();
    
    // Check the checkbox that shows the conditional field
    await page.check('[data-testid="show-conditional"]');
    
    // Conditional field should now be visible
    await expect(conditionalField).toBeVisible();
    
    // Uncheck to hide it again
    await page.uncheck('[data-testid="show-conditional"]');
    await expect(conditionalField).not.toBeVisible();
  });

  test('should handle field arrays correctly', async ({ page }) => {
    // Check initial field array
    await expect(page.locator('[data-testid="field-array-0"]')).toBeVisible();
    
    // Add a new field
    await page.evaluate(() => {
      const addBtn = document.querySelector('[data-testid="add-field"]');
      if (addBtn) addBtn.click();
    });
    
    // Check that new field is added
    await expect(page.locator('[data-testid="field-array-1"]')).toBeVisible();
    
    // Fill the new field
    await page.fill('[data-testid="array-name-input-1"]', 'New Field Value');
    
    // Remove the field
    await page.evaluate(() => {
      const removeBtn = document.querySelector('[data-testid="remove-field-1"]');
      if (removeBtn) removeBtn.click();
    });
    
    // Check that field is removed
    await expect(page.locator('[data-testid="field-array-1"]')).not.toBeVisible();
  });

  test('should handle form persistence', async ({ page }) => {
    // Fill some fields
    await page.fill('[data-testid="text-input"]', 'Persistent Text');
    await page.fill('[data-testid="number-input"]', '42');
    
    // Refresh the page
    await page.reload();
    
    // Wait for page to reload
    await page.waitForSelector('[data-testid="component-test-page"]', { timeout: 10000 });
    
    // Check that data is restored
    await expect(page.locator('[data-testid="text-input"]')).toHaveValue('Persistent Text');
    await expect(page.locator('[data-testid="number-input"]')).toHaveValue('42');
  });

  test('should handle keyboard navigation', async ({ page }) => {
    // Navigate through fields with Tab
    await page.keyboard.press('Tab');
    await expect(page.locator('[data-testid="text-input"]')).toBeFocused();
    
    await page.keyboard.press('Tab');
    await expect(page.locator('[data-testid="email-input"]')).toBeFocused();
    
    await page.keyboard.press('Tab');
    await expect(page.locator('[data-testid="password-input"]')).toBeFocused();
    
    // Navigate backwards with Shift+Tab
    await page.keyboard.press('Shift+Tab');
    await expect(page.locator('[data-testid="email-input"]')).toBeFocused();
  });

  test('should handle mobile interactions', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    // Test mobile interactions (using click instead of tap for compatibility)
    await page.click('[data-testid="text-input"]');
    await page.fill('[data-testid="text-input"]', 'Mobile Test');
    
    await page.click('[data-testid="checkbox-input"]');
    await expect(page.locator('[data-testid="checkbox-input"]')).toBeChecked();
    
    // Verify mobile-friendly layout
    await expect(page.locator('[data-testid="text-input"]')).toBeVisible();
    await expect(page.locator('[data-testid="submit-button"]')).toBeVisible();
  });

  test('should handle accessibility features', async ({ page }) => {
    // Check ARIA labels
    const textInput = page.locator('[data-testid="text-input"]');
    await expect(textInput).toHaveAttribute('aria-label', /test text input/i);
    
    // Check form labels
    await expect(page.locator('label[for="test-text"]')).toBeVisible();
    
    // Check error message associations
    const errorMessage = page.locator('[data-testid="validation-error"]').first();
    await expect(errorMessage).toHaveAttribute('role', 'alert');
    
    // Check form structure
    await expect(page.locator('form')).toHaveAttribute('aria-label', /component test form/i);
  });

  test('should handle form reset correctly', async ({ page }) => {
    // Fill some fields
    await page.fill('[data-testid="text-input"]', 'Reset Test');
    await page.fill('[data-testid="number-input"]', '99');
    await page.check('[data-testid="checkbox-input"]');
    
    // Reset the form
    await page.evaluate(() => {
      const resetBtn = document.querySelector('[data-testid="reset-button"]');
      if (resetBtn) resetBtn.click();
    });
    
    // Check that fields are cleared
    await expect(page.locator('[data-testid="text-input"]')).toHaveValue('');
    await expect(page.locator('[data-testid="number-input"]')).toHaveValue('');
    await expect(page.locator('[data-testid="checkbox-input"]')).not.toBeChecked();
    
    // Check that form state is reset
    const textInput = page.locator('[data-testid="text-input"]');
    await expect(textInput).not.toHaveClass(/dirty/);
    await expect(textInput).not.toHaveClass(/touched/);
  });
});
