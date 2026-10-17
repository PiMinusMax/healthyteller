import { expect, test } from "@playwright/test";
import { acceptNotice, checkFixtureFood, createProfile, mockOpenFoodFacts } from "./helpers";

test.beforeEach(async ({ page }) => {
  await mockOpenFoodFacts(page);
  await acceptNotice(page);
});

test("compare two foods side by side", async ({ page }) => {
  await createProfile(page, { allergies: ["Milch"] });
  // A second food on the device: the search fixture.
  await page.getByRole("link", { name: /Lebensmittel prüfen/ }).click();
  await page.getByLabel("Lebensmittel suchen").fill("Quark");
  await page.getByRole("button", { name: "Suchen", exact: true }).click();
  await page.getByRole("button", { name: /Testquark mager/ }).click();
  await expect(page.getByRole("heading", { level: 1, name: "Testquark mager" })).toBeVisible();
  await page.goto("/");
  await checkFixtureFood(page);
  await page.getByRole("link", { name: "Mit anderem Lebensmittel vergleichen" }).click();
  await page.getByRole("link", { name: /Testquark mager/ }).click();
  await expect(page.getByRole("heading", { level: 1, name: "Statt Testjoghurt natur" })).toBeVisible();
  const table = page.getByRole("table");
  await expect(table.getByRole("columnheader", { name: "Testquark mager" })).toBeVisible();
  await expect(table.getByRole("row", { name: /Salz/ })).toBeVisible();
  await expect(table.getByRole("row", { name: /Allergene/ })).toContainText("meiden");
  await page.getByRole("button", { name: "Testquark mager auf den Teller legen" }).click();
  await expect(page.getByRole("status")).toContainText("Testquark mager liegt auf dem Teller.");
});

test("questions for the doctor come from the profile", async ({ page }) => {
  await createProfile(page, { conditions: ["Gicht oder erhöhte Harnsäure"] });
  await page.goto("/profile");
  await page.getByRole("link", { name: "Fragen an den Arzt" }).click();
  await expect(page.getByRole("heading", { level: 1, name: "Fragen für den nächsten Termin" })).toBeVisible();
  await expect(page.getByText("Gibt es für mich eine Grenze für Purin (als Harnsäure) pro Tag?")).toBeVisible();
  await expect(page.getByRole("status")).toHaveText("1 von 1 Fragen ausgewählt");
  await page.getByLabel("In den Bericht aufnehmen").uncheck();
  await expect(page.getByRole("status")).toHaveText("0 von 1 Fragen ausgewählt");
  await page.getByRole("textbox", { name: "Eigene Frage" }).fill("Darf ich Kaffee trinken?");
  await expect(page.getByRole("status")).toHaveText("1 von 2 Fragen ausgewählt");
  await expect(page.locator('[data-notice="doctor"]')).toBeVisible();
});

test("medication groups are recorded, stay unknown on the card and become questions", async ({ page }) => {
  await createProfile(page, { medications: ["Cholesterinsenker aus der Gruppe der Statine"] });
  await expect(page.getByText(/Statine/)).toBeVisible();
  await checkFixtureFood(page);
  const unknown = page.locator(".result", { hasText: "wegen Ihrer Medikamente" });
  await expect(unknown).toContainText("Medikamente");
  await expect(unknown).toContainText("unbekannt");
  await page.goto("/arztfragen");
  await expect(
    page.getByText("Worauf muss ich beim Essen achten, solange ich Cholesterinsenker aus der Gruppe der Statine nehme?"),
  ).toBeVisible();
  await expect(page.locator('[data-notice="doctor"]')).toBeVisible();
});
