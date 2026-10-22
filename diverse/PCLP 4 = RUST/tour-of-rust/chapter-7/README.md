# Chapter 7


## Ce este POO?

Programarea orientată pe obiecte se referă, în linii mari,
la limbajele de programare ce prezintă anumite caracteristici reprezentative:

- Încapsularea - Asocierea datelor și funcțiilor într-o unitate conceptuală de un singur tip numită obiect.

- Abstractizarea - Ascunderea datelor și funcțiilor pentru a acoperi detaliile implementării unui obiect

- Polimorfismul - Abilitatea de a interacționa cu obiecte de tipuri diferite prin intermediul unei interfețe.

- Moștenirea - Abilitatea de a moșteni date și comportamente de la alte obiecte.




## Rust NU este POO

Rust nu dispune de o metodă semnificativă de moștenire a datelor și comportamentelor.

- Structurile nu pot moșteni câmpuri de la o structura părinte.
- Structurile nu pot moșteni funcții de la o structura părinte. Acestea fiind spuse, Rust implementează o mulțime de caracteristici ale unui limbaj de programare, deci probabail nu vă vor deranja aceste lipsuri.
- in Rust, `nu` putem realiza `overloading` cu metode



## Încapsulare prin metode

Rust suportă conceptul de obiect ce reprezintă o structură asociată unor funcții
(cunoscute și ca `metode``).
Primul parametru al oricărei metode trebuie să fie o referință la instanța asociată apelului de metodă (exemplu: `instantaObiectului.foo()`). Rust folosește:

- `&self` - referință imutabilă la instanța
- `&mut self` - Referință mutabilă la instanța. Metodele sunt definite în cadrul unuia sau mai multor blocuri de implementare prin cuvântul cheie `impl`

```Rust
impl StructuraMea { 
    ...
    fn foo(&self) {
        ...
    }
}
```


## Abstracție prin Expunere Selectivă

Rust `poate ascude implementările` interioare ale obiectelor.
În mod implicit, câmpurile și metodele sunt accesibile doar modulului de care aparțin.
Cuvântul cheie `pub` expune câmpurile și metodele structurii în afara modulului.


## Polimorfismul prin Trăsături

Rust suportă polimorfismul prin trăsături (în engleză traits).
Trăsăturile ne permit asocierea unui set de metode cu o structura.
Prima dată definim semnăturile metodelor unei trăsături în:

```Rust
trait TrasaturaMea {
    fn foo(&self);
    ...
}
```


Când o structură implementează o trăsătură, este stabilit un contract ce permite interacțiunea 
noastră indirectă cu structura, prin tipul trăsăturii (exemplu: `&dyn TrasaturaMea``), fără a fi 
nevoie să cunoaștem tipul real al structurii. Metodele trăsăturii implementate de structură sunt 
definite în cadrul unui bloc de implementare:

```Rust
impl TrasaturaMea for StructuraMea { 
    fn foo(&self) {
        ...
    }
    ... 
}
```


## Metode Implementate Trăsăturilor

Trăsăturile pot avea metode implementate. 
Funcțiile nu au acces direct asupra câmpurilor interioare ale unei structuri,
dar aceste metode implementate trăsăturilor pot fi folositoare în distribuirea comportamentelor 
între mai mulți implementarori ai trăsăturilor.



## Moștenirea Trăsăturilor

Trăsăturile pot moșteni metode de la alte trăsături.



## Expediere Dinamică vs Statică

Metodele sunt executate în două moduri:

- expediere statică (static dispatch)- Când tipul instanței este cunoscut, știm în mod direct ce funcție să apelăm.
- expediere dinamică (dynamic dispatch)- Când tipul instanței nu este cunoscut trebuie să găsim o modalitate de a apela funcția corectă. Tipurile trăsăturilor &dyn TrasaturaMea ne dau abilitatea de a lucra cu instanțe ale obiectelor în mod indirect, folosind expedierea dinamică. Când expedierea dinamică este folosită, este necesar să puneți `dyn` înaintea `trăsăturii``. Detalii cu privire la memorie:
- Expedierea dinamică este puțîn mai lentă, din cauza urmăririi prin pointeri pentru a determina apelul de funcție real.


# Trait Objects

Când pasăm o instanța a unui obiect ca parametru de tipul `&dyn TrasaturaMea`,
pasăm ceea ce se numește obiect al trăsăturii (în engleză `trait object). Acestea sunt ceea ce ne permite să apelăm în mod indirect metodele corecte ale unei instanțe. Un obiect al trăsăturii este o structura ce conține un pointer al instanței noastre, alături de o lista de pointeri la funcții către metodele acesteia. Detalii:

Această lista de funcții este cunoscută în C++ sub denumirea de vtable.



# Manevrarea Tipurilor de Date Fără Dimensiune Fixă

Trăsăturile introduc o provocare interesantă când dorim să le stocăm în interiorul altei structuri. 
Trăsăturile ascund structura originală, astfel ascunzând și dimensiunea originală. Tipurile de date 
fără dimensiune fixa sunt manevrate în două moduri în Rust:

- tipuri `generice` - Folosind tipuri parametrizate creăm structuri/funcții cu tipuri de date generice, a căror dimensiune va fi cunoscută în momentul folosirii lor, când programatorul va specifica un tip de date efectiv, care va avea și dimensiuni cunoscute.

- tipuri `indirecte` - Punerea instanțelor pe heap ne oferă avantajul de a nu fi necesar să cunoaștem dimensiunea tipului de date la compilare, aceasta fiind folosita doar în momentul rulării, când programatorul poate folosi orice tip de date care se portivește. Accesarea valorii efective se realizează printr-un pointer (presupune o indirectie), iar acest lucru poate fi considerat un dezavantaj. Mai există și alte modalități!




## functii Generice

Tipurile generice în Rust funcționează mâna în mâna cu trăsăturile. Când descriem un tip 
parametrizat `T`, putem constrânge ce tipuri pot fi folosite ca argumente, listând trăsăturile 
necesare (în engleză `trait bound`) pe care argumentul trebuie să le implementeze. În acest exemplu, 
tipul `T` trebuie să implementeze trăsătură `Foo`:


```
fn functia_mea<T>(foo: T)
where
    T:Foo
{
    ...
}
```

Folosind tipurile generice, creăm șabloane de funcții care au tipuri de date ce vor fi înlocuite la 
momentul compilării cu tipuri de date cunoscute, permițându-ne să utilizăm apelarea statică.



## Prescurtare a Funcțiilor Generice

Rust are o prescurtare pentru exprimarea tipurilor generice constrânse de o 
răsătură: `rust fn functia_mea(foo: impl Foo) { ... }` 
Este echivalent cu a scrie: `rust fn functia_mea<T>(foo: T) where T:Foo { ... }`.



## Box

`Box` este o structura de date ce ne permite să mutăm date de pe stivă în heap. Box este o structura 
cunoscută sub denumirea de smart pointer, ce conține pointerul către datele noastre în heap. 
Deoarece `Box` este o structură de dimensiune cunoscută (conține doar un pointer), este adesea folosit 
atunci când o structură trebuie să cunoască dimensiunea câmpurilor sale. Astfel, în interiorul 
structurii, vom stoca o referință prin intermediul lui `Box`. `Box` este atât de comun încât poate fi 
utilizat de oriunde:


```
Box::new(Foo { ... })
```



## Structuri Generice Revizuite


Structurile generice pot, de asemenea, să-și aibă tipurile parametrizate constrânse de trăsături

```
struct StructuraMea<T>
where
    T: StructuraMea
{
    foo: T
    ...
}
```

Structurile generice își au tipurile parametrizate în blocul lor de implementare:

```
impl<T> StructuraMea<T> {
    ...
}

```

 
# Capitolul 7 - Concluzie

Acum avem mai multe particularități ale limbajului prin care să ne exprimăm ideile într-un mod clar! 
Abstractizările făcutre de limbajul pot fi simple, însă sunt suficient de puternice pentru a face 
lucrul cu codul o bucurie. În acest capitol am aruncat o privire asupra smart pointerilor prin `Box`. 
În următorul capitol vom învață despre cum aceștia ne pot ajuta în alte situații de memorie 
specializate. Resurse:
