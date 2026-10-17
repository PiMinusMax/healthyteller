# ADR 0004: Datenlage und Schätzung fehlender Werte

Stand: 17.10.2026 · Status: angenommen

## Kontext

Open Food Facts kennt fast jeden Barcode, aber oft nur wenige Nährwerte. Beispiele vom 17.10.2026: „Äpfel Elstar“ mit 3 Etikettwerten, ein Apfelprodukt ohne Marke mit keinem Wert. Das Konzept legt drei Schätzmethoden fest (Grundnahrungsmittel, Zutaten, Schätzrezepte), aber nicht, wie sie zusammenspielen und wie die App die Güte der Daten zeigt.

Open Food Facts berechnet für viele Produkte selbst Werte aus Zutaten und deren Anteilen (`nutriments_estimated`, bei „Äpfel Elstar“ 45 Werte) und liefert Zutaten mit geschätzten Anteilen und Kennungen (`ingredients[].id`, `percent_estimate`) sowie Kategorien (`categories_tags`).

## Entscheidungen

**Reihenfolge.** Jede Stufe füllt nur Werte, die noch fehlen. Etikettwerte haben immer Vorrang.

1. Etikett und andere Quellwerte (● gemessen, ∑ Herstellerangabe oder von der Quelle berechnet).
2. Von der Quelle aus Zutaten berechnet: `nutriments_estimated` von Open Food Facts (◐). Umgesetzt.
3. Übernahme vom zugeordneten Grundnahrungsmittel (≈, Methode 1). Die Kategorie wird über eine geprüfte Zuordnungstabelle in `mappings/` einem Eintrag einer Nährwertdatenbank (BLS, CIQUAL oder USDA) zugeordnet. Weichen vorhandene Etikettwerte deutlich ab (Energie um mehr als 20 %), wird nicht geschätzt. Oberkategorien wie „Obst“ reichen nicht.
4. Berechnung aus Zutaten durch die App (≈, Methode 2), wenn Open Food Facts nichts berechnet hat: Anteile mal Werte der zugeordneten Grundnahrungsmittel, geprüft gegen die Etikettwerte.
5. Schätzrezepte des Projekts (≈, Methode 3).

Stufen 3 bis 5 brauchen eine eingebundene Nährwertdatenbank und fachlich geprüfte Zuordnungen; sie folgen in eigenen Pull Requests. `Derivation::Estimated` trägt dafür die Methode (`EstimateMethod`).

**Datenlage je Lebensmittel.** Ein Zeichen mit Form und Wort zeigt die Güte (`crates/core/src/quality.rs`):

| Zeichen | Stufe | Bedeutung |
| --- | --- | --- |
| ● | Quellwerte | alle wichtigen Werte von der Quelle |
| ◐ | hochgerechnet | mindestens ein wichtiger Wert aus Zutaten berechnet (Stufe 2 oder 4) |
| ◔ | geschätzt | mindestens ein wichtiger Wert vom Grundnahrungsmittel oder aus einem Schätzrezept (Stufe 3 oder 5) |
| ○ | unvollständig | mindestens ein wichtiger Wert fehlt |

Es zählt der schwächste wichtige Wert. **Wichtig** sind nicht alle Nährstoffe, denn kein Lebensmittel ist in allen beschrieben, sondern:

- die sieben Pflichtangaben des EU-Etiketts (Energie, Fett, gesättigte Fettsäuren, Kohlenhydrate, Zucker, Eiweiß, Salz; Natrium ersetzt Salz). Jedes Lebensmittel hat sie, notfalls als echte 0,
- alle Nährstoffe, die das aktive Profil braucht: Regeln seiner Erkrankungen und Arztvorgaben,
- Allergenangaben, wenn das Profil Allergien hat.

Fehlt einer davon, ist das Lebensmittel unvollständig; die Karte nennt, was fehlt. Ein fehlendes Vitamin B12 beim Apfel zählt ohne passende Arztvorgabe also nicht, fehlendes Purin bei einem Gicht-Profil schon.

## Folgen

- Dasselbe Lebensmittel kann je nach Profil „Quellwerte“ oder „unvollständig“ zeigen. Das ist gewollt: Die Datenlage beantwortet, ob die Daten für diese Person reichen.
- Werte aus Stufe 2 sind keine Schätzung der App und stehen nicht in der Schätzliste; sie tragen ◐ und nennen Open Food Facts als Quelle.
- Suchtreffer ohne geladene Daten zeigen kein Zeichen, erst das per Barcode geladene Produkt.
