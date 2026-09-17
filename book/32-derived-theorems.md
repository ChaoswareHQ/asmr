# Chapter 32 — Derived Theorems

> *Part IV — Theorems*

## 32.1 Scope of this chapter

The axioms of Parts I–III imply a number of theorems. This chapter collects the
central ones. Two are proven in full (Theorems 1 and 2); two are standard
results stated with their standard proofs (Theorems 3 and 4); two are bounds
whose arguments are sketched (Theorems 5 and 6). A theorem stated with a sketch
is marked as such, so that the reader may distinguish the fully proven from the
provisional.

---

## 32.2 Theorem 1 (Gap characterization)

**Statement.** A state lies in the gap if and only if some distinct state has
the same projection:

$$
s \in \Delta  \Longleftrightarrow  \exists  s' \ne s : \pi(s') = \pi(s).
$$

**Proof.** The definition of the gap is
the set of states whose projection has more than one preimage.

(If.) If there exists some other state with the same projection as the state, then
both states lie in the same fiber, so
that fiber contains at least two states. Hence the state lies in the gap.

(Only if.) If the state lies in the gap, then its projection's fiber contains more than one state, so
the fiber contains some element other than the state; call it the distinct other state. By definition of the
fiber, that other state has the same projection.

The two directions together give the equivalence. ∎

This theorem is the formal content of "the gap is exactly the set of states the
projection cannot distinguish." It is the foundation of Chapter 2 and is used
repeatedly in the capacity analysis of Chapters 4 and 16.

---

## 32.3 Theorem 2 (Optimal decision threshold)

**Statement.** For two actions $a$ and $a'$, the posterior probability at which
their expected costs are equal is

$$
p^{*}  =  \frac{C_{\neg M}(a') - C_{\neg M}(a)}{\bigl(C_M(a) - C_M(a')\bigr) + \bigl(C_{\neg M}(a') - C_{\neg M}(a)\bigr)}.
$$

Below the threshold one action is cheaper; above it, the other.

**Proof.** The expected costs are

$$
\mathbb{E}[C(a)] = p  C_M(a) + (1-p)  C_{\neg M}(a),
\qquad
\mathbb{E}[C(a')] = p  C_M(a') + (1-p)  C_{\neg M}(a').
$$

Setting them equal and expanding,

$$
p  C_M(a) + C_{\neg M}(a) - p  C_{\neg M}(a)
 = 
p  C_M(a') + C_{\neg M}(a') - p  C_{\neg M}(a').
$$

Collecting the terms in $p$ on one side and the constants on the other,

$$
p\Bigl[ C_M(a) - C_{\neg M}(a) - C_M(a') + C_{\neg M}(a') \Bigr]
 = 
C_{\neg M}(a') - C_{\neg M}(a).
$$

The bracket simplifies to
the first action's real-case cost minus the second's, plus the second action's not-real-case cost minus the first's,
and solving for the posterior gives the stated threshold. ∎

**Worked instance.** For the actions of Chapter 9, take the block action, whose
cost is 0 when the attack is real and 500 when it is not, and the alert action, whose
cost is 50 when the attack is real and 50 when it is not. Then

$$
p^{*} = \frac{50 - 500}{(0 - 50) + (50 - 500)} = \frac{-450}{-500} = 0.9.
$$

The threshold is derived, not chosen: block only when the posterior exceeds
0.9.

---

## 32.4 Theorem 3 (Trust–capacity tradeoff)

**Statement.** Trust and channel capacity are substitutes: reducing trust in the
observation source raises the floor on detection error.

**Sketch.** The trust-weighted likelihood of Chapter 11 replaces the honest
likelihood with a mixture

$$
P(o \mid s, \tau) = \tau  P_{\mathrm{honest}}(o \mid s) + (1-\tau)  P_{\mathrm{adversarial}}(o \mid s).
$$

As trust approaches one this converges to the honest channel; as trust decreases, the
effective channel is a mixture with the adversarial channel, and its effective capacity
is nondecreasing in trust. Substituting
the effective capacity into the Fano bound of Chapter 16,

$$
P_e  \ge  \frac{H(S) - C_{\mathrm{eff}}(\tau) - 1}{\log_2 |S|},
$$

the error floor rises as trust falls. A convenient first-order approximation
is

$$
P_e  \ge  \frac{H(S) - C - 1}{\log_2 |S|} + \frac{1 - \tau}{2},
$$

which captures the qualitative effect in a closed form. The essential claim —
that trust and capacity substitute for one another, so a defender may trade
richer telemetry against stronger trust, and vice versa — holds independently of
the particular approximation. ∎ (sketch)

**Corollary.** A defender can compensate for an untrusted sensor either by
raising the trust in that sensor or by adding an independent sensor; the two
levers address the same term in the error bound.

---

## 32.5 Theorem 4 (Master equation)

**Statement.** The defender's optimal policy in the partially observable
stochastic game of Chapter 8 is the maximin solution of the discounted
objective:

$$
\mu_D^{*}  =  \arg\max_{\mu_D}\ \min_{\mu_A}\ \mathbb{E}\left[ \sum_{t=0}^{\infty} \gamma^{t}  U_D \right].
$$

**Proof.** For a discounted infinite-horizon stochastic game with finite state
and action spaces, Shapley's theorem guarantees that the value exists and is
the unique solution of the Bellman equation; the maximin value equals the
minimax value under the standard conditions. The Bellman principle then reduces
the infinite-horizon problem to a fixed point, and the optimal policy is
stationary. Applying this to the defender's objective, with the minimax over
the attacker playing the role of the worst case, yields the master equation.
∎

The master equation is the formal statement that detection and response are
coupled: the defender's policy is evaluated against the adversary's *best*
response, not against a fixed adversary.

---

## 32.6 Theorem 5 (Zero-day detection bound)

**Statement.** The probability of detecting a zero-day is bounded above by the
fraction of the state space's entropy that the defender has previously
characterized:

$$
P_d  \le  \frac{H(S_{\mathrm{known}})}{H(S_{\mathrm{known}}) + H(S_{\mathrm{unknown}})}.
$$

**Sketch.** A detector trained on the known portion of the state space can
exploit at most the information contained in that portion. The unknown portion
is, by construction, invisible to it: there is no likelihood
of the observation given an unknown state with which to score it, because the detector
has never seen such states. The detection probability is therefore bounded
above by the known entropy share; as the unknown share grows, the ceiling on
detection falls. The bound is an information-theoretic ceiling, not a statement
about any particular detector. ∎ (sketch)

The direction of the bound is the important content: a larger unknown region
*reduces* the achievable detection probability, which is why the zero-day
chapters of Part III replace detection with novelty, structural anomaly, and
hunting rather than attempting to extend the known detector.

---

## 32.7 Theorem 6 (Freeze optimality)

**Statement.** If novelty exceeds threshold and no patch is available, the
minimax-optimal action is freeze:

$$
\mathrm{Novel}(s) > \theta  \wedge  T_{\mathrm{patch}} = \infty
 \Longrightarrow  a^{*} = \mathrm{Freeze}.
$$

**Proof.** By Chapter 27, the patch time being infinite means the usual remedy
is unavailable. By Chapter 25, high novelty means the defender's uncertainty is
epistemic: no trustworthy probability over the state exists. The minimax
criterion of Chapter 26 then applies. Under maximal epistemic uncertainty, the
action whose worst-case cost is smallest is the action that changes the state
least, because every change risks moving the system further into the unknown;
that action is freeze, whose effect is the identity (Chapter 31). Hence freeze
minimizes worst-case regret and is the minimax-optimal action. ∎

Theorem 6 is the formal bridge between the zero-day axioms and the operational
response of Chapter 31, and it is exercised in the Log4Shell case study of
Chapter 38.
