import { test, expect } from "@playwright/test";

test.describe("Performance Testing", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/performance-test.html");
    await page.waitForSelector('[data-testid="perf-test-container"]', {
      timeout: 10000,
    });
  });

  test("should handle large forms efficiently", async ({ page }) => {
    // Generate form with 100 fields
    const generateButton = page.locator('[data-testid="generate-large-form"]');
    await generateButton.click();

    const startTime = Date.now();

    // Wait for all fields to be rendered
    await page.waitForSelector('[data-testid="field-99"]', { timeout: 10000 });

    const renderTime = Date.now() - startTime;
    console.log(`Large form render time: ${renderTime}ms`);

    // Should render within reasonable time (less than 2 seconds)
    expect(renderTime).toBeLessThan(2000);

    // Test field interaction performance
    const measureInteractionTime = async (fieldIndex: number) => {
      const field = page.locator(`[data-testid="field-${fieldIndex}"]`);
      const interactionStart = Date.now();

      await field.fill(`test value ${fieldIndex}`);
      await field.blur();

      // Wait for any validation or state updates
      await page.waitForTimeout(50);

      return Date.now() - interactionStart;
    };

    // Test interaction with multiple fields
    const interactionTimes = [];
    for (let i = 0; i < 10; i++) {
      const time = await measureInteractionTime(i * 10); // Every 10th field
      interactionTimes.push(time);
    }

    const avgInteractionTime =
      interactionTimes.reduce((a, b) => a + b, 0) / interactionTimes.length;
    console.log(`Average field interaction time: ${avgInteractionTime}ms`);

    // Field interactions should be fast (less than 100ms each)
    expect(avgInteractionTime).toBeLessThan(100);
  });

  test("should handle rapid user input efficiently", async ({ page }) => {
    const textField = page.locator('[data-testid="rapid-input-field"]');

    // Simulate rapid typing
    const rapidText = "This is a test of rapid typing performance";
    const typingStart = Date.now();

    // Type quickly without delays
    await textField.type(rapidText, { delay: 0 });

    const typingTime = Date.now() - typingStart;
    console.log(`Rapid typing time: ${typingTime}ms`);

    // Should handle rapid input without blocking (less than 500ms for 41 characters)
    expect(typingTime).toBeLessThan(500);

    // Field should contain all the typed text
    await expect(textField).toHaveValue(rapidText);

    // Test rapid backspacing
    const backspaceStart = Date.now();

    // Select all and delete
    await textField.selectText();
    await textField.press("Delete");

    const backspaceTime = Date.now() - backspaceStart;
    console.log(`Rapid deletion time: ${backspaceTime}ms`);

    expect(backspaceTime).toBeLessThan(100);
    await expect(textField).toHaveValue("");
  });

  test("should handle validation debouncing efficiently", async ({ page }) => {
    const emailField = page.locator('[data-testid="debounced-email-field"]');
    const validationCounter = page.locator(
      '[data-testid="validation-counter"]',
    );

    // Type email quickly
    await emailField.type("test@example.com", { delay: 50 });

    // Wait for debouncing period
    await page.waitForTimeout(500);

    // Check how many validations were triggered
    const validationCount = await validationCounter.textContent();
    const count = parseInt(validationCount || "0");

    console.log(`Validation triggered ${count} times`);

    // Should be debounced to reduce validation calls (ideally 1-2 times)
    expect(count).toBeLessThanOrEqual(3);
  });

  test("should handle memory efficiently with form arrays", async ({
    page,
  }) => {
    const addItemButton = page.locator('[data-testid="add-array-item"]');
    const removeItemButton = page.locator('[data-testid="remove-array-item"]');
    const itemCount = page.locator('[data-testid="array-item-count"]');

    // Add many items
    const itemsToAdd = 50;
    const addStart = Date.now();

    for (let i = 0; i < itemsToAdd; i++) {
      await addItemButton.click();
      if (i % 10 === 0) {
        // Check intermediate state
        await expect(itemCount).toContainText((i + 1).toString());
      }
    }

    const addTime = Date.now() - addStart;
    console.log(`Time to add ${itemsToAdd} items: ${addTime}ms`);

    // Should add items efficiently
    expect(addTime).toBeLessThan(5000);

    // Verify all items were added
    await expect(itemCount).toContainText(itemsToAdd.toString());

    // Remove all items
    const removeStart = Date.now();

    for (let i = 0; i < itemsToAdd; i++) {
      await removeItemButton.click();
    }

    const removeTime = Date.now() - removeStart;
    console.log(`Time to remove ${itemsToAdd} items: ${removeTime}ms`);

    // Should remove items efficiently
    expect(removeTime).toBeLessThan(3000);

    // Verify all items were removed
    await expect(itemCount).toContainText("0");
  });

  test("should handle form submission efficiently", async ({ page }) => {
    // Fill out a complex form
    const fillStart = Date.now();

    await page.fill('[data-testid="name-input"]', "Performance Test User");
    await page.fill('[data-testid="email-input"]', "test@performance.com");
    await page.fill('[data-testid="age-input"]', "30");
    await page.selectOption('[data-testid="country-select"]', "US");
    await page.check('[data-testid="newsletter-checkbox"]');

    // Fill array field
    for (let i = 0; i < 5; i++) {
      await page.fill('[data-testid="tag-input"]', `tag-${i}`);
      await page.click('[data-testid="add-tag-button"]');
    }

    const fillTime = Date.now() - fillStart;
    console.log(`Form filling time: ${fillTime}ms`);

    // Form filling should be fast
    expect(fillTime).toBeLessThan(2000);

    // Submit form and measure time
    const submitStart = Date.now();

    await page.click('[data-testid="submit-button"]');

    // Wait for submission to complete
    await page.waitForSelector('[data-testid="success-message"]', {
      timeout: 5000,
    });

    const submitTime = Date.now() - submitStart;
    console.log(`Form submission time: ${submitTime}ms`);

    // Submission should be reasonably fast
    expect(submitTime).toBeLessThan(3000);
  });

  test("should handle concurrent validations efficiently", async ({ page }) => {
    // Open multiple forms in tabs or windows
    const context = page.context();
    const pages = [page];

    // Create additional pages
    for (let i = 1; i < 3; i++) {
      const newPage = await context.newPage();
      await newPage.goto("/performance-test.html");
      await newPage.waitForSelector('[data-testid="perf-test-container"]');
      pages.push(newPage);
    }

    // Perform concurrent operations
    const concurrentStart = Date.now();

    const operations = pages.map(async (p, index) => {
      await p.fill('[data-testid="name-input"]', `User ${index}`);
      await p.fill('[data-testid="email-input"]', `user${index}@example.com`);
      await p.fill('[data-testid="age-input"]', (20 + index).toString());
      return p.click('[data-testid="validate-button"]');
    });

    await Promise.all(operations);

    const concurrentTime = Date.now() - concurrentStart;
    console.log(`Concurrent validation time: ${concurrentTime}ms`);

    // Concurrent operations should not significantly slow down individual operations
    expect(concurrentTime).toBeLessThan(2000);

    // Verify all validations succeeded
    for (const p of pages) {
      await expect(
        p.locator('[data-testid="validation-success"]'),
      ).toBeVisible();
    }

    // Clean up additional pages
    for (let i = 1; i < pages.length; i++) {
      await pages[i].close();
    }
  });

  test("should handle scroll performance with large forms", async ({
    page,
  }) => {
    // Generate a very large form
    await page.click('[data-testid="generate-huge-form"]');
    await page.waitForSelector('[data-testid="field-199"]', { timeout: 15000 });

    // Measure scroll performance
    const scrollStart = Date.now();

    // Scroll to bottom
    await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));

    // Wait for scroll to settle
    await page.waitForTimeout(100);

    const scrollToBottomTime = Date.now() - scrollStart;

    // Scroll to middle
    const scrollMiddleStart = Date.now();
    await page.evaluate(() =>
      window.scrollTo(0, document.body.scrollHeight / 2),
    );
    await page.waitForTimeout(100);
    const scrollToMiddleTime = Date.now() - scrollMiddleStart;

    // Scroll to top
    const scrollTopStart = Date.now();
    await page.evaluate(() => window.scrollTo(0, 0));
    await page.waitForTimeout(100);
    const scrollToTopTime = Date.now() - scrollTopStart;

    console.log(
      `Scroll performance - Bottom: ${scrollToBottomTime}ms, Middle: ${scrollToMiddleTime}ms, Top: ${scrollToTopTime}ms`,
    );

    // Scrolling should be smooth and fast
    expect(scrollToBottomTime).toBeLessThan(500);
    expect(scrollToMiddleTime).toBeLessThan(300);
    expect(scrollToTopTime).toBeLessThan(300);
  });

  test("should maintain performance during error states", async ({ page }) => {
    // Create form with many validation errors
    const errorStart = Date.now();

    // Fill form with invalid data
    for (let i = 0; i < 20; i++) {
      await page.fill(`[data-testid="error-field-${i}"]`, "invalid");
    }

    // Trigger validation
    await page.click('[data-testid="validate-all-button"]');

    // Wait for all errors to appear
    await page.waitForSelector('[data-testid="error-field-19-error"]', {
      timeout: 5000,
    });

    const errorTime = Date.now() - errorStart;
    console.log(`Error state creation time: ${errorTime}ms`);

    // Should handle many errors efficiently
    expect(errorTime).toBeLessThan(3000);

    // Clear all errors
    const clearStart = Date.now();

    for (let i = 0; i < 20; i++) {
      await page.fill(`[data-testid="error-field-${i}"]`, `valid-value-${i}`);
    }

    await page.click('[data-testid="validate-all-button"]');

    // Wait for all errors to clear
    await expect(
      page.locator('[data-testid="error-field-19-error"]'),
    ).not.toBeVisible();

    const clearTime = Date.now() - clearStart;
    console.log(`Error clearing time: ${clearTime}ms`);

    // Should clear errors efficiently
    expect(clearTime).toBeLessThan(2000);
  });

  test("should handle memory cleanup properly", async ({ page }) => {
    // This test would ideally check memory usage but we'll simulate
    // by creating and destroying many form elements

    const iterations = 10;
    const timings = [];

    for (let iter = 0; iter < iterations; iter++) {
      const iterStart = Date.now();

      // Create form
      await page.click('[data-testid="create-dynamic-form"]');
      await page.waitForSelector('[data-testid="dynamic-form"]');

      // Fill form
      await page.fill('[data-testid="dynamic-name"]', `Test ${iter}`);
      await page.fill(
        '[data-testid="dynamic-email"]',
        `test${iter}@example.com`,
      );

      // Destroy form
      await page.click('[data-testid="destroy-dynamic-form"]');
      await expect(
        page.locator('[data-testid="dynamic-form"]'),
      ).not.toBeVisible();

      const iterTime = Date.now() - iterStart;
      timings.push(iterTime);

      console.log(`Iteration ${iter + 1}: ${iterTime}ms`);
    }

    const avgTime = timings.reduce((a, b) => a + b, 0) / timings.length;
    const maxTime = Math.max(...timings);
    const minTime = Math.min(...timings);

    console.log(
      `Average iteration time: ${avgTime}ms, Min: ${minTime}ms, Max: ${maxTime}ms`,
    );

    // Performance should remain consistent (max time shouldn't be more than 2x average)
    expect(maxTime).toBeLessThan(avgTime * 2);

    // Overall performance should be reasonable
    expect(avgTime).toBeLessThan(1000);
  });
});
