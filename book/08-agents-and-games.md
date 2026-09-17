# Chapter 8 — Agents and Games (Axiom A7)

> *Part I — Foundations*

## 8.1 Motivation

The defender is not alone. There is an adversary who observes the defender's
behavior and adapts. A framework that models only one side will systematically
underestimate the difficulty of the problem. This chapter introduces both
agents and the game they play.

## 8.2 Axiom

**Axiom A7 (Multi-agent structure).** There exist a defender D and an attacker A, each with partial information and a policy,

$$
\mu_D : I_t^D \to A_D,\qquad \mu_A : I_t^A \to A_A,
$$

mapping its own information to an action. The interaction is a partially
observable stochastic game.

## 8.3 Operations

**Master equation.** The defender's optimal policy is the maximin solution of
the discounted game:

$$
\mu_D^{*} \;=\; \arg\max_{\mu_D}\ \min_{\mu_A}\ \mathbb{E}\!\left[\sum_{t=0}^{\infty} \gamma^{t}\, U_D \right].
$$

The defender maximizes the worst case over the attacker, because the attacker
is an optimizing adversary, not a stationary noise source.

**Nash equilibrium.** A pair of policies, the defender's and the attacker's, is a Nash equilibrium when each
side is a best response to the other:

$$
\mathrm{Nash}(\mu_D, \mu_A) \;\Longleftrightarrow\; \mu_D = \mathrm{BR}(\mu_A) \ \wedge\ \mu_A = \mathrm{BR}(\mu_D).
$$

**Stackelberg equilibrium.** When the defender commits first and the attacker
responds, the defender solves

$$
\mathrm{SE}(\mu_D) \;=\; \arg\max_{\mu_D}\ \min_{\mu_A}\ \mathbb{E}[U_D \mid \mu_D, \mu_A].
$$

## 8.4 Worked example

Consider an intrusion by the group publicly known as FIN7. Its observed
tactics, mapped to the MITRE ATT&CK taxonomy, are:

| Phase | Technique | Identifier |
|---|---|---|
| Initial access | Spearphishing attachment | T1566.001 |
| Execution | PowerShell | T1059.001 |
| Persistence | Registry run keys | T1547.001 |
| Command and control | Web protocols (HTTPS) | T1071.001 |

The defender's responses, phase by phase, are:

| Phase | Response | Autonomy |
|---|---|---|
| Phishing | Email filtering | Autonomous |
| PowerShell | Endpoint blocking | Autonomous |
| Run key | Registry monitoring | Alert |
| C2 | Firewall block | Autonomous |

The information asymmetry is decisive. The attacker sees the compromised host
and the credentials it has obtained; the defender sees alerts and logs. Neither
sees the other's full state. Each forms a belief about the other from partial
observation and acts on that belief. This is exactly the partially observable
stochastic game of Axiom A7.

The master equation then expresses the defender's goal: choose a policy that
maximizes its own cumulative utility under the *worst-case* attacker policy.
This is not pessimism for its own sake; it is the recognition that a policy
optimized against a passive adversary will be exploited by an adaptive one.
Chapter 15 develops the adaptive side explicitly, and Chapter 26 replaces the
expectation with a minimax criterion when the attacker's model is unknown.

## 8.5 Consequences

The two-agent formulation has two immediate consequences.

First, *detection and response are coupled*. The defender's detection policy
determines what the attacker learns about the defender, and therefore what the
attacker does next. A detector that is too eager teaches the attacker which
behaviors are observed; a detector that is too quiet fails to respond. The
master equation, not a standalone detection rule, is the correct objective.

Second, *robustness is a game-theoretic property*. A policy that is optimal
against a fixed adversary is not robust; a policy that is optimal under maximin
is, by construction, robust to the adversary's adaptation. This is the
justification for the minimax criterion throughout the framework.

## 8.6 What fails without A7

Without the adversary there is no game, no master equation, and no adversarial
reasoning. The framework would treat the attacker as a stationary distribution,
which is precisely the assumption that fails under adversarial adaptation. The
zero-day chapters of Part III, the adaptation of Chapter 15, and the minimax
response of Chapter 26 all presuppose A7.
