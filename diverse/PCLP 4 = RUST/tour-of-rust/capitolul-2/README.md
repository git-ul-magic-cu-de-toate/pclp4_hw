## Capitolul 2 - Controlul de bază al fluxului

În acest capitol vom vorbi despre metodele de bază de 
control ale fluxului în Rust. Dacă sunteți familiar cu 
limbaje bazate pe C, o să vă simțiți ca acasă și poate vă 
veți bucura de o surpriză sau două.

## IF / ELSE

Ramificarea codului în Rust nu este surprinzătoare.

Condițiile nu au nevoie de paranteze! Am avut vreodată de 
fapt nevoie de ele? Logica noastră poate arăta acum frumos și curat.

În rest, toți operatorii relaționali și logici funcționează la fel:
**`==`**, **`!=`**, **`<`**, **`>`**, 
**`<=`**, **`>=`**, **`!`**, **`||`**, **`&&`**.

## Bucle

Aveți nevoie de o buclă infinită?

Rust face asta într-un mod foarte simplu.

**`break`** vă va arunca în afara buclei când sunteți pregătit.

**`loop`** are un secret despre care vom vorbi curând.

## While

**`while`** vă lasă să adăugați o condiție logică unei bucle.

Dacă condiția impusă buclei devine falsă, bucla se va termina.

## for

Bucla **`for`** din Rust e o îmbunătățire importantă. Ea iterează 
peste valorile oricărei expresii care poate fi transformată 
într-un iterator. Vă întrebați ce este un iterator? Un 
iterator este un obiect pe care îl puteți întreba "Care este 
următorul element pe care îl ai?" până când acesta nu mai are elemente.

Vom explora acest lucru într-un capitol viitor. Între timp, 
țineți minte că Rust poate creea ușor iteratori care 
generează o secvență de numere întregi.

Operatorul **`..`** creează un iterator care generează numere de 
la un număr pâna la alt număr, din unu în unu, fără să îl 
includă pe cel din urmă.

Operatorul **`.=`** creează un iterator care generează numere de 
la un număr pâna la alt număr, din unu în unu, inclusiv cel din urmă.

## match

Vă lipsește **`switch`**-ul din C? Rust are un cuvânt 
cheie incredibil de folositor pentru analiza tuturor 
valorilor posibile ale unei variabile și executarea unei 
bucăți de cod dacă o anumită valoare corespunde condițiilor 
puse. Hai să vedem cum merge asta pentru numere. Avem de 
discutat mai multe în capitolele viitoare despre potrivirea 
șabloanelor (_pattern matching_) pentru date mai complexe. 
Vă promit că va merita așteptarea.

**`match`** este **`exhaustiv`**, deci toate cazurile trebuie să fie 
abordate și implementate.

Matching, combinat cu destructurarea datelor, este de 
departe unul din cele mai întâlnite șabloane de programare pe care le veți vedea în Rust.

operatorul **`_`** este utilizat într-un bloc **match** 
- reprezinta "orice valoare"
- reprezinta "orice altceva care nu este acoperit explicit de celelalte ramuri"
- Este cunoscut sub numele de "underscore" sau "wildcard". 

Atunci când se utilizează într-un bloc **match**, 
**`_`** acționează ca un "capturator de orice", 
încadrând orice valoare care nu a fost deja acoperită de alte cazuri.

## Returnarea unor valori dintr-o buclă

**`loop`** poate fi oprit pentru a returna o valoare.

## Returnarea unor valori din expresii bloc

**`if`**-urile, **`match`**-urile, funcțiile și domeniile bloc au un mod 
unic de a returna valori în Rust.

Dacă ultima instrucțiune din interiorul unui/unei **`if`**, **`match`**, 
funcții sau domeniu bloc este o expresie fără **`;`**, Rust o va 
returna sub forma unei valori din acel bloc. Acesta este un 
mod foarte bun de a crea o logică concisă care returnează o 
valoare care poate fi pusă într-o variabilă nouă.

Observăm cum acest lucru permite unui **`if`** să funcționeze ca o 
expresie ternară concisă.

# Capitolul 2 - Concluzii

Sper că v-am arătat o părticică din puterea Rust-ului chiar 
și prin cele mai simple caracteristici ale unui limbaj de 
programare. Vom vorbi despre **`for`** și **`match`** și mai mult pe 
măsură ce acumulăm mai multe cunoștințe care ne pot ajuta să 
ne folosim de capabilitățile lor. În cele ce urmează o să 
aruncăm o privire peste structurile de date fundamentale din 
Rust.
              
                                                                                                    
                                                                                      