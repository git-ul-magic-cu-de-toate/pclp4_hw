# Capitolul 4 - Tipuri generice

Tipurile generice sunt incredibil de importante în Rust. Sunt folosite pentru a reprezenta valori 
care pot fi nule (ex: variabile care poate nu au încă o valoare), abordarea erorilor, colecții și 
multe altele! În această secțiune vom învăța despre tipurile generice fundamentale pe care le veți 
folosi în majoritatea timpului.


## Ce sunt tipurile generice?

Tipurile generice ne lasă să definim parțial o structură (`struct`) sau o enumerare (`enum`),
permițând compilatorului să creeze o versiune definită complet în timpul compilării,
bazată pe modul în care am folosit codul.


În general, Rust poate deduce tipul final analizând instanțierea noastră, dar dacă are nevoie de 
ajutor, puteți oricând să instanțiați explicit folosind operatorul ``::<T>``,
cunoscut sub numele de `turbofish` (e un bun prieten de-ai mei!).



## Reprezentarea "nimicului"

În alte limbaje, cuvântul cheie `null` este folosit pentru a reprezenta absența unei valori.
El creează dificultăți în limbajele de programare, pentru că este posibil ca 
aplicația noastră să eșueze când interacționează cu o variabilă sau un câmp cu această valoare.

`Rust` `nu` are tipul `null`, dar nu este ignorant când vine vorba de importanța 
reprezentării nimicului! Gândiți-vă la o reprezentare naivă folosind o unealtă pe care 
o cunoaștem deja.



## Option
> Option => Some / None

Rust are o `enumerare` deja implementată numită `Option` care ne permite să 
reprezentăm valori care pot fi nule (`nullable`) fără să folosim null.

```
enum Option<T> {
    None,
    Some(T),
}
```

Această enumerare este foarte comună, instanțe ale acestei enumerări pot fi create 
oriunde prin intermediul elementelor din enumerare `Some` și `None`.



## Result
> Result => Ok / Err

Rust are o enumerare deja implementată numită `Result` care ne permite să returnăm o 
valoare care poate eșua în cadrul unei instrucțiuni.
Este modalitatea idiomatică în care limbajul Rust tratează posibilele erori.

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Observăm ca aceste tipuri generice au multiple **tipuri parametrizate** separate prin virgulă.

Această enumerare este foarte comună, instanțe ale ei pot fi create oriunde prin intermediul elementelor din enumerare `Ok` și `Err`.


## Main-ul care poate eșua

`main` are capacitatea de a returna un `Result!`

```
fn main() -> Result<(), String> {
    return Ok(());
}
```

Ruland in terminal `echo $?` pe codul de mai sus, vom obtine `0`.




```
fn main() -> Result<(), String> {
    return Err(String::from("eroare"));
}
```

Ruland codul de mai sus, se va afisa `eroare` in `stderr` (file descriptor = 2).
In terminal, comanda `echo $?` va afisa `1`.





```
fn main() -> Result<(), i32> {
    // in terminal will be displayed
    // Error: -11
    // echo $? => 1
    return Err(-11)?;
}
```



```
fn main() -> Result<(), i32> {
    // ne merge Result<i32, i32>
    return Ok(());
}
```

Codul de mai sus nu va genera nicio eroare