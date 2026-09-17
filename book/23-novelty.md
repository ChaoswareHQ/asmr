# Chapter 23 — Novelty Measurement (Axiom A22)

> *Part III — Zero-Day Axioms*

## 23.1 Motivation

If the state space is open, the defender needs a way to recognize that a state
is *new*. This chapter defines a novelty score: the distance from a state to the
nearest known state.

## 23.2 Axiom

**Axiom A22 (Novelty measurement).** There exists a novelty function

$$
\mathrm{Novel} : S \to \mathbb{R}_{\ge 0}
$$

measuring how far a state is from the set of known states.

## 23.3 Operations

**Nearest-known distance.** Novelty is the distance to the nearest known state:

$$
\mathrm{Novel}(s)  =  \min_{s' \in S_{\mathrm{known}}} d(s, s').
$$

**Weighted metric.** The distance is a weighted sum of per-factor mismatches:

$$
d(s, s')  =  \sum_{i \in I} w_i \cdot \delta(s_i, s'_i),
$$

where the mismatch indicator is one when the factors differ and zero otherwise, and the factor weight is the importance of factor i.

**Threshold.** A zero-day alert fires when novelty exceeds a threshold:

$$
\text{Zero-day alert}  \Longleftrightarrow  \mathrm{Novel}(s) > \theta_{\mathrm{novel}}.
$$

## 23.4 Worked example

Let the known state be `(powershell, c2_conn, ps1, normal)`, and suppose a new
state appears with one additional, previously unseen factor:
`(powershell, c2_conn, ps1, normal, new_factor)`.

The novelty of the new state is determined by the distance to the nearest known state.
Because the new state carries a factor that no known state possesses, its distance to
every known state includes the weight of that novel factor, and (with unit
weights) the novelty is

$$
\mathrm{Novel}(s_2) = 1.0.
$$

If the novelty threshold is 0.5, the zero-day alert fires.
The example illustrates the intended use: novelty is not about a suspicious
*value* of a known factor but about the *presence* of a factor — or a factor
value — outside the known envelope. A zero-day is novel precisely in this sense.

## 23.5 Consequences

Novelty is the detector for the *appearance* of the unseen, as distinct from
the detectors of Part I, which classify the seen. It is complementary to
structural anomaly (Chapter 24): novelty measures distance in the factor space,
while structural anomaly measures distance in the *graph* of relationships.
Both are needed, because a zero-day may be novel in its values, its structure,
or both.

## 23.6 What fails without A22

Without novelty there is no way to recognize the unseen, and the open state
space of Chapter 22 is a formal possibility with no operational consequence.
The defender would have no trigger for the freeze response of Chapter 31.
