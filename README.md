# ruine

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


