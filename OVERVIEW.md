# r1-lang : overview & bibliothèque standard


## Table des matières

- [r1-lang : overview \& bibliothèque standard](#r1-lang--overview--bibliothèque-standard)
  - [Table des matières](#table-des-matières)
  - [Préface](#préface)
    - [Influences](#influences)
  - [Programme type](#programme-type)
  - [Règles basiques](#règles-basiques)
  - [Mots-clés réservés](#mots-clés-réservés)
  - [Types primitifs \& abstraits](#types-primitifs--abstraits)
    - [Mutables \& Constantes](#mutables--constantes)
    - [Union](#union)
    - [Casts](#casts)
    - [Types primitifs](#types-primitifs)
    - [Types abstraits](#types-abstraits)
    - [Déclaration de variables \& opérations](#déclaration-de-variables--opérations)
    - [L'accesseur '`::`'](#laccesseur-)
    - [Types utilisateur](#types-utilisateur)
  - [Fonctions](#fonctions)
    - [Paramètres \& références](#paramètres--références)
  - [Interpolation](#interpolation)
  - [Interpréteur](#interpréteur)
  - [Boucles \& itération](#boucles--itération)

## Préface

> Langage interprété, Programmation fonctionnelle & impérative, Fortement typé, éditables par défaut.
> R1 tente une approche nouvelle au langage interprété, en apportant la flexibilité d'un langage de haut-niveau, à la précision d'un langage de bas-niveau.

### Influences

> R1 est inspiré de plusieurs langages et paradigmes, tels que _Python, Rust, C & PHP_.
> La flexibilité d'un langage de haut-niveau tel que Python peut parfois se retourner contre le programmeur, lors par exemple de casts non désirés.
>
> De plus, R1 est par design sujet à la règle suivante : `grammaire > syntaxe && grammaire != syntaxe`.
>
> Il est alors convenu pour R1 d'ignorer les espaces.
> En effet, il convient de laisser au programmeur le choix de la syntaxe, mais de laisser au langage et à l'interpréteur le rôle d'imposer un respect de la grammaire et des règles de logique.

## Programme type

```none
// Add two variables and display their contents
let a<int.s32> = 2;
let b<int.s32> = a + 2;

stdlib::println::stdout(b);
b = b + 6;
stdlib::println::stderr(b);

c = a + b // fatal: variable `c` is not defined in this scope
```

## Règles basiques

- Noms de variables : `[a-zA-Z0-9_-]+`
- Une fonction (`func`) _peut ne pas_ contenir le mot-clé `return`, auquel cas le type de la fonction _devra être_ `<void>`
- Toute instruction doit se terminer par un point-virgule `;`
- L'utilisateur ne peut pas déclarer de variables globales
- Les lignes commençant par `//`, `/*` ou `#!` sont ignorées
  - Pour le cas du commentaire multilignes (`/*`), tout contenu sera ignoré avant le mot-clé `*/`
- R1 suit la philosophie du _RAII_ (_Resource Acquisition Is Initialization_)
  - Tout déclaration de variables doit aussi être initialisée
- R1 ne dispose pas de concepts tels que `null` ou `undefined`

## Mots-clés réservés

R1 ne permet pas à l'utilisateur de re-déclarer les mots-clés suivants :

- `if`
- `else`
- `func`
- `void`
- `int` (& variantes)
- `float` (& variantes)
- `str`
- `vec`
- `arr`
- `stdlib` (librairie standard)
- `::` (accesseur)
- `define`
- `let` (déclaration dans le scope local)
- `return`
- `@` (définit un cast)
- `for`
- `while`
- `break`
- `continue`
- `const`
- `&`
- `true`
- `false`

## Types primitifs & abstraits

### Mutables & Constantes

Une variable peut être définie comme constante grâce à l'ajout du mot-clé `const` après `let`.

Exemple :
```none
let const A<int.?> = 32;
stdlib::println::stdout(stdlib::typeof(A)); // '<int.s32>'
A = 0; // fatal: cannot reassign value to const variable
```

### Union

R1 permet à l'utilisateur de définir une union de deux types.

Cela peut-être utile si l'utilisateur veut spécifier qu'une fonction peut optionnellement retourner une erreur.

Syntaxe type :
```none
func unsafe_function<int.u32|error<str>>(x<int.s32>)
{
  // dynamically allocate & cast an anonymous string to an error container
  if ( x < 0 ) { return "x cannot be inferior to 0"@<error.str>; }
  return x@<int.u32>; // cast x to an unsigned 32-bit integer
}
```

### Casts

L'utilisateur peut effectuer un cast entre deux types, là où cela fait du sens, comme ceci :

```none
let a<int.s32> = -4;
let b<int.s64> = 64;
let c<int.u8>  = (a + b)@<int.u8>;
```

### Types primitifs

- `void` - Pas de type de retour, utilisé pour les fonctions
- `str` - Chaîne de caractères allouée sur le heap dynamiquement
- `float.{signe}{N}` - Nombre à virgule flottant où `signe = {s,u}` et `N = {8,16,32,64,128}`
  - `float.s8` - Nombre à virgule flottant signé sur 8 bits
  - `float.s16` - Nombre à virgule flottant signé sur 16 bits
  - `float.s32` - Nombre à virgule flottant signé sur 32 bits
  - `float.s64` - Nombre à virgule flottant signé sur 64 bits
  - `float.s128` - Nombre à virgule flottant signé sur 128 bits
  - `float.u8` - Nombre à virgule flottant non-signé sur 8 bits
  - `float.u16` - Nombre à virgule flottant non-signé sur 16 bits
  - `float.u32` - Nombre à virgule flottant non-signé sur 32 bits
  - `float.u64` - Nombre à virgule flottant non-signé sur 64 bits
  - `float.u128` - Nombre à virgule flottant non-signé sur 128 bits
  - `float.?` - Nombre à virgule flottant dont le signe et la taille seront devinés à l'exécution

- `int.{signe}{N}` - Nombre entier où `signe = {s,u}` et `N = {8,16,32,64,128}`
  - `int.s8` - Nombre entier signé sur 8 bits
  - `int.s16` - Nombre entier signé sur 16 bits
  - `int.s32` - Nombre entier signé sur 32 bits
  - `int.s64` - Nombre entier signé sur 64 bits
  - `int.s128` - Nombre entier signé sur 128 bits
  - `int.u8` - Nombre entier non-signé sur 8 bits
  - `int.u16` - Nombre entier non-signé sur 16 bits
  - `int.u32` - Nombre entier non-signé sur 32 bits
  - `int.u64` - Nombre entier non-signé sur 64 bits
  - `int.u128` - Nombre entier non-signé sur 128 bits
  - `int.?` - Nombre entier dont le signe et la taille seront devinés à l'exécution

### Types abstraits

- `vec<T>` - Vecteur d'un nombre non défini d'éléments de type primitif T.
  - Un `vec` peut grandir ou rétrécir à la volée, grâce aux méthodes `*::pop(index?)`, `*::push(x)`.
  - Méthodes accessibles via l'accesseur `::` (e.g. `let var<vec<int.s32>> = [1,2,3]; var::size; var::pop(index?); var::push(element)`)
- `arr<N,T>` - Liste de N éléments de type primitif T.
- `error<T>` - Type d'erreur,

### Déclaration de variables & opérations

- Syntaxe type - `let IDENTIFIER<TYPE> = VALUE | EXPRESSION;`
- Réassignation - `IDENTIFIER = VALUE | EXPRESSION;`

### L'accesseur '`::`'

Sur des types abstraits & éléments de la librairie standard (`stdlib`), accéder à une sous-propriété d'un élément se fait par le mot-clé `::`.

Exemple type :
```none
let myVec<vec<int.s32>> = [1, 2, 3, 4];
stdlib::println::stdout(myVec::size);  // prints '4' to standard output
stdlib::println::stdout(myVec::pop()); // prints '4', myVec now contains [1, 2, 3]
stdlib::println::stdout(myVec::sum()); // Prints '6'
```

### Types utilisateur

Un utilisateur peut définir ses propres types, grâce au mot-clé `define`. Un type utilisateur peut se traduire assez facilement à un `struct` en C.

Syntaxe type :
```none
define my_type
{
  label<str>,
  x<int.s32>,
  y<float.u128>,
};

func create_user_type<my_type>()
{
  let a<my_type> = {
    x: -55,
    y: 5.9754,
    label: 'point_coordinates',
  };

  return a;
}

func display_user_type<void>(a<my_type>)
{
  stdlib::println::stdout(f"${a::label} has values: x = ${a::x}, y = ${a::y}");
}
```

## Fonctions

Syntaxe type :
```none
func function_name<return_type>(optional_parameter<type>)
{
  // instructions...
  return value;
}
```

Exemple concret :
```none
func add_numbers<int.s64>(a<int.s32>, b<int.s32>) { return a+b; }
func greet<void>(name<str>) { stdlib::println::stdout("Hello ", name); }
```

### Paramètres & références

Par défaut, tout paramètre passé à une fonction est passé par valeur. Cependant, il est possible de passer une référence à l'objet grâce au mot-clé `&`.

Exemple :
```none
func add(&ref, value<float.?>) { ref = ref + value; }
let a<int.s32> = 10;
add(&a, 10);
stdlib::println::stdout(f"a's value is ${a}");
```

## Interpolation

L'interpolation de variables dans des chaînes de caratères est possible via le préfixe `f` avant le contenu de la variable.

Par exemple : `let a<str> = f"Hello, my name is ${variable_name}";`

## Interpréteur

L'interpréteur r1 sera écrit en rust, et pourra être appelé comme ceci :

```
$ r1 --help
r1 interpreter [version x.y.z]
usage : r1 <flags> [file]
  -i, --interactive : start an interactive r1 shell, evaluating scripts line-by-line.
  -v, --version     : print r1's version.
```

## Boucles & itération

R1 supporte des paradigmes traditionnels, tels que les boucles `for` & `while`.

Exemple :
```none
// (declaration & initialization, loop condition, loop instruction)
for (let i<int.?> = 0; i <= 10; i++)
{
  stdlib::println::stdout(i);
}

let i<int.?> = 0;
while (true)
{
  if (i == 10) { break; }
  stdlib::println::stdout(i);
  i++;
}
```

Avec la boucle `for`, il est possible d'avoir plusieurs accumulateurs. Par exemple :
```none
for ( let i<int.?> = 0, let j<int.?> = 10; i <= 10; i++, j-- )
{
  stdlib::println::stdout(f"$i = {i}, j = ${j}");
}
```