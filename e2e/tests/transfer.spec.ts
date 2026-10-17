import { expect, test } from "@playwright/test";
import { acceptNotice, createProfile } from "./helpers";

const PASSWORD = "test-password-123";

test("encrypted export can be imported after deleting all data", async ({ page }, testInfo) => {
  test.setTimeout(90_000);
  await acceptNotice(page);
  await createProfile(page, { conditions: ["Bluthochdruck"], allergies: ["Milch"] });
  await page.goto("/einstellungen/geraetewechsel");

  await page.getByLabel("Passwort", { exact: true }).first().fill(PASSWORD);
  await page.getByLabel("Passwort wiederholen").fill(PASSWORD);
  await page.getByRole("button", { name: "Verschlüsselte Datei erstellen" }).click();
  const downloadPromise = page.waitForEvent("download");
  await page.getByRole("link", { name: "Datei speichern" }).click();
  const file = testInfo.outputPath("export.htx");
  await (await downloadPromise).saveAs(file);

  await page.goto("/einstellungen/erweitert");
  await page.getByRole("button", { name: "Alle Daten löschen" }).click();
  await page.getByRole("button", { name: "Endgültig löschen" }).click();
  await acceptNotice(page);
  await expect(page.getByText("Noch kein Profil.")).toBeVisible();

  await page.goto("/einstellungen/geraetewechsel");
  await page.getByLabel("Exportdatei (.htx)").setInputFiles(file);
  await page.locator("#import-pass").fill("wrong password!");
  await page.getByRole("button", { name: "Importieren", exact: true }).click();
  await expect(page.getByRole("alert")).toHaveText("Das Passwort ist falsch oder die Datei ist beschädigt.");
  await page.locator("#import-pass").fill(PASSWORD);
  await page.getByRole("button", { name: "Importieren", exact: true }).click();
  await expect(page.getByRole("status")).toHaveText("Import abgeschlossen.");

  await page.goto("/");
  await expect(page.getByText("Test Person")).toBeVisible();
  await expect(page.getByText(/Milch-Allergie/)).toBeVisible();
});

test("export needs matching passwords of sufficient length", async ({ page }) => {
  await acceptNotice(page);
  await page.goto("/einstellungen/geraetewechsel");
  await page.getByLabel("Passwort", { exact: true }).first().fill("short");
  await page.getByLabel("Passwort wiederholen").fill("short");
  await page.getByRole("button", { name: "Verschlüsselte Datei erstellen" }).click();
  await expect(page.getByRole("alert")).toHaveText("Das Passwort ist zu kurz.");
  await page.getByLabel("Passwort", { exact: true }).first().fill("long enough 1");
  await page.getByLabel("Passwort wiederholen").fill("long enough 2");
  await page.getByRole("button", { name: "Verschlüsselte Datei erstellen" }).click();
  await expect(page.getByRole("alert")).toHaveText("Die beiden Passwörter stimmen nicht überein.");
});
