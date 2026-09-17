# Chapter 10 — Information Sets (Axiom A9)

> *Part I — Foundations*

## 10.1 Motivation

The defender does not reason from the current observation alone; it reasons from
the whole history. A single anomalous observation means little; repeated
anomalies build confidence. This chapter formalizes the defender's history and
the sequential update it supports.

## 10.2 Axiom

**Axiom A9 (Information sets).** The defender's information at time t is the
sequence of observations and its own past actions,

$$
I_t^D  =  (o_0, a_0, o_1, a_1, \ldots, o_t),
$$

and it is *bounded*: it is drawn from a bounded space

$$
I_t^D \subseteq (O \times A)^{t} \times K.
$$

The bound reflects that the defender's memory and storage are finite — a
constraint developed in Chapter 12.

## 10.3 Operations

**Bayesian update.** The posterior over the state, given the history, is updated
recursively:

$$
P(s_t \mid I_t^D)  \propto  P(o_t \mid s_t)  P(s_t \mid I_{t-1}^D).
$$

Each observation multiplies the prior by the likelihood of that observation;
the posterior is the normalized product. The history matters because the
likelihoods multiply across time: independent observations of the same anomaly
compound.

## 10.4 Worked example

Suppose the defender observes, in order, three intervals with no connection
followed by three command-and-control connections. Writing `no_conn` for the
first kind of observation and `c2_conn` for the second, its history is

`(no_conn, no_conn, no_conn, c2_conn, c2_conn, c2_conn)`.

Let the likelihoods be

$$
P(\text{no conn} \mid M) = 0.1,\quad P(\text{c2 conn} \mid M) = 0.85,
$$

$$
P(\text{no conn} \mid \neg M) = 0.98,\quad P(\text{c2 conn} \mid \neg M) = 0.02,
$$

and the base rate of the attack is 0.001. In odds form the prior is

$$
\frac{P(M)}{P(\neg M)} = \frac{0.001}{0.999} = 0.001001.
$$

Each observation multiplies the odds by its likelihood ratio. The likelihood
ratios are

$$
\mathrm{LR}(\text{no conn}) = \frac{0.1}{0.98} = 0.102041,
\qquad
\mathrm{LR}(\text{c2 conn}) = \frac{0.85}{0.02} = 42.5.
$$

After three benign observations and three C2 observations, the odds are

$$
0.001001 \cdot (0.102041)^3 \cdot (42.5)^3 = 0.081644,
$$

and the posterior is

$$
P(M \mid I_5) = \frac{0.081644}{1 + 0.081644} = 0.075481.
$$

Two things are worth noting. First, the three benign observations *reduce* the
odds, because `no_conn` is more common when there is no attack than when there is an attack;
they pull the posterior down before the C2 observations pull it up. Second, the
three repeated C2 observations are what drive the posterior from the single-
observation value of 0.040807 (Chapter 6) up to 0.075481. Repetition
compounds. A single C2 observation leaves the posterior below 5%; three
consecutive ones nearly double it. This is the quantitative content of the
claim that "history matters."

## 10.5 Consequences

The information set is the state of the defender's belief, as opposed to the
state of the system. It is what the defender's policy of Chapter 8 maps to an
action, and it is the input to the expected-cost calculation of Chapter 9. The
recursive form of the update means the defender need not retain the entire
history as raw data; it can retain a sufficient statistic (the posterior) and
fold each new observation into it. This is what makes the pipeline of
Chapter 33 computationally realizable even under the memory bounds of
Chapter 12.

## 10.6 What fails without A9

Without information sets there is no sequential detection, no recursive update,
and no correlation across time. The defender would reason from each observation
in isolation, forfeiting exactly the compounding effect demonstrated above. The
Bayesian machinery of Chapter 6, the decision rule of Chapter 9, and the
pipeline of Chapter 33 all presuppose A9.
