import { expect, test } from "@playwright/test";
import { acceptNotice, checkFixtureFood, createProfile, mockOpenFoodFacts } from "./helpers";

test.beforeEach(async ({ page }) => {
  await mockOpenFoodFacts(page);
  await acceptNotice(page);
});

test("plate sums the day and shows found allergens on top", async ({ page }) => {
  await createProfile(page, { conditions: ["Bluthochdruck"], allergies: ["Milch"], saltLimit: "5" });
  await checkFixtureFood(page);
  await page.getByRole("button", { name: "Auf den Teller legen" }).click();
  await expect(page.getByRole("heading", { level: 1, name: "Heute" })).toBeVisible();
  await expect(page.getByRole("alert").first()).toContainText("Milch gefunden");
  await expect(page.getByRole("img", { name: /Salz: heute .* von höchstens 5 g/ })).toBeVisible();
  await expect(page.locator("#plate-eaten").locator("..")).toContainText("Testjoghurt natur");
});

test("hypertension shows today's salt even without a daily value", async ({ page }) => {
  await createProfile(page, { conditions: ["Bluthochdruck"] });
  await page.getByRole("link", { name: /Teller ansehen/ }).click();
  const metrics = page.locator("#plate-metrics").locator("..");
  await expect(metrics).toContainText("Salz");
  await expect(metrics).toContainText("0 g");
  await expect(metrics).toContainText("Kein Tageswert hinterlegt");
  await expect(metrics.getByRole("link", { name: "Wert vom Arzt eintragen" })).toBeVisible();
});

test("my foods: add from the card, put on the plate, remove", async ({ page }) => {
  await createProfile(page);
  await checkFixtureFood(page);
  await page.getByRole("button", { name: "Zu Meine Lebensmittel hinzufügen" }).click();
  await expect(page.getByText("Steht in Meine Lebensmittel.")).toBeVisible();
  await page.goto("/");
  await page.getByRole("button", { name: /Testjoghurt natur/ }).click();
  await expect(page.getByRole("status")).toContainText("liegt auf dem Teller");
  await page.goto("/teller");
  await page.getByRole("button", { name: "Testjoghurt natur entfernen" }).click();
  await expect(page.getByText("Heute ist noch nichts eingetragen.")).toBeVisible();
});

test("print list contains the mandatory notices", async ({ page }) => {
  await createProfile(page, { conditions: ["Gicht oder erhöhte Harnsäure"], allergies: ["Milch"] });
  await checkFixtureFood(page);
  await page.getByRole("button", { name: "Zu Meine Lebensmittel hinzufügen" }).click();
  await page.goto("/druckliste");
  await expect(page.getByRole("heading", { level: 1, name: "Lebensmittelliste für Test Person" })).toBeVisible();
  for (const notice of ["basic", "doctor", "allergens", "data-status"]) {
    await expect(page.locator(`[data-notice="${notice}"]`)).toBeVisible();
  }
  await expect(page.locator("#print-avoid").locator("..")).toContainText("Testjoghurt natur");
});
