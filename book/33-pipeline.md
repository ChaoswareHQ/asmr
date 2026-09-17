# Chapter 33 — The Monitoring–Response Pipeline

> *Part V — The Pipeline and Computation*

## 33.1 Assembly

The axioms of Parts I–III are not independent curiosities; they assemble into a
single closed loop. This chapter presents the assembly. Each stage of the loop
consumes the output of the previous stage and produces the input of the next,
and each stage is an axiom, or a conjunction of axioms, from the earlier
chapters.

## 33.2 The pipeline

```
                ┌────────────┐
                │   STATE    │   s_t ∈ S                    (A1)
                └─────┬──────┘
                      │  π (projection)
                      ▼
                ┌────────────┐
                │  OBSERVE   │   o_t = η(π(s_t), θ_t)       (A3)
                └─────┬──────┘
                      │  extend history
                      ▼
                ┌────────────┐
                │   INFO     │   I_t = (o_0, a_0, …, o_t)   (A9)
                └─────┬──────┘
                      │  Bayesian / robust update
                      ▼
                ┌────────────┐
                │ POSTERIOR  │   p = P(M | I_t)             (A5, A10)
                └─────┬──────┘
                      │  minimize expected cost
                      ▼
                ┌────────────┐
                │   DECIDE   │   a* = argmin_a E[C(a) | I_t] (A8, A11)
                └─────┬──────┘
                      │  policy filter
                      ▼
                ┌────────────┐
                │  GOVERN    │   a_gov = policy ⊢ a*        (A12)
                └─────┬──────┘
                      │  effect
                      ▼
                ┌────────────┐
                │    ACT     │   s_{t+1} = eff_{a_gov}(s_t) (A6)
                └─────┬──────┘
                      │
                      └──────────────► (loop back to STATE)
```

## 33.3 Stage by stage

| Stage | Operation | Axioms |
|---|---|---|
| State | The system is in a hidden state $s_t \in S$. | A1 |
| Observe | The defender sees a noisy projection $o_t = \eta(\pi(s_t), \theta_t)$. | A3 |
| Info | The observation is appended to the history $I_t$. | A9 |
| Posterior | The history is reduced to a posterior $p = P(M \mid I_t)$, trust-weighted and decay-weighted. | A5, A10, A16 |
| Decide | The posterior selects the action minimizing expected cost, subject to resources. | A8, A11 |
| Govern | The action is filtered by autonomy policy; a prohibited action is blocked, a human-approved action is escalated. | A12 |
| Act | The governed action's effect carries the system to the next state. | A6 |

The loop is the content of the "core insight" of Chapter 1, now fully
axiomatized. Every arrow is a structure that has been defined and studied in
the preceding chapters; there is no unanalyzed step.

## 33.4 The loop is closed

The final arrow returns to the state, which is the point of the exercise: the
defender's action changes the state, and the changed state is observed again.
This closure is what makes the framework a *system* rather than a one-shot
classifier. It also connects to the game of Chapter 8: the state the defender
returns to is not the same state it left, because the adversary has been acting
throughout, and the defender's own action has altered what the adversary
observes and therefore what the adversary does next.

## 33.5 Variations

The loop admits two principled variations.

In the **robust variant**, the posterior stage is replaced by an uncertainty
set and the decide stage by minimax regret (Chapters 25–26). This is the correct
loop when the state space is open and uncertainty is epistemic.

In the **deceptive variant**, the observe stage is augmented by a deception
operator (Chapter 18), so that the adversary's observation of the system is
decoupled from the defender's observation of the adversary.

The core loop of Section 33.2 is the default; the variations are activated by
the zero-day and active-defense regimes respectively.

## 33.6 Computation

Each stage is computable in closed form: projection is coordinate selection,
the Bayesian update is a likelihood product, the decision is the threshold
theorem of Chapter 32, and the governance filter is a set-membership test. The
next chapter specifies the computational interface for these stages, and the
repository commits a reference computation for every worked example.
