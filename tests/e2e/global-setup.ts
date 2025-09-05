import { chromium, FullConfig } from "@playwright/test";

async function globalSetup(config: FullConfig) {
  const { baseURL } = config.projects[0].use;

  // Start the browser and navigate to the app to ensure it's ready
  const browser = await chromium.launch();
  const page = await browser.newPage();

  try {
    // Wait for the app to be ready
    await page.goto(baseURL!);

    // Wait for the app to load completely
    await page.waitForLoadState("networkidle");

    // Verify the app is working by checking for key elements
    await page.waitForSelector("h1", { timeout: 30000 });

    // Verify we're on the test suite page
    const title = await page.textContent("h1");
    console.log(`✅ Test suite loaded: ${title}`);
  } catch (error) {
    console.error("❌ Failed to prepare application for testing:", error);
    throw error;
  } finally {
    await browser.close();
  }
}

export default globalSetup;
