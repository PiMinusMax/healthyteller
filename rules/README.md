# Regelwerk

Eine YAML-Datei pro Regel. Format und Beispiel stehen in [docs/auftrag.md](../docs/auftrag.md), Abschnitt „Regelwerk“.

Jede Regel nennt ihre Quellen und hat Referenzfälle innerhalb und außerhalb der Schwelle. Die Quellen stehen in [citations/](citations/), die Testlebensmittel der Referenzfälle in [fixtures/rules/](../fixtures/rules/); `cargo test` prüft beides. Ausgeliefert wird sie erst mit `status: reviewed` nach fachlicher Freigabe.

Lizenz: CC BY-SA 4.0, siehe [LICENSES/CC-BY-SA-4.0.txt](../LICENSES/CC-BY-SA-4.0.txt).
