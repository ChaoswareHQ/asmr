# Chapter 15 — Adversarial Adaptation (Axiom A14)

> *Part II — Extended Axioms*

## 15.1 Motivation

A static rule is a signal. Once the adversary learns that PowerShell is
blocked, it moves to WMI; once WMI is blocked, it moves to .NET; and so on. A
framework that treats the adversary as fixed will be correct only until the
adversary changes. This chapter formalizes adaptation.

## 15.2 Axiom

**Axiom A14 (Adversarial adaptation).** The attacker's policy evolves in
response to what it observes and what it earns:

$$
\mu_A^{t+1} \;=\; L_A\bigl(\mu_A^{t},\, I_t^A,\, r_t^A\bigr),
$$

where the learning operator is the attacker's update rule, the information is
what it observes, and the reward is what it earns at time t.

## 15.3 Operations

**Concept drift.** The distance between the attacker's behavior at two times is
measured by the KL divergence of the induced state–action distributions:

$$
D_{\mathrm{KL}}(P_t \parallel P_{t'}) \;=\; \sum_{s, a} P_t(s, a) \log \frac{P_t(s, a)}{P_{t'}(s, a)}.
$$

A large value signals that the defender's model of the attacker has gone stale.

**Regret.** The defender's regret over a horizon T is the accumulated
shortfall from the best fixed action in hindsight:

$$
R_T \;=\; \sum_{t=1}^{T} \Bigl(\max_a U(a, s_t) - U(a_t, s_t)\Bigr).
$$

A policy with sublinear regret is, asymptotically, as good as the best fixed
action in hindsight; this is the standard yardstick for learning under an
adversary.

## 15.4 Worked example

The cat-and-mouse dynamic is familiar in practice:

| Round | Attacker | Defender | Attacker's adaptation |
|---|---|---|---|
| 1 | PowerShell | Block PowerShell | Use WMI |
| 2 | WMI | Block WMI | Use .NET |
| 3 | .NET | Block .NET | Use signed binaries |
| 4 | Living-off-the-land | Behavioral detection | Encrypt C2 |

At each round the defender's response becomes part of the attacker's
observation, and the attacker changes its policy accordingly. This is not a
sequence of independent events; it is a single trajectory of the game of
Chapter 8, with the attacker's learning operator moving its policy at
each step. The defender that optimizes only against round 1 will be surprised
by round 2.

The quantitative signature of this process is drift. If the defender has
estimated a distribution over attacker state–action pairs, and the
attacker shifts to WMI, the KL divergence between the old and new
distributions rises; monitoring that rise is
the defender's warning that its model is stale.

## 15.5 Consequences

Adaptation is why robustness, not just optimality, is the correct objective.
A policy optimized against a fixed adversary achieves low loss against that
adversary and unbounded loss against its adaptation; a policy optimized under
maximin (Chapter 8) or minimax regret (Chapter 26) is robust to the shift. This
is the deepest reason the framework uses minimax criteria throughout rather
than point estimates.

Adaptation also motivates *behavioral* detection over signature detection. A
signature is a particular behavior; the adversary can change the behavior. A
behavioral prior — "living off the land, in general" — is harder to evade,
because evading it requires the adversary to abandon an entire class of
behavior, not a single tool. This is the subject of Chapter 24, where anomaly
is measured against structure rather than value.

## 15.6 What fails without A14

Without adaptation there is no arms race and no evolving threat. The framework
would model a static adversary and produce static defenses, which fail exactly
when the adversary moves. The zero-day chapters of Part III, and in particular
the transfer-learning response of Chapter 30, presuppose A14.
