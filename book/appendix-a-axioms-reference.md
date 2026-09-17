# Appendix A — Reference of Axioms

This appendix states all thirty axioms without commentary, as a reference. The
axioms are numbered linearly, A1 through A30, in the order of construction:
the core axioms A1–A12, the extended axioms A13–A20, and the zero-day axioms
A21–A30. The numbering is the build order — each axiom builds on those that
precede it.

## A.1 Core axioms

**A1 (State product).**
$S = \prod_{i \in I} S_i$, with projection $\pi_J : S \to \prod_{i \in J} S_i$,
fiber $\pi^{-1}(o)$, and gap
$\Delta = \{\, s \mid |\pi^{-1}(\pi(s))| > 1 \,\}$.

**A2 (Event transition).** An event is a sparse partial function
$e : S \to S$; traces compose under $(\,;\,)$, $+$, and $*$, with identity
$\mathrm{id}$ and abort $0$.

**A3 (Observation projection).**
$o_t = \eta_t(\pi(s_t), \theta_t)$.

**A4 (Temporal–causal structure).** Local time is a total order
$(T_h, \le_h)$; global causality is a partial order $\prec$, witnessed by vector
clocks.

**A5 (Uncertainty model).** There exists a probability space
$(\Omega, \mathcal{F}, P)$, or a robust set $\mathcal{U}$.

**A6 (Action space).** There exists a set $A$ of actions
$a = (\mathrm{pre}_a, \mathrm{eff}_a, C_a, \ell_a)$, with applicable set $A(s)$.

**A7 (Multi-agent structure).** There exist a defender $D$ and attacker $A$
with policies $\mu_D, \mu_A$ over information sets, forming a partially
observable stochastic game.

**A8 (Cost–utility model).** Each action has costs $C_M(a), C_{\neg M}(a)$;
expected cost is $p\, C_M + (1-p)\, C_{\neg M}$.

**A9 (Information sets).**
$I_t^D = (o_0, a_0, \ldots, o_t)$, with recursive update
$P(s_t \mid I_t^D) \propto P(o_t \mid s_t)\, P(s_t \mid I_{t-1}^D)$.

**A10 (Trust and integrity).** Observations carry trust $\tau \in T$, with
propagation $\tau(s_1 \circ s_2) = \tau(s_1) \odot \tau(s_2) \odot \kappa(b_{12})$
and trust-weighted likelihood
$P(o \mid s, \tau) = \tau P_{\mathrm{honest}} + (1-\tau) P_{\mathrm{adversarial}}$.

**A11 (Resource constraints).** $R(t) \le C$, with resource-aware action set
$A_R(s, t) = \{\, a \in A(s) \mid \mathrm{req}(a) \le C - R(t) \,\}$.

**A12 (Governance and policy).** Autonomy levels
$L = \{\texttt{Prohibited}, \texttt{HumanApproved}, \texttt{Autonomous}\}$,
governed action space $A_D^{\mathrm{gov}}$, and
$\mathrm{Allowed}(a)$.

## A.2 Extended axioms

**A13 (Composition algebra).** An associative composition
$\circ : S_i \times S_j \to S_{i \cup j}$, reachability $\mathrm{Reach}(c)$, and
blast radius $\mathrm{BR}(c) = \sum \mathrm{Impact}(s_j)\, \tau(s_j)$.

**A14 (Adversarial adaptation).**
$\mu_A^{t+1} = L_A(\mu_A^t, I_t^A, r_t^A)$, with drift measured by
$D_{\mathrm{KL}}$.

**A15 (Observation channel capacity).**
$C = \max_{P(s)} I(S; O)$, with the Fano bound on error.

**A16 (Temporal decay).**
$w(t, t') = e^{-\lambda (t - t')}$, half-life $t_{1/2} = \ln 2 / \lambda$.

**A17 (Deception and active defense).** A deception operator
$\delta : S \to S$, with gain
$G_{\mathrm{dec}} = I(s_{\mathrm{true}}; o_D) - I(s_{\mathrm{true}}; o_A)$.

**A18 (Multi-scale hierarchy).**
$S^{(1)} \subset S^{(2)} \subset \cdots \subset S^{(L)}$, with abstraction
$\alpha_l$, refinement $\rho_l$, and information loss
$\Delta_l = H(S^{(l)}) - H(S^{(l+1)})$.

**A19 (Privacy and data minimization).** A privacy operator
$\Pi : S \to S_{\mathrm{safe}}$ satisfying differential privacy, with utility
loss $L_{\mathrm{util}} = H(S) - H(S_{\mathrm{safe}})$.

**A20 (Recovery and resilience).** A recovery operator
$\rho : S_{\mathrm{compromised}} \to S_{\mathrm{restored}}$, with
$T_{\mathrm{rec}}$, $\mathrm{Res}(s)$, $\mathrm{MTTR}$, and availability
$A = \mathrm{MTBF} / (\mathrm{MTBF} + \mathrm{MTTR})$.

## A.3 Zero-day axioms

**A21 (Open state space).** $S(t) = S_{\mathrm{known}}(t) \cup S_{\mathrm{unknown}}(t)$,
with Good–Turing estimate $P(s_{\mathrm{new}}) = N_1 / N$.

**A22 (Novelty measurement).**
$\mathrm{Novel}(s) = \min_{s' \in S_{\mathrm{known}}} d(s, s')$.

**A23 (Structural anomaly).**
$\mathrm{Anomaly}(s) = \alpha\, \mathrm{ValueAnomaly}(s) + \beta\, \mathrm{StructureAnomaly}(s)$.

**A24 (Epistemic uncertainty).**
$H(Y \mid X) = H_{\mathrm{aleatoric}} + H_{\mathrm{epistemic}}$.

**A25 (Minimax response).**
$a^{*} = \arg\min_a \max_\theta \mathrm{Regret}(a, \theta)$.

**A26 (Patch latency).** $T_{\mathrm{patch}} \sim \mathrm{LogNormal}(\mu, \sigma^2)$,
with $T_{\mathrm{patch}} = \infty$ for a zero-day.

**A27 (Compensating controls).**
$\mathrm{Compensate}(v) = \{\, a \mid \mathrm{reduces}(a, v) \wedge \neg \mathrm{patch}(v) \,\}$.

**A28 (Zero-day hunting).**
$\mathrm{Hunt}(h) = \{\, e \mid \mathrm{StructureAnomaly}(e) > \theta \,\}$.

**A29 (Transfer learning).**
$P(M_{\mathrm{new}} \mid o) = \sum_{M_{\mathrm{old}}} P(M_{\mathrm{new}} \mid M_{\mathrm{old}})\, P(M_{\mathrm{old}} \mid o)$.

**A30 (Freeze and isolate).** Freeze operator $\mathrm{Freeze}(s) = s_{\mathrm{frozen}}$
with $\mathrm{eff}_a = \mathrm{id}$, isolation operator
$\mathrm{Isolate}(s)$, and containment $\mathrm{Contain}(c)$.
