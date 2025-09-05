import { test, expect } from "@playwright/test";
import { TestHelpers, FormSelectors, TestData } from "./test-helpers";

test.describe("Test Helpers", () => {
  test("should provide common form selectors", () => {
    // Check that all selectors are defined
    expect(FormSelectors.TEXT_INPUT).toBe('[data-testid="text-input"]');
    expect(FormSelectors.NUMBER_INPUT).toBe('[data-testid="number-input"]');
    expect(FormSelectors.CHECKBOX_INPUT).toBe('[data-testid="checkbox-input"]');
    expect(FormSelectors.SELECT_INPUT).toBe('[data-testid="select-input"]');
    expect(FormSelectors.TEXTAREA_INPUT).toBe('[data-testid="textarea-input"]');
    expect(FormSelectors.RADIO_INPUT).toBe('[data-testid="radio-input"]');
    expect(FormSelectors.FILE_INPUT).toBe('[data-testid="file-input"]');

    expect(FormSelectors.SUBMIT_BUTTON).toBe('[data-testid="submit-button"]');
    expect(FormSelectors.RESET_BUTTON).toBe('[data-testid="reset-button"]');
    expect(FormSelectors.NEXT_BUTTON).toBe('[data-testid="next-button"]');
    expect(FormSelectors.PREV_BUTTON).toBe('[data-testid="prev-button"]');

    expect(FormSelectors.VALIDATION_ERROR).toBe(
      '[data-testid="validation-error"]',
    );
    expect(FormSelectors.ERROR_MESSAGE).toBe('[data-testid="error-message"]');
    expect(FormSelectors.SUCCESS_MESSAGE).toBe(
      '[data-testid="success-message"]',
    );

    expect(FormSelectors.FORM).toBe("form");
    expect(FormSelectors.FORM_WIZARD).toBe('[data-testid="form-wizard"]');
    expect(FormSelectors.STEP_INDICATOR).toBe('[data-testid="step-indicator"]');

    expect(FormSelectors.ADD_FIELD).toBe('[data-testid="add-field"]');
    expect(FormSelectors.REMOVE_FIELD).toBe('[data-testid="remove-field"]');

    expect(FormSelectors.SHOW_CONDITIONAL).toBe(
      '[data-testid="show-conditional"]',
    );
    expect(FormSelectors.CONDITIONAL_FIELD).toBe(
      '[data-testid="conditional-field"]',
    );
  });

  test("should provide test data constants", () => {
    // Check that test data is defined
    expect(TestData.VALID_EMAILS).toHaveLength(3);
    expect(TestData.VALID_EMAILS).toContain("test@example.com");
    expect(TestData.VALID_EMAILS).toContain("user.name@domain.co.uk");
    expect(TestData.VALID_EMAILS).toContain("user+tag@example.org");

    expect(TestData.INVALID_EMAILS).toHaveLength(5);
    expect(TestData.INVALID_EMAILS).toContain("invalid");
    expect(TestData.INVALID_EMAILS).toContain("@domain.com");

    expect(TestData.VALID_URLS).toHaveLength(3);
    expect(TestData.VALID_URLS).toContain("https://example.com");
    expect(TestData.VALID_URLS).toContain("http://domain.org/path");

    expect(TestData.INVALID_URLS).toHaveLength(3);
    expect(TestData.INVALID_URLS).toContain("invalid");
    expect(TestData.INVALID_URLS).toContain("ftp://example.com");
  });

  test("should provide utility methods", () => {
    // Check that utility methods exist
    expect(typeof TestHelpers.waitForFormReady).toBe("function");
    expect(typeof TestHelpers.fillField).toBe("function");
    expect(typeof TestHelpers.checkCheckbox).toBe("function");
    expect(typeof TestHelpers.uncheckCheckbox).toBe("function");
    expect(typeof TestHelpers.selectOption).toBe("function");
    expect(typeof TestHelpers.uploadFile).toBe("function");
    expect(typeof TestHelpers.submitForm).toBe("function");
    expect(typeof TestHelpers.goToNextStep).toBe("function");
    expect(typeof TestHelpers.goToPreviousStep).toBe("function");
    expect(typeof TestHelpers.expectValidationErrors).toBe("function");
    expect(typeof TestHelpers.expectNoValidationErrors).toBe("function");
    expect(typeof TestHelpers.expectFieldValue).toBe("function");
    expect(typeof TestHelpers.expectFieldEmpty).toBe("function");
    expect(typeof TestHelpers.expectCheckboxChecked).toBe("function");
    expect(typeof TestHelpers.expectCheckboxUnchecked).toBe("function");
    expect(typeof TestHelpers.expectVisible).toBe("function");
    expect(typeof TestHelpers.expectNotVisible).toBe("function");
    expect(typeof TestHelpers.expectFocused).toBe("function");
    expect(typeof TestHelpers.expectNotFocused).toBe("function");
    expect(typeof TestHelpers.expectHasClass).toBe("function");
    expect(typeof TestHelpers.expectNotHasClass).toBe("function");
    expect(typeof TestHelpers.waitForSuccess).toBe("function");
    expect(typeof TestHelpers.waitForError).toBe("function");
    expect(typeof TestHelpers.setMobileViewport).toBe("function");
    expect(typeof TestHelpers.setDesktopViewport).toBe("function");
    expect(typeof TestHelpers.navigateWithTab).toBe("function");
    expect(typeof TestHelpers.navigateWithShiftTab).toBe("function");
    expect(typeof TestHelpers.pressEnter).toBe("function");
    expect(typeof TestHelpers.pressEscape).toBe("function");
    expect(typeof TestHelpers.wait).toBe("function");
    expect(typeof TestHelpers.takeScreenshot).toBe("function");
    expect(typeof TestHelpers.logPageState).toBe("function");
  });
});
