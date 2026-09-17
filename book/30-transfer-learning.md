# Chapter 30 — Transfer Learning (Axiom A29)

> *Part III — Zero-Day Axioms*

## 30.1 Motivation

A zero-day is new, but it is rarely *entirely* new. It resembles known attacks,
and the defender should be able to transfer what it has learned from the known
to the unknown. This chapter formalizes transfer.

## 30.2 Axiom

**Axiom A29 (Transfer learning).** The posterior over a novel attack is
obtained by transferring beliefs from known, similar attacks:

$$
P(M_{\mathrm{new}} \mid o)  =  \sum_{M_{\mathrm{old}}} P(M_{\mathrm{new}} \mid M_{\mathrm{old}})  P(M_{\mathrm{old}} \mid o).
$$

## 30.3 Operations

**Similarity.** The similarity of two attacks is the Jaccard index of their
feature sets:

$$
\mathrm{Sim}(M_1, M_2)  =  \frac{|F_1 \cap F_2|}{|F_1 \cup F_2|}.
$$

**Transfer.** The prior over a novel attack is a similarity-weighted combination
of known priors:

$$
P(M_{\mathrm{new}})  =  \sum_{i} w_i \cdot P(M_i),
\qquad w_i \propto \mathrm{Sim}(M_{\mathrm{new}}, M_i).
$$

## 30.4 Worked example

Suppose a zero-day attack abuses GraphQL. The defender has two known, similar
attacks:

| Attack | Similarity to GraphQL abuse | Weight |
|---|---|---|
| REST API abuse | 0.7 | 0.7 |
| SQL injection | 0.3 | 0.3 |

The transferred prior over the novel attack is

$$
P(\texttt{GraphQL})  =  0.7 \cdot P(\texttt{REST}) + 0.3 \cdot P(\texttt{SQLi}).
$$

The defender does not start from zero for the unknown attack; it starts from a
similarity-weighted blend of what it already knows about API abuse and
injection. This is the formal content of "zero-days resemble old attacks":
transfer learning bridges the gap between the known and the unknown by
exploiting the resemblance.

## 30.5 Consequences

Transfer learning is the constructive response to the open state space of
Chapter 22. It does not pretend the unknown is known; it uses the *structure* of
similarity to the known to form a defensible prior, which the Bayesian machinery
of Chapter 6 then updates as evidence arrives. It also feeds the hypothesis
space of Chapter 29: the hypotheses a defender can form about a zero-day are
exactly the known attacks, transferred by similarity.

## 30.6 What fails without A29

Without transfer, the defender faces the unknown with no prior at all, and is
reduced to the minimax response of Chapter 26 as its *only* tool. Transfer does
not replace minimax — it complements it, by supplying the prior where a
defensible one exists.
