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
  await expect(page.locator("#plate-current").locator("..")).toContainText("Testjoghurt natur");
});

test("my foods: add from the card, put on the plate, undo", async ({ page }) => {
  await createProfile(page);
  await checkFixtureFood(page);
  await page.getByRole("button", { name: "Zu Meine Lebensmittel hinzufügen" }).click();
  await expect(page.getByText("Steht in Meine Lebensmittel.")).toBeVisible();
  await page.goto("/pruefen");
  await page.getByRole("button", { name: /Testjoghurt natur/ }).click();
  await expect(page.getByRole("status")).toContainText("liegt auf dem Teller");
  await page.getByRole("button", { name: "Rückgängig" }).click();
  await page.goto("/teller");
  await expect(page.getByText("Der aktuelle Teller ist leer.")).toBeVisible();
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
