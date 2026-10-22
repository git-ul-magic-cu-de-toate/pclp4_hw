# Capitolul 1

În acest prim capitol vom explora ideile și conceptele de
bază despre funcții, variabile și tipurile primare de date.
Ne bucurăm că ne sunteți alături!

De asemenea, în cazul în care vă întrebați cine este crabul 
adorabil care vă vorbește, mă numesc **Ferris**, mascota 
neoficială a limbajului Rust. Încântat de cunoștință!

Odată ce vă veți familiariza cu Rust, puteți să vă declarați 
un **Rustaceu**. Așa se numesc toți oamenii care folosesc, 
contribuie sau sunt interesați de Rust.

## Rust Playground

În acest curs veți folosi un utilitar interactiv pentru scrierea codului [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021).

E o modalitate bună de a vă juca cu Rust și a arăta altora creativitatea și provocările dumneavoastră!

## Variabile

Variabilele sunt declarate folosind cuvântul cheie **`let`**.

Atunci când dați unei variabile o valoare, Rust este capabil 
să deducă tipul acesteia în 99% din cazuri. Dacă acesta nu 
poate, puteți adăuga tipul de date la declararea variabilei.

Observați cum putem să folosim același nume de variabilă 
(același identificator) de mai multe ori. Acest fenomen se 
numește variable shadowing (punere în umbră) și tipul de 
date se poate schimba pentru referințe ulterioare la acel 
nume de variabilă.

> Folosind underscore **`_`** inaintea numelui de variabila, nu se va genera warning daca nu este utilizata in program

Numele de variabile sunt mereu scrise sub formă **`snake_case`** 
(ex: my_first_variable).
![case types](https://miro.medium.com/v2/resize:fit:720/format:webp/1*PSDlLqeuDgeqClF-LLqPPw.png)

## Modificarea variabilelor

Limbajul Rust ține foarte mult la ce variabile pot fi 
modificate. Variabilele fac parte din două categorii:

- **mutabile** (mutable) - compilatorul va lăsa utilizatorul să modifice șî să citească valoarea acestui tip de variabile.
- **imutabile** (immutable) - compilatorul va lăsa utilizatorul doar să citească valoarea acestei variabile.

Valorile mutabile sunt declarate cu ajutorul cuvântului cheie **`mut`**.

O să revenim pentru a vorbi mai multe despre acest concept, dar deocamdată doar fiți atenți la acest cuvânt cheie.

> Folosind underscore **`_`** inaintea numelui de variabila, nu se va genera warning daca nu este utilizata in program

## Tipuri de date de bază

Rust are o varietate de tipuri de date familiare dumneavoastră:

- variabilă booleană - **`bool`** pentru a reprezenta adevărat și fals
- numere întregi fără semn - **`u8` `u16` `u32` `u64` `u128`** pentru a reprezenta numere naturale
- numere întregi cu semn - **`i8` `i16` `i32` `i64` `i128`** pentru a reprezentare numere întregi
- numere întregi de dimensiunea unui pointer - usize isize pentru a reprezenta indexi și dimensiunea datelor în memorie
- numere cu virgulă mobilă - **`f32` `f64`** pentru a reprezenta numere reale
- tuplu - **`(valoare, valoare, ...)`** pentru trecerea unor secvențe fixe de valori pe stivă
- tablou - **`[valoare, valoare, ...]`** o colecție de elemente de același tip; dimensiunea colecției este fixă și devine cunoscută doar în momentul compilării
- parte (slice) - o parte dintr-o colecție de elemente de același tip; dimensiunea părții devine cunoscută doar în timpul rulării
- **`str`** (string slice) - text de lungime cunoscută în timpul rulării
![tipuri de date](https://miro.medium.com/v2/resize:fit:640/format:webp/1*5bd_OBT6Nha2Wl05asyqfg.png)

Sintaxa s-ar putea să fie mai complexă în Rust față de alte 
limbaje de programare cu care sunteți obișnuiți; având în 
vedere că Rust este un limbaj de programare pentru sisteme, 
îi pasă de problemele de memorie cu care s-ar putea să nu 
fiți obișnuiți. O să aprofundăm aceste lucruri mai târziu.

Tipurile numerice pot fi specificate explicit prin adăugarea 
tipului la finalul numărului (ex: **`13u32`**, **`2u8`**).

## Conversia tipurilor de bază

Rust necesită claritate atunci când vine vorba de tipurile 
numerice. Nu puteți folosi un **`u8`** pentru un **`u32`** în mod 
neglijent fără apariția unei erori.

Din fericire, Rust poate face conversia de la un tip numeric 
la altul, foarte ușor, folosind cuvântul cheie **`as`**.

## Constante

Constantele ne permit să specificăm o valoare folosită uzual 
de-a lungul codului nostru în mod eficient. În loc să 
copieze valori, constantele înlocuiesc direct, acolo unde 
sunt utilizate, identificatorul de text cu valoarea sa, la 
momentul compilării.

Spre deosebire de variabile, constantelor trebuie să li se 
specifice explicit tipul la declarare.

Numele constantelor sunt mereu scrise în format 
**`SCREAMING_SNAKE_CASE`** (ex: MY_FIRST_CONSTANT).
![case types](https://miro.medium.com/v2/resize:fit:720/format:webp/1*PSDlLqeuDgeqClF-LLqPPw.png)

## Tablouri

Un _tablou_ este o colecție de dimensiune fixă de elemente care conțin date de același tip.

Tipul de date pentru un tablou este scris sub forma **`[T;N]`**, unde T reprezintă tipul elementelor, iar N reprezintă dimensiunea fixă cunoscută la momentul compilării.

Elemente individuale pot fi accesate cu ajutorul operatorului **`[x]`**, unde x este un index usize (începând cu 0) al elementului pe care doriți să-l accesați.

Colecțiile cu dimensiune dinamică, deseori numite tablouri dinamice sau variabile, vă vor fi prezentate într-un capitol viitor numit **Vectori**.

## Funcții

O funcție admite zero sau mai mulți parametri.

În acest exemplu, funcția add admite doi parametri de tip 
**`i32`** (număr întreg cu semn cu dimensiune de 32 de biți).

La returnarea unei expresii se poate omite cuvântul cheie 
'return' și simbolul punct-virgulă de la final, așa cum s-a 
procedat în funcția subtract.

Numele funcțiilor sunt mereu scrise în format **`snake_case`**.
![case types](https://miro.medium.com/v2/resize:fit:720/format:webp/1*PSDlLqeuDgeqClF-LLqPPw.png)

Hint: dacă definiți o funcție, datele pe care le admite se 
numesc parametrii formali. Dacă apelați o funcție și îi 
transmiteți niște date, atunci ele se numesc parametrii actuali.

## Returnarea mai multor valori

Funcțiile pot returna mai multe valori prin returnarea unui 
**tuplu** de valori.

Elementele unui tuplu pot fi accesate folosind indexul 
acestora. (ex: my_tuple.0)

Rust suportă diferite tipuri de destructurări pe care le vom 
vedea în multe contexte, permințându-ne să extragem 
fracțiuni ale unor structuri de date în moduri ingenioase. 
Fiți pe fază!

## A nu returna nimic

Dacă pentru o funcție nu se specifică ce tip returnează, 
aceasta va returna un tuplu gol, cunoscut și sub numele de 
unitate (unit).

Un tuplu gol este reprezentat de **`()`**.

Folosirea unui **`()`** nu este des întâlnită, dar va apărea de 
suficiente ori, deci este bine să știți ce se întâmplă.

## Capitolul 1 - Concluzii

Până acu', e bine! Bazele Rust-ului nu sunt așa de rele, nu-i așa? Am aruncat o privire modului în care gândește compilatorul de Rust. Fiind un limbaj de programare pentru sisteme, acesta ține mult la dimensiunea valorilor în memorie, fie că pot fi modificate sau nu, și la corectitudinea matematicii pe care o aplicați. În cele ce urmează, ne vom întâlni cu niște vechi prieteni: teste **`if`** și bucle **`for`**.

Bibliografie:

- [Youtube: Rust Cast - A deeper dive on Rust's primitive number types](https://www.youtube.com/watch?v=n5TRBkbystY)
- [Website: Rust Book 2018 - A deeper description on basic data types](https://doc.rust-lang.org/1.30.0/book/2018-edition/ch03-02-data-types.html)
- [Website: Rust Cheat Sheet - Data Types](https://cheats.rs/#basic-types)
