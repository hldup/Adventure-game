# Adventure game (school project)

<details>
<summary>Game description (HUN)</summary>
<br>
Adventure game (karakterek)

classok:
3 alap class van, minden classnak van: támadó erő, életerő és védelem
A kezdő classok és statjaik egyénileg kitalálhatóak, törekedjünk arra, hogy mindegyik class
egyedi legyen és egyik se legyen sokkal erősebb
támadó erő: a ellenfélre mért sebzés mértéke
életerő: emennyi sebzést el tudunk szenvedni, ha <=0 akkor meghaltunk/megöltük az ellenfelet
védelem: 1 védelem = 1% sebződés csökkenés, tehát ha az ellenfél 5 sebzést osztana ki, de van 10 védelmünk, akkor csak 4.5 sebzést kapunk
Minden ellenfél legyőzése után jár tapasztalati pont(tp) és megadott tapasztalati pont után
szintet lép a karakterünk és a játékos tudja fejleszteni valamely statját
A játékos nevét, class-t szintet és a statokat mentsük le egy fájlba,
így később lehet folytatni az állást

1. Új játékos esetén adjon meg egy nevet és ki lehessen választani,
hogy melyik classal kezd
2. Amennyiben van már játékos fájlunk, akkor listázzuk ki a játékos(ok) nevét
3. Minden szinten generáljunk egy ellenfelet random statokkal, a statjai legyenek a saját statjaink körül
   +-15%-al, tehát ha van 50 életerőnk akkor 42.5 és 57.5 közötti életereje legyen az ellenfelünknek
4. Tartsuk számon a legyőzött ellenfelek számát
5. Legyen 3 élettöltő főzetünk, mely 30% életerőt tölt fel (lehet több szintű főzetet csinálni mely 30%, 50%, 70%-ot tölt)  
   A főzetek szerzésére van esélyünk minden ellenfél legyőzése után, a 30%-osra 10% esélyünk van
6. A játék addig megy amíg él a karakterünk
7. Minden ellenél legyőzése után adjunk a karakternek tapasztalati pontot, ha az ellenfelünk erősebb volt mint mi,
   akkor többet, ha gyengébb akkor kevesebbet
   Minél magasabb szintű a karakterünk annál több tapasztalati pont kell a szintlépéshez
   Minden szint lépés után a játékos növelhet valamely statján
8. Minden ellenfél legyőzése után frissítsük a fájlunkat, mentsük le az alábbi dolgokat:
   - játékos neve
   - szintje
   - mennyi tp-nél jár és mennyi kell még, hogy szintet lépjen
   - statjai
   - legyőzött ellenfelek száma
9. Amennyiben vesztettünk, mentsük le, hogy hány ellenfelet sikerült legyőznünk
10. A legyőzött ellenfelek számából csináljunk ranglistát, melyet a játék kezdete előtt meg lehet tekinteni
</details>


# Developed on
### Developed on:
- OS: Fedora 37 Workstation Edition
- Kernel: 6.1.6-200
