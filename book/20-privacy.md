# Chapter 20 — Privacy and Data Minimization (Axiom A19)

> *Part II — Extended Axioms*

## 20.1 Motivation

Security monitoring collects sensitive data, and the law — GDPR, HIPAA, and
their analogues — constrains how much may be collected and how it must be
protected. A framework that ignores privacy will be noncompliant, and a
framework that treats privacy as an afterthought will discover too late that it
collected data it may not retain. This chapter formalizes data minimization.

## 20.2 Axiom

**Axiom A19 (Privacy and data minimization).** There exists a privacy operator

$$
\Pi : S \to S_{\mathrm{safe}}
$$

that maps the monitored state to a minimized, anonymized form.

## 20.3 Operations

**Differential privacy.** For two adjacent states and any set of outputs,

$$
P\bigl(\Pi(s) \in R\bigr) \;\le\; e^{\epsilon}\, P\bigl(\Pi(s') \in R\bigr) + \delta.
$$

The parameter epsilon bounds how much the output can reveal about any single
individual's data.

**Privacy budget.** The total budget is the sum over releases:

$$
\epsilon_{\mathrm{total}} \;=\; \sum_{i=1}^{n} \epsilon_i.
$$

**Utility loss.** Anonymization discards information; the loss is

$$
L_{\mathrm{util}} \;=\; H(S) - H(S_{\mathrm{safe}}).
$$

**Tradeoff.** The defender minimizes a weighted combination of utility loss and
privacy exposure:

$$
\min_{\Pi}\ \bigl( L_{\mathrm{util}}(\Pi) + \lambda \cdot \epsilon(\Pi) \bigr),
$$

where lambda weights privacy against utility.

## 20.4 Worked example

A monitoring pipeline handles both personally identifying and purely technical
data:

| Data | Identifying? | Handling |
|---|---|---|
| IP address | Yes | Hashed |
| Username | Yes | Tokenized |
| Process name | No | Preserved |
| File path | Maybe | Redacted |

For a release with privacy parameter epsilon equal to 0.1, the multiplicative bound
on disclosure is

$$
e^{0.1} = 1.105170918.
$$

That is, the output of the anonymization changes by at most a factor of
1.105 — about 10.5% — whether or not any single individual's record is
included. This is the precise meaning of "the release does not materially
depend on any one person's data." Smaller epsilon gives stronger privacy at
greater utility loss; the tradeoff term lambda times epsilon in the
objective makes that tension explicit.

## 20.5 Consequences

Privacy is a constraint on the *observation* side of the pipeline, and it
therefore interacts with the capacity analysis of Chapter 16. Anonymization
reduces the information available to the detector, so the privacy operator and
the observation map (the projection) of Chapter 4 compose: the defender's
effective channel is the privacy operator composed with the projection, and its
capacity is lower than that of the projection alone. The Fano bound then applies to the composed channel, quantifying
the detection cost of privacy. This is a concrete, computable tradeoff: a given
epsilon implies a given ceiling on detection performance, and the framework
makes that ceiling visible rather than letting it appear as a surprise.

## 20.6 What fails without A19

Without privacy there is no GDPR, no HIPAA, and no compliance. The framework
would treat data minimization as optional and would be unable to reason about
the detection cost of anonymization. In jurisdictions with mandatory
minimization, a framework that lacks A19 cannot even state the constraint under
which it operates.
