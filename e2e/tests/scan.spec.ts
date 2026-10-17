import { expect, test } from "@playwright/test";
import { acceptNotice, mockOpenFoodFacts } from "./helpers";

// Runs in the project "chromium-camera", which plays fixtures/barcode-4000000000013.y4m
// as the camera picture: an EAN-13 barcode with the test number 4000000000013.
test("barcode is recognised from the camera picture", async ({ page }) => {
  let requested = "";
  page.on("request", (r) => {
    if (r.url().includes("openfoodfacts")) requested = r.url();
  });
  await mockOpenFoodFacts(page);
  await acceptNotice(page);
  await page.getByRole("link", { name: /Lebensmittel prüfen/ }).click();
  await page.getByRole("button", { name: "Mit der Kamera scannen" }).click();
  await expect(page.getByLabel("Kamerabild zum Scannen des Barcodes")).toBeVisible();
  // The fixture answer is shown regardless of the number; the request proves what was read.
  await expect(page.getByRole("heading", { level: 1, name: "Testjoghurt natur" })).toBeVisible({ timeout: 15_000 });
  expect(requested).toContain("/product/4000000000013.json");
});
