```bash
  _____            _   _   _                            _                         _
 |  ___|   ___    (_) | | | | __   ___    _ __   _ __  (_)   __ _    ___   _ __  (_)  _ __     __ _
 | |_     / _ \   | | | | | |/ /  / _ \  | '__| | '__| | |  / _` |  / _ \ | '__| | | | '_ \   / _` |
 |  _|   |  __/   | | | | |   <  | (_) | | |    | |    | | | (_| | |  __/ | |    | | | | | | | (_| |
 |_|      \___|  _/ | |_| |_|\_\  \___/  |_|    |_|    |_|  \__, |  \___| |_|    |_| |_| |_|  \__, |
                |__/                                        |___/                             |___/
```

## **Introduktion**
Dette er et CLI-program skrevet i Rust, der implementerer 4 forskellige måder at bruge fejlkorrigering på data, det er lavet som en del af en SOP (En stor skriftlig opgave).

Programmet gemmer filerne med fejlkorrigerings indkodning lokalt på computeren i disse filtyper:
- **Paritets bit** (.pab)
- **Kontrolsum** (.ces)
- **Tre kopier** (.3k)
- **Hamming Koder** (.hak)

##  **Installation**
>  **Vigtigt**     
>  For at køre dette projekt skal du have [Rust installeret](https://www.rust-lang.org/tools/install) (Projektet blev lavet med version 1.82.0).

### **Følg disse trin for at køre projektet direkte i rust:**
1. Klon dette repository:
    ```bash
    git clone https://github.com/BertramAakjaer/Fejlkorrigering-Implementering.git
    ```
2. Naviger til projektmappen:
    ```bash
    cd Fejlkorrigering-Implementering
    ```
3. Kør projektet:
    ```bash
    cargo run --release
    ```

### **Programmet kan også downloades "precompiled" under [releases](https://github.com/BertramAakjaer/Fejlkorrigering-Implementering/releases/tag/Fejlkorrigering) !!**

1. Bare kør `.exe` filen og programmet burde køre.


##  **Principper**
### **Paritets bit**
Paritets bit er princippet om at ved at tilføje en ekstra bit til dataen for at sikre, at antallet af 1 bits er lige. Her kan man så senere tjekke om antallet er lige, og hvis det er ulige ved vi at der er sket en fejl. `Bemærk at der bruges en hel byte i programmet`

### **Kontrolsum**
Kontrolsum er en måde at sammenlægge en større mængde data til noget der ikke fylder meget, men repræsentere det originale data. Her kan man så senere udregne en ny kontrolsum og, hvis den ikke stemmer overens med den gamle ved vi at dataen er ændret. `I programmet er denne kontrolsummen tilføjet i slutningen`

### **Tre kopier**
Her gemmes der tre kopiere af det originale data i en sekvens efter hinanden. Her kan man så sammenligne hver bit og hvis man har nogle der er anderledes kan vi tage den største delen siger, så hvis to af kopierne siger 0, går vi ud fra at det er 0, der er rigtigt.

### Hamming Koder
Hamming koder er en mere kompliceret metoder, hvor man inkoder dataen det kan læses om [her](https://people.math.aau.dk/~olav/gymnasie/talk2.pdf). `I min implementering benytter jeg også en paritets bit sammen med hamming koden, for at få det til at fylde 8 bit altså en byte, da det er nemmere at arbejde med i Rust`

##  **Socials** 🐦
>  [aakjaer.site](https://www.aakjaer.site) &nbsp;&middot;&nbsp;
>  GitHub [@BertramAakjær](https://github.com/BertramAakjaer) &nbsp;&middot;&nbsp;
>  Twitter [@BertramAakjær](https://twitter.com/BertramAakjaer)