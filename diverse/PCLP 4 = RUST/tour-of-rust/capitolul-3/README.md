# Capitolul 3

## Structuri

Un **`struct`** este o colecție de câmpuri (field-uri).

_Câmpurile_ sunt pe scurt date asociate unei structuri. Valorile lor pot fi de tip primar sau o structură de date.

Definiția structurii este ca o matriță pentru compilator pentru a ști cum să aranjeze câmpurile în memorie într-un mod compact.

> Note : Folosind underscore **`_`** inaintea numelui de variabila, nu se va genera warning daca nu este utilizata in program

## Apelarea metodelor

Spre deosebire de funcții, metodele sunt funcții asociate 
unui tip specific de date.

**metode statice** — metode care aparțin unui tip de date și 
sunt apelate folosind operatorul **`::`**.

**metode ale instanței** — metode care aparțin unei instanțe a 
unui tip de date și sunt apelate folosind operatorul **`.`**.

Vom dezvolta acest concept, cu implementarea propriilor 
metode, în capitolele următoare.


## Memorie

Aplicațiile scrise în Rust au 3 zone de memorie unde este stocată informație:
- **memoria pentru date** - pentru date care sunt de dimensiune fixă și sunt statice (adică mereu disponibile pe toată durata rulării aplicației). Considerați textul din programul dumneavoastră (ex: "Hello World!"): memoria ocupată (bytes) de acest text este citită dintr-un singur loc în cod deci poate fi stocat în această zonă de memorie. Compilatoarele fac foarte multe optimizări pentru acest tip de date și folosirea lor în general este considerată foarte rapidă, pentru că locația lor este cunoscută și fixă.
- **memoria pentru stivă (stack)** - pentru date declarate ca variabile în interiorul unei funcții (variabile locale). Locația în memorie a acestor date nu se schimbă pe durata apelului funcției; datorită acestui lucru compilatoarele pot optimiza codul astfel încât datele din stivă se accesează foarte rapid.
- **memoria pentru alocare dinamică (heap)** - pentru date care sunt create în timpul rulării aplicației. Datele în această zonă de memorie pot fi adăugate, mutate, șterse, redimensionate, etc. Din cauza naturii sale dinamice, este în general considerată mai lentă, dar această zonă permite utilizări mult mai creative ale memoriei. Când adăugăm date în această zonă de memorie, numim această operație **alocare (de memorie)**. Când stergem date din această zonă de memorie, numim această operație **dealocare (de memorie)**.

## Crearea datelor în Memorie

Când **instanțiem** o **structură** în codul nostru, aplicația creează câmpurile de date unul lângă altul în memorie.

Instanțiem o structură specificând toate valorile câmpurilor în interiorul structurii, dată prin numele său.

Câmpurile unei structuri sunt accesate folosind operatorul **`.`**.

Detalii despre memorie în exemplul nostru:

- Textul din interiorul ghilimelelor este o dată care poate fi numai citită (ex: "Ferris"), deci acesta este pus în _zona memoriei pentru date_.
- Apelul funcției **`String::from`** creează o structură de tip **`String`** ale cărei câmpuri sunt depuse, unul lângă altul, lângă câmpurile structurii, pe _stivă_. Un String reprezintă text care poate fi modificat în următoarele moduri:
1. Se alocă memorie pe _heap_ pentru text și acolo va putea fi modificat
1. Stochează o referință la acea locație de pe _heap_ în structura **`String`** (Mai multe despre acest concept în următoarele lecții)
- Așadar, cei doi prieteni ai noștri, _Ferris_ și _Sarah_, sunt structuri de date care vor avea mereu locații fixe în aplicația noastră, deci vor fi puși în _stivă_.

## Structuri de tip tuplu

Pentru a fi conciși, puteți creea structuri care sunt folosite ca un tuplu.

## Structuri de tip unitate

Structurile nu trebuie neapărat să aibă câmpuri.

După cum am menționat în Capitolul 1, _unitate_ este o altă 
denumire pentru un tuplu gol, **`()`**. De aceea, acest tip de structuri se numește _Unit-like_.

Acest tip de structuri este folosit rar.

## Enumerări

Enumerările vă permit să creați un tip nou de date care poate avea o valoare 
dintr-o mulțime de elemente prestabilite folosind cuvântul cheie **`enum`**.

Elementele unei enumarari pot fi accesate folosind operatoru **`::`**.

**`match`** ne ajută să abordăm toate valorile posibile 
ale unei enumerări, făcând din acest tip un instrument puternic pentru asigurarea calității codului.

## Enumerările care conțin tipuri de Date

Elementele unui **`enum`** pot avea de asemenea unul sau mai multe 
tipuri de date, permițându-i acestuia să se comporte ca un _union_ din limbajul C.

Atunci când un **`enum`** este utilizat într-un **`match`**, puteți atașa un nume de variabilă fiecărei valori din **`enum`**.

Detalii despre memorie pentru un **`enum`** :

- Spațiul de memorie alocat unei variabile de tip enumerare va fi egal cu spațiul de memorie necesar pentru stocarea celui mai mare element al enumerării. Acest lucru asigură faptul că orice valoare posibilă a enumerării va încăpea în același spațiu din memorie.
- Pe lângă tipul de date al unui element (dacă acesta are un tip), fiecare element are de asemenea asociată o valoare numerică care reprezintă indexul acestuia în enumerare.

Alte detalii:

- **`enum`**-ul din Rust este cunoscut și ca _uniune etichetată_ (tagged union).
- Combinarea mai multor tipuri de date pentru a creea unul nou este ceea ce îi face pe oameni să afirme faptul că Rust are _tipuri algebrice_.

## Capitolul 3 - Concluzie

Ce palpitant! Acum avem în sfârșit toate uneltele de bază 
pentru a reprezenta ideile noastre în cod. Sper că acum 
putem vedea puțin din felul în care operațiile fundamentale 
din Rust funcționează armonios și în mod concis cu tipurile 
sale de date. În cele ce urmează o să vorbim despre un 
concept care atribuie tipurilor noastre de date și mai multă 
flexibilitate în reprezentare: _genericitate_ (generics).
