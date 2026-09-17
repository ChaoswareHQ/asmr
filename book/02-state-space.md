# Chapter 2 — The State Space (Axiom A1)

> *Part I — Foundations*

## 2.1 Motivation

Before anything can be monitored, there must be a notion of what is being
monitored. A security framework that cannot name its states cannot state what
it is missing. This chapter defines the state space and the two operations that
make partial knowledge expressible: projection and the fiber.

## 2.2 Axiom

**Axiom A1 (State product).** The state space is a product of factor spaces,

$$
S \;=\; \prod_{i \in I} S_i,
$$

where I is a finite index set of *factors* (the dimensions being monitored)
and S_i is the set of possible values of factor i. A state is a tuple that
assigns one value to each factor.

The cardinality of the state space is

$$
|S| \;=\; \prod_{i \in I} |S_i|.
$$

## 2.3 Operations

**Projection.** For a subset of factors J, the projection selects the
coordinates in J:

$$
\pi_J(s) \;=\; (s_i)_{i \in J}.
$$

Projection is what the defender is able to see.

**Fiber.** The fiber of an observation o is the set of states consistent
with it:

$$
\pi^{-1}(o) \;=\; \{\, s \in S \mid \pi(s) = o \,\}.
$$

**Gap.** The *gap* is the set of states that projection cannot distinguish from
some other state:

$$
\Delta \;=\; \{\, s \in S \mid |\pi^{-1}(\pi(s))| > 1 \,\}.
$$

A state is in the gap exactly when there exists a distinct state with the same
projection. The *gap ratio* is the number of ambiguous states divided by the
total number of states.

## 2.4 Worked example

Take four factors with the following value sets:

$$
S_{\mathrm{proc}} = \{\texttt{winword},\ \texttt{powershell},\ \texttt{cmd},\ \texttt{explorer},\ \texttt{unknown}\},\quad |S_{\mathrm{proc}}| = 5,
$$

$$
S_{\mathrm{net}} = \{\texttt{no\_conn},\ \texttt{internal},\ \texttt{c2\_conn},\ \texttt{ms\_cdn},\ \texttt{unknown}\},\quad |S_{\mathrm{net}}| = 5,
$$

$$
S_{\mathrm{file}} = \{\texttt{no\_download},\ \texttt{doc},\ \texttt{ps1},\ \texttt{exe},\ \texttt{unknown}\},\quad |S_{\mathrm{file}}| = 5,
$$

$$
S_{\mathrm{user}} = \{\texttt{normal},\ \texttt{privileged},\ \texttt{service},\ \texttt{unknown}\},\quad |S_{\mathrm{user}}| = 4.
$$

The state space has

$$
|S| = 5 \cdot 5 \cdot 5 \cdot 4 = 500
$$

states. A concrete state is

$$
s = (\texttt{powershell},\ \texttt{c2\_conn},\ \texttt{ps1},\ \texttt{normal}).
$$

Projecting onto the network factor alone gives

$$
\pi_{\mathrm{net}}(s) = \texttt{c2\_conn}.
$$

The fiber of that observation pins the network factor and leaves the other
three free:

$$
|\pi_{\mathrm{net}}^{-1}(\texttt{c2\_conn})| = 5 \cdot 1 \cdot 5 \cdot 4 = 100.
$$

The *ambiguity fraction* of a single network observation — the proportion of the
state space consistent with it — is therefore

$$
\frac{|\pi_{\mathrm{net}}^{-1}(\texttt{c2\_conn})|}{|S|} = \frac{100}{500} = 0.2.
$$

Because the network factor is the only one observed, *every* state has a fiber
of size 100, which is greater than one. Hence the gap is the entire space,

$$
\Delta = S,\qquad |\Delta| = 500,\qquad \frac{|\Delta|}{|S|} = 1.0.
$$

This is the point the example is intended to make, and it deserves emphasis: a
projection onto a single factor is *totally ambiguous*. Seeing `c2_conn` tells
the defender that the network factor is `c2_conn` and nothing else about the
remaining three factors. The observation is real evidence, but it leaves 100 of
the 500 states possible.

## 2.5 Consequences

The gap ratio is a measure of observability. If the projection is injective —
that is, if the defender observes enough factors to pin down the state
uniquely — then every fiber has size one and the gap is the empty set. If the
projection maps onto a single factor of a nontrivial product, then the gap is
the whole state space and the gap ratio is one. Between these extremes, the gap
ratio quantifies how far the defender is from full observability. This quantity
reappears in Chapter 4, where it is tied to the channel capacity of the
observation process, and in Chapter 40, where it is proposed as a predictor of
the false-positive rate.

## 2.6 What fails without A1

Without a state space there is no projection, no fiber, and no gap. In
particular, the sentence "the defender does not know X" has no referent: there
is no object X for the defender to fail to know. Every subsequent axiom —
observation, uncertainty, action, information — presupposes A1. A framework
without A1 cannot state the central problem of monitoring, which is precisely
the gap between the full state and what is observed.
