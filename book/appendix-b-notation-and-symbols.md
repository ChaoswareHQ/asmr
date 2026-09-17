# Appendix B — Notation and Symbols

This appendix collects the notation used throughout the book. Symbols are
grouped by the part of the framework they belong to.

## B.1 Sets, spaces, and state

| Symbol | Meaning |
|---|---|
| $I$ | Index set of factors |
| $S$ | State space |
| $S_i$ | Factor space of dimension $i$ |
| $s = (s_i)_{i \in I}$ | A state |
| $O$ | Observation space |
| $|S|$ | Cardinality of $S$ |
| $\pi, \pi_J$ | Projection |
| $\pi^{-1}(o)$ | Fiber of observation $o$ |
| $\Delta$ | Gap (set of ambiguous states) |

## B.2 Events and traces

| Symbol | Meaning |
|---|---|
| $e : S \to S$ | An event |
| $\sigma$ | A trace |
| $\Delta(s, s')$ | Set of factors changed between $s$ and $s'$ |
| $e_1 \,;\, e_2$ | Sequential composition |
| $e_1 + e_2$ | Choice |
| $e^{*}$ | Iteration |
| $\mathrm{id}$ | Identity event |
| $0$ | Abort event |
| $\bot$ | Undefined |

## B.3 Observation and information

| Symbol | Meaning |
|---|---|
| $o_t$ | Observation at time $t$ |
| $\eta_t$ | Sensor noise |
| $\theta_t$ | Observation context |
| $I_t^D$ | Defender's information set at time $t$ |
| $H(X)$ | Entropy of $X$ |
| $H(X \mid Y)$ | Conditional entropy |
| $I(X; Y)$ | Mutual information |
| $C$ | Channel capacity |
| $P_e$ | Error probability |

## B.4 Time and causality

| Symbol | Meaning |
|---|---|
| $(T_h, \le_h)$ | Local time order on host $h$ |
| $\prec$ | Causal precedence |
| $\parallel$ | Concurrency |
| $VC(e)$ | Vector clock of event $e$ |

## B.5 Probability

| Symbol | Meaning |
|---|---|
| $(\Omega, \mathcal{F}, P)$ | Probability space |
| $P(M \mid o)$ | Posterior probability |
| $P(o \mid M)$ | Likelihood |
| $\mathbb{E}[X]$ | Expectation of $X$ |
| $D_{\mathrm{KL}}(P \parallel Q)$ | Kullback–Leibler divergence |
| $\mathcal{U}$ | Robust uncertainty set |
| $p$ | Posterior probability of attack, $P(M \mid I)$ |

## B.6 Actions and decisions

| Symbol | Meaning |
|---|---|
| $A$ | Action space |
| $\mathrm{pre}_a$ | Precondition of action $a$ |
| $\mathrm{eff}_a$ | Effect of action $a$ |
| $A(s)$ | Applicable action set at $s$ |
| $C_M(a)$ | Cost of $a$ under attack |
| $C_{\neg M}(a)$ | Cost of $a$ under no attack |
| $p^{*}$ | Decision threshold |

## B.7 Agents

| Symbol | Meaning |
|---|---|
| $D, A$ | Defender, attacker |
| $\mu_D, \mu_A$ | Defender and attacker policies |
| $\gamma$ | Discount factor |
| $U_D$ | Defender utility |
| $L_A$ | Attacker learning operator |
| $R_T$ | Regret over horizon $T$ |

## B.8 Trust and composition

| Symbol | Meaning |
|---|---|
| $\tau$ | Trust level |
| $T$ | Trust lattice |
| $\odot$ | Trust combination (lattice meet) |
| $\kappa(b)$ | Boundary integrity |
| $\tau_{\min}(a)$ | Trust threshold for action $a$ |
| $\circ$ | Composition operator |
| $\mathrm{Reach}(c)$ | Reachable set from $c$ |
| $\mathrm{BR}(c)$ | Blast radius of $c$ |

## B.9 Resources and governance

| Symbol | Meaning |
|---|---|
| $R(t)$ | Resource vector |
| $C$ | Capacity vector |
| $A_R(s, t)$ | Resource-aware action set |
| $L$ | Autonomy levels |
| $\ell_a$ | Autonomy level of action $a$ |
| $A_D^{\mathrm{gov}}$ | Governed action space |

## B.10 Temporal decay, deception, scale, privacy, recovery

| Symbol | Meaning |
|---|---|
| $w(t, t')$ | Decay weight |
| $\lambda$ | Decay rate |
| $t_{1/2}$ | Half-life |
| $\delta$ | Deception operator |
| $G_{\mathrm{dec}}$ | Deception gain |
| $H_A$ | Attacker confusion entropy |
| $S^{(l)}$ | State space at scale $l$ |
| $\alpha_l, \rho_l$ | Abstraction, refinement |
| $\Delta_l$ | Information loss at scale $l$ |
| $\Pi$ | Privacy operator |
| $\epsilon$ | Differential-privacy parameter |
| $S_{\mathrm{safe}}$ | Minimized state space |
| $L_{\mathrm{util}}$ | Utility loss |
| $\rho$ | Recovery operator |
| $T_{\mathrm{rec}}$ | Recovery time |
| $\mathrm{Res}(s)$ | Resilience |
| $\mathrm{MTTR}, \mathrm{MTBF}$ | Mean time to recovery / between failures |
| $A$ | Availability |

## B.11 Zero-day

| Symbol | Meaning |
|---|---|
| $S_{\mathrm{known}}, S_{\mathrm{unknown}}$ | Known and unknown state subspaces |
| $N_1, N$ | Singletons and total observations (Good–Turing) |
| $\mathrm{Novel}(s)$ | Novelty score |
| $d(s, s')$ | Weighted factor distance |
| $\theta_{\mathrm{novel}}$ | Novelty threshold |
| $\mathrm{ValueAnomaly}, \mathrm{StructureAnomaly}$ | Value and structural anomaly |
| $G_s$ | Relationship graph of state $s$ |
| $H_{\mathrm{aleatoric}}, H_{\mathrm{epistemic}}$ | Aleatoric and epistemic entropy |
| $\mathrm{Regret}(a, \theta)$ | Regret of action $a$ under $\theta$ |
| $T_{\mathrm{patch}}$ | Patch latency |
| $\mathrm{Compensate}(v)$ | Compensating controls for $v$ |
| $\mathrm{Eff}(a, v)$ | Effectiveness of $a$ against $v$ |
| $\mathrm{Hunt}(h)$ | Hunt over hypothesis $h$ |
| $\mathrm{Sim}(M_1, M_2)$ | Jaccard similarity |
| $\mathrm{Freeze}, \mathrm{Isolate}, \mathrm{Contain}$ | Freeze, isolate, contain operators |
