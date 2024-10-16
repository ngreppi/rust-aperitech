# Aperitech RUST 2024

## Abstract
Qui sono raccolti degli esempi pratici utilizzati per il vespertino di introduzione al linguaggio RUST del 24/10/2024. Gli argomenti trattati coprono i principali concetti del linguaggio, con enfasi sulla sicurezza del suo modello di gestione della memoria e dei tipi.

## Contenuti

1. **01-hello-world**  
   Il primo programma in RUST.

2. **02a-simple-types**  
   Approfondimento sui tipi primitivi in RUST, inferenza dei tipi, variabili mutabili e immutabili.

3. **02b-complex-types**  
   Strutture e generici.

4. **03a-types-ownership**  
   Il meccanismo di ownership nelle variabili.

5. **03b-fn-ownership**  
   Il meccanismo di ownership nelle funzioni.

6. **04a-types-borrowing**  
   Il meccanismo di borrowing.

7. **04b-types-borrowing-rules**  
   Regole del meccanismo di borrowing.

8. **05a-lifetimes**  
   Utilizzo dei lifetimes.
   
9. **05b-lifetimes-safety**  
   Esempio di controlli di sicurezza effettuati dai lifetimes.
   
10. **06-concurrency**  
   Esempio di API che utilizza la concorrenza sicura.

## Comandi utili

1. Build di tutto il workspace:
   ```bash
   cargo build
   ```
2. Run di un modulo specifico (es: hello-world):
   ```bash
   cargo run --bin hello-world
   ```
3. Test di tutto il workspace:
   ```bash
   cargo test
   ```
4. Test di un modulo specifico (es: concurrency):
   ```bash
   cargo test -p concurrency
   ```