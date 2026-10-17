import { expect, test } from "@playwright/test";
import { acceptNotice, checkFixtureFood, createProfile, mockOpenFoodFacts } from "./helpers";

test.beforeEach(async ({ page }) => {
  await mockOpenFoodFacts(page);
  await acceptNotice(page);
});

test("found allergen comes first, with shape, symbol and word", async ({ page }) => {
  await createProfile(page, { conditions: ["Gicht oder erhöhte Harnsäure"], allergies: ["Milch"] });
  await checkFixtureFood(page);
  const attention = page.locator("#card-attention").locator("..");
  await expect(attention.locator(".result").first()).toContainText("Allergene");
  const badge = attention.locator(".badge-avoid").first();
  await expect(badge).toHaveText("meiden");
  await expect(badge.locator("svg")).toBeVisible();
  await expect(page.getByText("In den Daten gefunden: Milch.")).toBeVisible();
  // Mandatory allergen notice on every card when the profile has an allergy.
  await expect(page.locator('[data-notice="allergens"]')).toContainText("Notruf 112");
});

test("a condition without a reviewed rule stays unknown", async ({ page }) => {
  await createProfile(page, { conditions: ["Gicht oder erhöhte Harnsäure"] });
  await checkFixtureFood(page);
  const unknown = page.locator("#card-unknown").locator("..");
  await expect(unknown).toContainText("Purin");
  await expect(unknown.locator(".badge-unknown")).toHaveText("unbekannt");
});

test("a doctor's limit is used for the verdict", async ({ page }) => {
  await createProfile(page, { conditions: ["Bluthochdruck"], saltLimit: "5" });
  await checkFixtureFood(page);
  const ok = page.locator("#card-unremarkable").locator("..");
  await expect(ok).toContainText("Salz");
  await expect(ok).toContainText("wegen Bluthochdruck");
  await expect(ok.locator(".badge-ok")).toHaveText("gut");
});

test("nutrient list shows origin marks, unknown values and the notices", async ({ page }) => {
  await createProfile(page);
  await checkFixtureFood(page);
  await page.getByRole("link", { name: "Alle Nährwerte" }).click();
  await expect(page.getByRole("heading", { level: 1, name: "Alle Nährwerte" })).toBeVisible();
  await expect(page.locator('[data-notice="variation"]')).toBeVisible();
  await expect(page.locator('[data-notice="data-status"]')).toContainText("Die Werte stammen von Open Food Facts");
  // Missing values are shown as unknown, never as zero.
  await expect(page.getByRole("row", { name: /Ballaststoffe unbekannt/ })).toBeVisible();
});

test("unknown barcode shows a message", async ({ page }) => {
  await page.unrouteAll();
  await mockOpenFoodFacts(page, "product-not-found.json");
  await page.getByRole("link", { name: /Lebensmittel prüfen/ }).click();
  await page.getByLabel("Barcode-Nummer").fill("4000000000031");
  await page.getByRole("button", { name: "Barcode prüfen" }).click();
  await expect(page.getByRole("alert")).toHaveText("Open Food Facts kennt dieses Produkt nicht.");
});

test("invalid barcode is rejected without a request", async ({ page }) => {
  let requests = 0;
  page.on("request", (r) => {
    if (r.url().includes("openfoodfacts")) requests++;
  });
  await page.getByRole("link", { name: /Lebensmittel prüfen/ }).click();
  await page.getByLabel("Barcode-Nummer").fill("123");
  await page.getByRole("button", { name: "Barcode prüfen" }).click();
  await expect(page.getByRole("alert")).toContainText("kein gültiger Barcode");
  expect(requests).toBe(0);
});

test("the card shows the data quality and what is missing", async ({ page }) => {
  await createProfile(page);
  await checkFixtureFood(page);
  // The fixture has no saturated fat, one of the seven label values.
  const quality = page.locator(".quality-explanation");
  await expect(quality.locator(".quality")).toContainText("unvollständig");
  await expect(quality).toContainText("Es fehlen: Gesättigte Fettsäuren.");
});

test("values calculated from ingredients are marked as such", async ({ page }) => {
  await page.unrouteAll();
  await mockOpenFoodFacts(page, "product-estimated.json");
  await createProfile(page);
  await page.getByRole("link", { name: /Lebensmittel prüfen/ }).click();
  await page.getByLabel("Barcode-Nummer").fill("4000000000062");
  await page.getByRole("button", { name: "Barcode prüfen" }).click();
  await expect(page.getByRole("heading", { level: 1, name: "Testapfel" })).toBeVisible();
  await page.getByRole("button", { name: "150 g" }).click();
  await page.getByRole("link", { name: "Bewerten" }).click();
  await expect(page.locator(".quality-explanation .quality")).toContainText("hochgerechnet");
});
