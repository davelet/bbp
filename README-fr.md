# Calculatrice de π avec la Formule BBP

Ce projet est une implémentation en Rust de la formule de Bailey-Borwein-Plouffe (BBP) pour le calcul des chiffres de π. Sa caractéristique la plus remarquable est la capacité de calculer directement le n-ième chiffre hexadécimal de π sans avoir à calculer tous les chiffres précédents.

## La Formule BBP

La formule BBP a été découverte en 1995 par Simon Plouffe en collaboration avec David H. Bailey et Peter Borwein. Elle offre une méthode pour calculer π en base 16.

La formule est :

![Formule BBP](https://latex.codecogs.com/png.latex?\pi%20=%20\sum_{k=0}^{\infty}%20\frac{1}{16^k}%20\left(%20\frac{4}{8k+1}%20-%20\frac{2}{8k+4}%20-%20\frac{1}{8k+5}%20-%20\frac{1}{8k+6}%20\right))

Cet algorithme dit "spigot" permet d'extraire directement le chiffre hexadécimal de π.

## Logique Mathématique : Guide pour les Étudiants

Imaginez que vous voulez trouver le 100e chiffre de π. Normalement, il faudrait calculer les 99 chiffres précédents. La formule BBP agit comme une astuce magique qui vous permet d'obtenir directement le 100e chiffre hexadécimal de π sans les précédents.

### L'Idée Principale : Déplacement de la Virgule Hexadécimale

Par exemple, π s'écrit en hexadécimal comme `3.243F6A...`.

- Le premier chiffre après la virgule est `2`.
- Le deuxième est `4`.
- Le troisième est `3`.

Si l'on souhaite obtenir le **2e** chiffre (`4`), il suffit de multiplier π par 16 (ce qui s'écrit `10` en hexadécimal), déplaçant ainsi la virgule d'une position vers la droite :

```
π * 16^1 = 32.43F6A...
```

Le chiffre `4` apparaît alors immédiatement après la nouvelle virgule.

Pour le **4e** chiffre (`F`), on multiplie par `16^3` :

```
π * 16^3 = 3243.F6A...
```

L'idée fondamentale est donc : **Pour trouver le n-ième chiffre hexadécimal de π, on calcule `16^(n-1) * π` et on observe le premier chiffre après la virgule.**

### Le Problème : On ne Peut Pas Utiliser π Directement

Pour calculer `16^(n-1) * π`, il faudrait connaître π, qui est un nombre infini. L'ordinateur ne peut pas stocker tous ses chiffres.

C'est là que la formule BBP intervient, en permettant de calculer la **partie fractionnaire** de `16^(n-1) * π` sans connaître π dans son intégralité.

### La Solution : Décomposer la Formule BBP

La formule BBP est une somme infinie de petites fractions. En la multipliant par `16^(n-1)`, on obtient :

![Formule BBP Multipliée](https://latex.codecogs.com/png.latex?16^{n-1}\pi%20=%20\sum_{k=0}^{\infty}%20\left(%20\frac{4\cdot%2016^{n-1-k}}{8k+1}%20-%20\frac{2\cdot%2016^{n-1-k}}{8k+4}%20-%20\frac{1\cdot%2016^{n-1-k}}{8k+5}%20-%20\frac{1\cdot%2016^{n-1-k}}{8k+6}%20\right))

On se concentre sur la partie fractionnaire de cette somme qui se décompose en deux parties :

1. **La partie "Head" (pour `k` de 0 à `n-1`)** : Ces termes, ayant des puissances positives, génèrent de grands nombres. Leur contribution fractionnaire est calculée via l'exponentiation modulaire.
2. **La partie "Tail" (pour `k` de `n` à l'infini)** : Ces termes, avec des puissances négatives, produisent des nombres très petits (par exemple `1/16`, `1/256`). Ils peuvent être additionnés avec des nombres en virgule flottante (f64).

La somme des contributions fractionnaires de ces deux parties donne la partie fractionnaire de `16^(n-1) * π`.

### Prérequis

Pour comprendre cette explication, il est recommandé d'avoir des notions de :
- Représentation des nombres en base 16,
- Séries infinies et sommation,
- Arithmétique modulaire (calcul des restes).

### Introduction à l'Arithmétique Modulaire

L'arithmétique modulaire s'intéresse aux restes de la division. Par exemple, "a mod m" est le reste de la division de a par m. Un exemple utile :

`16^3 = 4096`, et `4096 mod 7 = 1`.

Cela permet de calculer efficacement des puissances même lorsque les exposants sont élevés.

### Exemple Numérique (n = 2)

Pour trouver le deuxième chiffre hexadécimal (qui est `4` dans `3.243F6A...`), procédez comme suit :
1. Calculez `16^(2-1) = 16` et multipliez par π.
2. Séparez la somme en deux parties : **Head** et **Tail**.
   - **Head** : Calculez la contribution fractionnaire via l'exponentiation modulaire.
   - **Tail** : Additionnez les termes très petits en utilisant f64.
3. Le résultat final, multiplié par 16, donne le chiffre souhaité.

### Remarque sur la Précision des Termes Tail

Les termes de la partie Tail décroissent rapidement, rendant l'utilisation de f64 suffisante pour la plupart des cas. Pour une précision plus élevée, utilisez l'arithmétique à précision arbitraire (comme le crate `bigdecimal` en Rust).

## Structure du Code

Le projet est organisé en trois modules principaux :

### `src/main.rs`

- Point d'entrée du programme.
- Appelle `bbp::get_digits()` pour calculer les chiffres hexadécimaux de π.
- Convertit et affiche la représentation décimale via `decimal::hex_pi_to_decimal()`.

### `src/bbp/mod.rs`

Contient la logique principale de la formule BBP :
- Fonctions : `get_digit(n)`, `series_sum(n, j)` et `mod_pow()`.

### `src/decimal/mod.rs`

Convertit les chiffres hexadécimaux en une représentation décimale en utilisant le crate `bigdecimal` pour des calculs de haute précision.

## Utilisation

### Prérequis

Installez le toolchain Rust (voir [rustup.rs](https://rustup.rs/)).

### Exécution

Dans le répertoire du projet, exécutez :
```bash
cargo run
```
Le programme affichera :
1. Les 500 premiers chiffres hexadécimaux de π,
2. Leur conversion en décimal,
3. Des exemples individuels (par exemple, les chiffres 10 et 50),
4. Une validation avec une valeur connue de π.

### Compilation

Pour compiler une version optimisée :
```bash
cargo build --release
```
Le binaire se trouve dans `target/release/bbp`.

## Dépendances

- `bigdecimal` : Utilisé pour les calculs en précision arbitraire lors de la conversion du format hexadécimal en décimal.
