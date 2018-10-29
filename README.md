# ruine

`ruine` est un utilitaire en ligne de commande permettant d'estimer les probabilités de gain dans
le problème de la ruine du joueur. Le calcul est par défaut effectué en parallèle pour
accélérer l'exécution, les résultats sont sérialisés en json.

## Installation

`ruine`  est écrit en Rust, il est nécessaire d'avoir une (installation Rust)[https://rustup.rs/]
pour pouvoir le compiler via  
```
$ cargo build --release
```


## Exemple

On peut utiliser les valeurs par défaut
```
$ ruine
{"f_max":10,"nb_parties":100,"proba":0.5,"seq":false,"valeurs":[0.13,0.27,0.26,0.47,0.48,0.69,0.75,0.76,0.95]}
```

On peut obtenir la documentation
```
$ ruine -h
Ruine du joueur 0.1
Vincent Perrollaz <vincent.perrollaz@laposte.net>
Calcul des probabilités de gains dans un jeu de pile ou face répétés

USAGE:
    ruine [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -s, --seq        Calculs sequentiels, par défaut en parallèle
    -V, --version    Prints version information

OPTIONS:
    -f, --f_max <f_max>              Fortune visée par le joueur pour s'arrêter
    -n, --nb_parties <nb_parties>    Nombre de parties jouées pour estimer une probabilité
    -p, --proba <proba>              Probabilité de gain du pile ou face
```

Exemple avec des paramètres
```
$ ruine -n 10000 -p 0.49 -f 50
{"f_max":50,"nb_parties":10000,"proba":0.49,"seq":false,"valeurs":[0.0058,0.0129,0.0206,0.0286,0.0349,0.0413,0.0528,0.0571,0.068,0.08,0.0835,0.0914,0.1118,0.1141,0.1305,0.1349,0.1497,0.1628,0.1736,0.1883,0.2048,0.2178,0.2286,0.2624,0.2733,0.291,0.3033,0.3301,0.3432,0.3607,0.3815,0.4046,0.4262,0.4486,0.4752,0.5089,0.5379,0.5592,0.5846,0.6251,0.6501,0.6833,0.7243,0.751,0.7957,0.8328,0.8669,0.908,0.9561]}
```

## Description mathématique du problème

On se donne  
- un réel $p\in[0,1]$,
- un entier positif $M$
- un entier $x\in [0,M]$

On considère alors une suite de variables aléatoires $(X_n)_{n\geq 0}$ satisfaisant 

$$\forall n\geq 0,\qquad \mathbb{P}(X_{n+1}=y+1|X_n=y)=p,\quad \mathbb{P}(X_{n+1}=y-1|
X_n=y)=1-p,$$ 

$$\mathbb{P}(X_0=x)=1.$$

On construit alors le temps d'arrêt

$$T:=\inf\{n\geq 0\ :\ X_n=0\quad \text{ou}\quad X_n=M\}.$$

On veut une estimation de $\mathbb{P}(X_T=M)$ en fonction des paramètres via une méthode de
Monte Carlo. 

On notera qu'il existe des méthodes pour résoudre analytiquement le problème, soit en se basant
sur la propriété de Markov soit via des techniques de Martingales.


