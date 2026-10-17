# Projektregeln

Diese Regeln gelten für Menschen wie für KI-Agenten. Startauftrag: docs/auftrag.md, Konzept: docs/konzept.md.

1. Keine Fremddaten im Repo. Erlaubt sind Code, Regelwerk, Zuordnungstabellen mit IDs, Schätzrezepte mit Anteilen und IDs sowie Fixtures mit erfundenen Werten; data/ steht in der .gitignore, der Repo-Wächter prüft jeden Push.
2. Keine erfundenen Zahlen. Jeder Nährwert und jeder Grenzwert hat Quelle, Version und Abrufdatum. KI-Agenten tragen nie Werte aus dem Gedächtnis ein; fehlt eine Quelle, entsteht ein Issue.
3. Schätzungen nur nach dokumentierter Methode aus Quellwerten. Jede Schätzung trägt das Zeichen ≈, nennt ihre Grundlage und steht in der Schätzliste.
4. Gesundheitsdaten bleiben auf dem Gerät. Kein Konto, kein eigener Server, keine Analyse- oder Absturzdienste Dritter; Netzverkehr nur zu den Originalquellen.
5. Fehlende Werte sind keine Null. Unbekannt bleibt unbekannt und wird so angezeigt.
6. Kein Urteil ohne Begründung. Jede Ampel und jeder Vorschlag zeigt Wert pro Portion, einen Satz Grund und die Quelle.
7. Keine Gesamtnote, keine Diagnose, keine Dosisberechnung, keine Speisepläne für Erkrankungen. Konflikte zwischen Profilen werden angezeigt, nicht entschieden.
8. Vorsichtige Sprache. Nie "sicher" oder "frei von", sondern "in den Daten gefunden" oder "nicht gefunden", jeweils mit Datenvollständigkeit.
9. Der Kern bleibt rein. crates/core kennt weder Oberfläche noch Netz noch Dateisystem.
10. Merge nur bei Grün: Formatierung, Linter ohne Warnungen, Lizenzprüfung, Funktions-, Frontend- und Barrierefreiheitstests (WCAG 2.2 AA ohne Verstöße).
11. Jede Regel hat Referenzfälle innerhalb und außerhalb der Schwelle und wird erst nach fachlicher Freigabe ausgeliefert.
12. Jede Quelle hat einen Adapter, den Datei-Import als Rückfallweg, eine Fixture und einen Steckbrief mit Lizenz und Zitierhinweis.
13. Barrierefreiheit von Anfang an: nie nur Farbe, volle Tastaturbedienung, Screenreader-Beschriftungen, 200 % Zoom, einfache Sprache.
14. Alles ist auf Linux bau- und testbar; kein Schritt setzt einen Mac voraus.
15. Abhängigkeiten nur mit GPL-kompatibler Lizenz und ohne proprietäre SDKs, damit die App F-Droid-tauglich bleibt.
16. Code, Kommentare und Commits auf Englisch; Oberflächentexte nur über Übersetzungsdateien, Deutsch zuerst.
17. Kleine Pull Requests mit Conventional Commits; Architekturentscheidungen als ADR in docs/adr/.
18. Testprofile sind erfunden. Echte Personen- oder Gesundheitsdaten erscheinen nie in Code, Tests, Issues oder Logs.
19. Die Pflichthinweise (Grundhinweis, Absprache mit dem Arzt, schwankende Zusammensetzung, Schätzungen, Allergene, Datenstand) sind Teil der Oberfläche, lassen sich nicht abschalten und werden in den Ende-zu-Ende-Tests geprüft.
