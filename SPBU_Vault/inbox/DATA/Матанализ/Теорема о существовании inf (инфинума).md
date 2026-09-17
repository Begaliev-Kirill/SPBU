# Теорема о существовании inf (инфинума)
$\gimel  G \subset \R, G$ называется ограниченным снизу, если $\exists \ l \in \R : \forall \ y \in G \ y \geq l$, тогда l - нижняя граница $G$

Утверждение: \
$\gimel G \subset R, G$ ограничен снизу, тогда $\exists l_0 : \forall y \in G\ y \geq l_0$ и $\forall l \in \R, l$ - нижняя граница $G \ l_0 \geq l$   
$l_0$ - инфинум $G$, максимальная нижняя гранциа.

Доказательство: \
$\gimel E = \{x : \exists \ y \in G : x = -y\}$ \
$\gimel c_0 = \sup E$ \
$\forall x \in E \ x \leq c_0 \Leftrightarrow -x \geq -c_0 \Rightarrow \forall y \in G \ y \geq -c_0 \Rightarrow -c_0$ - нижняя граница $G$, назовем $-c_0$ как $l_0$ \
$\exists l : y \geq l \Leftrightarrow -y \leq -l \Rightarrow \forall x \in E \ x \leq -l$,тогда $-l$ - верхняя граница $E$ $\Rightarrow -l \geq c_0 \Rightarrow l \leq -c_0 = l_0$\
Получается, что любаяя нижняя граница будет меньше или равна $-c_0$, тогда $-c_0 = l_0$ - инфинум $\blacksquare$


