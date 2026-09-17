# Chapter 25 — Epistemic Uncertainty (Axiom A24)

> *Part III — Zero-Day Axioms*

## 25.1 Motivation

Uncertainty comes in two kinds. *Aleatoric* uncertainty is irreducible noise:
the sensor is noisy, the world is random. *Epistemic* uncertainty is ignorance:
the defender does not know the model, because it has never seen this class of
state before. A zero-day is dominated by the second kind. This chapter
distinguishes them.

## 25.2 Axiom

**Axiom A24 (Epistemic uncertainty).** Total uncertainty decomposes additively,

$$
\mathrm{Uncertainty}  =  \mathrm{Aleatoric} + \mathrm{Epistemic}.
$$

## 25.3 Operations

**Aleatoric uncertainty.** The expected entropy of the outcome, averaged over
models theta:

$$
H_{\mathrm{aleatoric}}  =  \mathbb{E}_{\theta}\bigl[ H(Y \mid X, \theta) \bigr].
$$

**Epistemic uncertainty.** The entropy of the model itself, given the data:

$$
H_{\mathrm{epistemic}}  =  H(\theta \mid X).
$$

**Total.** The two sum to the full conditional entropy:

$$
H(Y \mid X)  =  H_{\mathrm{aleatoric}} + H_{\mathrm{epistemic}}.
$$

For a zero-day, the signature is

$$
H_{\mathrm{epistemic}} \gg H_{\mathrm{aleatoric}}.
$$

## 25.4 Worked example

Before a zero-day, a detector might exhibit

$$
H_{\mathrm{aleatoric}} = 0.1,\qquad H_{\mathrm{epistemic}} = 0.2.
$$

Most of the uncertainty is noise; the model is confident in what it knows. After
a zero-day is introduced, the model's ignorance dominates:

$$
H_{\mathrm{aleatoric}} = 0.1,\qquad H_{\mathrm{epistemic}} = 5.0.
$$

The noise has not changed — the sensors are as noisy as before — but the
model's uncertainty about its *own parameters* has risen by orders of
magnitude, because the data now contain a class of states the model has no
parameters for. This is the quantitative signature of the open state space of
Chapter 22: the defender's problem is no longer "classify within what I know"
but "I do not know what I do not know."

## 25.5 Consequences

The distinction determines the correct response to uncertainty. Aleatoric
uncertainty is irreducible and must be *averaged over*: the Bayesian machinery
of Chapter 6 is the right tool. Epistemic uncertainty is reducible in principle
and must be *guarded against*: the right tool is the robust, minimax criterion
of Chapter 26, because there is no trustworthy distribution to average over
when the model itself is in question.

The distinction also explains why a well-calibrated detector can fail
catastrophically on a zero-day without being "wrong" in any ordinary sense: its
aleatoric uncertainty was correctly estimated, but its epistemic uncertainty —
the possibility that the state space has expanded — was not part of the model.
A24 makes that possibility first-class.

## 25.6 What fails without A24

Without the aleatoric–epistemic distinction, the framework would treat
ignorance as noise and average over a model that does not contain the true
state, producing confident and wrong answers. The minimax response of
Chapter 26, which is the correct response to epistemic uncertainty, presupposes
A24.
