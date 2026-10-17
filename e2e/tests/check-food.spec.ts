import { expect, test } from "@playwright/test";
import { acceptNotice, checkFixtureFood, createProfile, mockOpenFoodFacts, typeBarcode } from "./helpers";

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

test("without meals a day a portion within the day limit is not called good", async ({ page }) => {
  await createProfile(page, { conditions: ["Bluthochdruck"], saltLimit: "5" });
  await checkFixtureFood(page);
  const unknown = page.locator("#card-unknown").locator("..");
  await expect(unknown).toContainText("Salz");
  await expect(unknown).toContainText("wie viele Mahlzeiten Sie am Tag essen");
});

test("a portion that takes more than one meal's share is judged with care", async ({ page }) => {
  // Invented limit: 0.3 g a day over 3 meals = 0.1 g per meal; the fixture portion has 0.18 g.
  await createProfile(page, { conditions: ["Bluthochdruck"], saltLimit: "0,3", meals: "3" });
  await checkFixtureFood(page);
  const attention = page.locator("#card-attention").locator("..");
  await expect(attention).toContainText("Salz");
  await expect(attention.locator(".badge-caution")).toHaveText("mit Maß");
  await expect(attention).toContainText("mehr als Ihr Anteil pro Mahlzeit von 0,1 g");
  await expect(attention).toContainText("Davon passen pro Mahlzeit");
  await page.goto("/pruefen");
  await page.getByLabel("Lebensmittel suchen").fill("Testjoghurt");
  await expect(page.locator("#search-results")).toContainText("Salz");
  await expect(page.locator("#search-results .badge-caution")).toBeVisible();
});

test("a doctor's limit is used for the verdict", async ({ page }) => {
  await createProfile(page, { conditions: ["Bluthochdruck"], saltLimit: "5", meals: "3" });
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
  await expect(page.getByRole("row", { name: /Eiweiß 6 g/ })).toBeVisible();
  await page.getByText(/^Unbekannt \(\d+\)$/).click();
  await expect(page.locator("#nutrients-unknown")).toContainText("Ballaststoffe");
  await expect(page.getByRole("row", { name: /Ballaststoffe/ })).toHaveCount(0);
});

test("unknown barcode shows a message", async ({ page }) => {
  await page.unrouteAll();
  await mockOpenFoodFacts(page, "product-not-found.json");
  await page.getByRole("link", { name: /Lebensmittel prüfen/ }).click();
  await typeBarcode(page, "4000000000031");
  await expect(page.getByRole("alert")).toHaveText("Open Food Facts kennt dieses Produkt nicht.");
});

test("invalid barcode is rejected without a request", async ({ page }) => {
  let requests = 0;
  page.on("request", (r) => {
    if (r.url().includes("openfoodfacts")) requests++;
  });
  await page.getByRole("link", { name: /Lebensmittel prüfen/ }).click();
  await typeBarcode(page, "123");
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
  await typeBarcode(page, "4000000000062");
  await expect(page.getByRole("heading", { level: 1, name: "Testapfel" })).toBeVisible();
  await page.getByRole("button", { name: "150 g" }).click();
  await page.getByRole("link", { name: "Bewerten" }).click();
  await expect(page.locator(".quality-explanation .quality")).toContainText("hochgerechnet");
});
