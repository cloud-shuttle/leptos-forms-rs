import { test, expect } from '@playwright/test';

test.describe('Smoke Tests', () => {
  test('should load the test suite index', async ({ page }) => {
    // Navigate to the test suite
    await page.goto('/');
    
    // Check that the page loads
    await expect(page).toHaveTitle(/Leptos Forms Test Suite/);
    
    // Check that we can see the test suite content
    await expect(page.locator('h1')).toBeVisible();
    await expect(page.locator('h1')).toHaveText(/🧪 Leptos Forms Test Suite/);
  });

  test('should have test suite navigation', async ({ page }) => {
    // Navigate to the test suite
    await page.goto('/');
    
    // Wait for content to load
    await page.waitForLoadState('networkidle');
    
    // Check for test suite navigation
    await expect(page.locator('.test-links')).toBeVisible();
    await expect(page.locator('a[href="basic-form.html"]')).toBeVisible();
    await expect(page.locator('a[href="complex-form.html"]')).toBeVisible();
    await expect(page.locator('a[href="components.html"]')).toBeVisible();
  });

  test('should handle basic page interactions', async ({ page }) => {
    // Navigate to the test suite
    await page.goto('/');
    
    // Test basic page functionality
    await page.mouse.move(100, 100);
    await page.keyboard.press('Tab');
    
    // Verify page is still responsive and navigation works
    await expect(page.locator('body')).toBeVisible();
    
    // Test navigation to basic form
    await page.click('a[href="basic-form.html"]');
    await expect(page).toHaveURL(/basic-form\.html$/);
    await expect(page.locator('h1')).toHaveText(/Login Form Example/);
  });
});
