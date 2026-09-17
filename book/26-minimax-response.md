# Chapter 26 — Minimax Response (Axiom A25)

> *Part III — Zero-Day Axioms*

## 26.1 Motivation

Under epistemic uncertainty (Chapter 25), the defender does not have a
trustworthy probability distribution over what is happening. Bayesian decision
(Chapter 9) requires such a distribution; where it is absent, the defender must
fall back to a criterion that does not. That criterion is minimax regret.

## 26.2 Axiom

**Axiom A25 (Minimax response).** When the probability over states is unknown,
the defender chooses the action minimizing worst-case regret:

$$
a^{*} \;=\; \arg\min_{a}\ \max_{\theta \in \Theta}\ \mathrm{Regret}(a, \theta).
$$

## 26.3 Operations

**Regret.** The regret of an action under a state of the world theta is its
cost minus the cost of the best action for that theta:

$$
\mathrm{Regret}(a, \theta) \;=\; C(a, \theta) - \min_{a'} C(a', \theta).
$$

Regret measures how much worse the defender does than it *could have done* had
it known theta. Minimizing worst-case regret is the robust alternative to
minimizing expected cost: it never assumes a probability over theta.

## 26.4 Worked example

Consider the following costs, under a known state of the world and an unknown
one (the zero-day):

| Action | $C_{\mathrm{known}}$ | $C_{\mathrm{unknown}}$ |
|---|---|---|
| Do nothing | 0 | 50,000 |
| Alert | 5 | 100 |
| Block | 500 | 0 |
| Isolate | 50,000 | 0 |

For each state, the best action is the cheapest: under the known state it is
"do nothing" (cost zero), and under the unknown state it is "block" or "isolate"
(cost zero). Regret is therefore the cost minus zero in each column, and the
worst-case regret is:

| Action | Regret (known) | Regret (unknown) | Max regret |
|---|---|---|---|
| Do nothing | 0 | 50,000 | 50,000 |
| Alert | 5 | 100 | **100** |
| Block | 500 | 0 | 500 |
| Isolate | 50,000 | 0 | 50,000 |

The minimax action is **Alert**, with worst-case regret 100. The reasoning is
worth stating explicitly. "Do nothing" is catastrophic if the unknown state is
in fact an attack; "isolate" is catastrophic if it is not; "block" is moderately
bad if it is a false alarm. "Alert" is mildly costly in *every* state — at most
100 worse than the best possible — and is therefore the action that is most
robust to the defender's ignorance. Under zero-day uncertainty, where no
probability over the unknown is defensible, this is the correct choice, and it
is derived, not asserted.

## 26.5 Consequences

Minimax regret is the response-side counterpart of the Fano bound: it is a
*guarantee* that holds without assumptions. It is also a bridge to the freeze
response of Chapter 31, because "freeze" is, in many domains, the action whose
worst-case regret is minimized precisely when the defender knows least. The
minimax criterion unifies the zero-day chapters: when the state space is open
(Chapter 22) and uncertainty is epistemic (Chapter 25), the response is minimax
(this chapter), and the extreme case is freeze-and-isolate (Chapter 31).

## 26.6 What fails without A25

Without minimax, the defender is forced to use Bayesian decision even when it
has no prior — which is to say, it must invent a prior and then optimize against
its own invention. Under a zero-day, that invented prior is exactly what is
wrong, and the defender acts with false confidence. The robust criterion of
this chapter is the protection against that failure.
