# Chapter 21 — Recovery and Resilience (Axiom A20)

> *Part II — Extended Axioms*

## 21.1 Motivation

A defensible position is not "we will not be breached" but "we will recover."
Recovery is the final stage of the pipeline, and resilience — the ability to
restore function after compromise — is the property that matters most for
business continuity. This chapter formalizes recovery.

## 21.2 Axiom

**Axiom A20 (Recovery and resilience).** There exists a recovery operator

$$
\rho : S_{\mathrm{compromised}} \to S_{\mathrm{restored}}
$$

that carries a compromised state to a restored state.

## 21.3 Operations

**Recovery time.** The time at which trust is fully restored is

$$
T_{\mathrm{rec}}  =  \inf \lbrace  t \mid \tau(s_t) = \tau_{\max}  \rbrace.
$$

**Resilience.** The resilience of a state is the normalized integral of trust
over the recovery trajectory:

$$
\mathrm{Res}(s)  =  \frac{\int_{0}^{\infty} \tau(s_t)  dt}{\tau_{\max} \cdot T_{\mathrm{rec}}}.
$$

**Mean time to recovery.** The expected recovery time is

$$
\mathrm{MTTR}  =  \mathbb{E}[T_{\mathrm{rec}}].
$$

**Availability.** Availability is the fraction of time the system is functional:

$$
A  =  \frac{\mathrm{MTBF}}{\mathrm{MTBF} + \mathrm{MTTR}},
$$

where $\mathrm{MTBF}$ is the mean time between failures.

## 21.4 Worked example

Recovery costs differ dramatically by incident type:

| Incident | MTTR | Cost |
|---|---|---|
| Ransomware | 72 hours | \$1M |
| Data breach | 30 days | \$5M |
| DDoS | 1 hour | \$10K |
| Insider | 6 months | \$2M |

For availability, take a system with $\mathrm{MTBF} = 1000$ hours and
$\mathrm{MTTR} = 10$ hours. Then

$$
A = \frac{1000}{1000 + 10} = \frac{1000}{1010} = 0.990099.
$$

The system is available roughly 99% of the time, unavailable roughly 1%.
The formula makes the tradeoff explicit: reducing MTTR from $10$ to $1$ hour
raises availability from $0.9901$ to $0.9990$ — a gain of nearly an order of
magnitude in downtime — which is the quantitative argument for investing in
faster recovery rather than only in prevention. Since $\mathrm{MTTR}$ appears
in the denominator, availability is most sensitive to recovery speed precisely
when $\mathrm{MTTR}$ is comparable to $\mathrm{MTBF}$.

## 21.5 Consequences

Recovery is the closure of the pipeline: after the governed action of
Chapter 13, the system must be returned to a trusted state, and the speed of
that return is a first-class quantity. Recovery also composes with the
composition algebra of Chapter 14: restoring a chain
$\texttt{WS} \to \texttt{FS} \to \texttt{DC} \to \texttt{DB}$ requires
restoring trust along each link, and the blast radius of the compromise is the
set of links that must be restored.

The resilience integral is the summary statistic for the entire recovery
trajectory, not just its endpoint. A system that recovers fully but slowly has
lower resilience than one that recovers partially but quickly and then
completes; the integral captures that shape.

## 21.6 What fails without A20

Without recovery there is no resilience and no business continuity. The
framework would stop at response — the governed action — and never reason about
the return to function, which is the outcome the organization actually cares
about. The freeze-and-isolate response of Chapter 31 and the recovery of the
SolarWinds case in Chapter 37 presuppose A20.
