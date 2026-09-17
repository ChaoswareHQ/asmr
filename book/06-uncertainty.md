# Chapter 6 — Uncertainty (Axiom A5)

> *Part I — Foundations*

## 6.1 Motivation

The observation channel of Chapter 4 leaves residual ambiguity. That ambiguity
must be represented, and the natural representation is probability. This
chapter introduces the probability space, Bayes' rule, and the distinction
between probabilistic and robust (set-based) modeling of uncertainty.

## 6.2 Axiom

**Axiom A5 (Uncertainty model).** There exists a probability space (a sample space, a collection of events, and a probability measure) over which uncertainty is represented, or, when probabilities cannot be assigned, a robust uncertainty set of admissible models.

## 6.3 Operations

**Bayes' rule.** For the hypothesis that an attack is present, and an observation,

$$
P(M \mid o)  =  \frac{P(o \mid M)  P(M)}{P(o)}.
$$

**Expectation.** For a random quantity X,

$$
\mathbb{E}[X]  =  \int_{\Omega} X   dP.
$$

**Robust decision.** When the model is only known to lie in the uncertainty set,

$$
a^{*}  =  \arg\min_{a} \max_{\omega \in \Omega} C(a, \omega),
$$

which is the minimax criterion over the uncertainty set.

**Kullback–Leibler divergence.** For two distributions P and Q,

$$
D_{\mathrm{KL}}(P \parallel Q)  =  \sum_{x} P(x) \log \frac{P(x)}{Q(x)}.
$$

The KL divergence reappears in Chapter 15, where it measures how far an
attacker's behavior has drifted, and in Chapter 16, where it bounds the cost of
a suboptimal prior.

## 6.4 Worked example: a C2 observation

Suppose a network sensor reports a command-and-control connection. The relevant
quantities are the base rate of compromise and the likelihoods of the
observation under the two hypotheses:

$$
P(M) = 0.001,\qquad P(\texttt{c2} \mid M) = 0.85,\qquad P(\texttt{c2} \mid \neg M) = 0.02.
$$

The marginal probability of the observation is

$$
P(\texttt{c2}) = 0.85 \cdot 0.001 + 0.02 \cdot 0.999 = 0.020830.
$$

Bayes' rule then gives

$$
P(M \mid \texttt{c2}) = \frac{0.85 \cdot 0.001}{0.020830} = 0.040807.
$$

The conclusion is important and, at first, surprising: an endpoint showing a
connection to a command-and-control server is still more likely *benign* than
malicious, by a factor of roughly 24 to 1. The reason is the base rate — compromise
is rare, and a rare condition requires strong evidence to become probable.
This single number is the foundation of the decision theory of Chapter 9: it is
the posterior that feeds every cost calculation.

## 6.5 Worked example: the base-rate fallacy

The same arithmetic explains the base-rate fallacy. Consider a detector with a 99% detection rate and a 1% false-positive rate, operating where one event
in ten thousand is a true attack:

$$
P(M) = 0.0001,\qquad P(\texttt{alert} \mid M) = 0.99,\qquad P(\texttt{alert} \mid \neg M) = 0.01.
$$

The alert fires with probability

$$
P(\texttt{alert}) = 0.99 \cdot 0.0001 + 0.01 \cdot 0.9999 = 0.010098,
$$

and the probability that a fired alert is a true attack is

$$
P(M \mid \texttt{alert}) = \frac{0.99 \cdot 0.0001}{0.010098} = 0.009804.
$$

Fewer than one alert in a hundred corresponds to a real attack, despite a
detector that sounds superficially excellent. The fallacy is to read the
99% detection rate as "99% of alerts are true"; the arithmetic shows
otherwise. ASMR makes this arithmetic explicit and, in Chapter 9, uses it to
choose actions.

## 6.6 Consequences

Probability is the connective tissue of the framework. It converts the
observation model of Chapter 4 into a posterior; the posterior is the input to
the decision rule of Chapter 9; the decision rule is filtered by governance in
Chapter 13. The robust alternative of Section 6.3 is not a rival to probability
but a fallback: when probabilities cannot be assigned — the zero-day setting of
Part III — the framework switches to minimax over an uncertainty set, which is
developed in Chapter 26.

## 6.7 What fails without A5

Without uncertainty there is no noise, no posterior, and no Bayesian decision.
The defender is reduced to binary rules that either fire or do not, with no way
to weigh the cost of acting against the cost of failing to act. Every
quantitative chapter that follows — cost (Chapter 9), information (Chapter 10),
trust (Chapter 11), and the pipeline (Chapter 33) — presupposes A5.
