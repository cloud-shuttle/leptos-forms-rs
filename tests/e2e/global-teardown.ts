import { FullConfig } from "@playwright/test";

async function globalTeardown(config: FullConfig) {
  // Clean up any test artifacts or temporary files
  console.log("🧹 Cleaning up test environment...");

  // Add any cleanup logic here if needed
  // For example, clearing test databases, removing temp files, etc.

  console.log("✅ Test environment cleanup complete");
}

export default globalTeardown;
