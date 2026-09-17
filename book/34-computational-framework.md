# Chapter 34 — The Computational Framework

> *Part V — The Pipeline and Computation*

## 34.1 Purpose

The axioms specify structures; computation makes them executable. This chapter
specifies the computational interface of the framework — the classes and
methods that realize the stages of the pipeline of Chapter 33 — and records
the reference computation that produces the worked examples.

The interface is specified here as an API. The reference computation for every
numeric example in this book is the Rust program committed at
`tools/compute_examples.rs`; it is dependency-free and reproduced with

```sh
rustc tools/compute_examples.rs -O -o compute_examples
./compute_examples
```

The API below is the natural object-oriented reading of the same operations.
Where the book quotes a number, that number is the output of the Rust program,
rounded for display only.

## 34.2 `State`

The state space of Axiom A1.

```python
S = asmr.State({
    "factor1": ["value1", "value2"],
    "factor2": ["value3", "value4"],
})

S.size                  # total number of states          (A1)
S.project(["factor1"])  # observation map                (A1, A3)
S.fiber(observation)    # states consistent with o       (A1)
S.gap()                 # gap ratio                      (A1)
S.sample()              # a random state                 (A1)
S.enumerate()           # iterate all states             (A1)
```

`S.size` is the product of the sizes of the factors; `S.fiber(o)` is the fiber of the observation under the projection; `S.gap()` is
the size of the gap divided by the size of the state space, as defined in Chapter 2.

## 34.3 `Trust`

The trust structure of Axiom A10.

```python
tau = asmr.Trust({"source1": 0.9, "source2": 0.7})

tau.combine(other)                 # composition           (A13)
tau.weighted_likelihood(P_honest, P_adversarial)            (A10)
tau.threshold(action)              # trust gate            (A10)
tau.decay(lam, dt)                 # temporal decay        (A16)
```

`tau.weighted_likelihood` implements the mixture of Chapter 11; `tau.combine`
implements the boundary-weighted composition of Chapter 14.

## 34.4 `Action`

The action structure of Axiom A6.

```python
a = asmr.Action(
    name="block_ip",
    pre=lambda s: s["ip"] in c2_list,
    eff=lambda s: s.update({"net": "no_conn"}),
    cost_M=0,
    cost_notM=500,
    autonomy="autonomous",
)

a.applicable(s)      # precondition          (A6)
a.execute(s)         # effect                (A6)
a.expected_cost(p)   # p*C_M + (1-p)*C_notM  (A8)
```

## 34.5 `Resource`

The resource structure of Axiom A11.

```python
R = asmr.Resource({"cpu": 64, "memory": 256, "analyst_hours": 160})

R.available          # remaining capacity    (A11)
R.consume(amount)    # deduct                (A11)
R.feasible(action)   # fits remaining?       (A11)
R.triage(alerts)     # rank by expected value (A11, A8)
```

## 34.6 `Policy`

The governance structure of Axiom A12.

```python
p = asmr.Policy({
    "block_ip": "autonomous",
    "isolate":  "human_approved",
    "delete":   "prohibited",
})

p.allowed(action)    # autonomy test         (A12)
p.enforce(action)    # apply policy          (A12)
p.escalate(action)   # request human approval (A12)
```

## 34.7 `Pipeline`

The assembled loop of Chapter 33.

```python
pipeline = asmr.Pipeline(S, pi, tau, A, R, policy)

result = pipeline.run(observations=["no_conn", "c2_conn", "c2_conn"])

result.posterior     # P(M | I)              (A5, A10, A16)
result.action        # optimal action        (A8)
result.cost          # expected cost         (A8)
result.governance    # governance decision   (A12)
result.gap           # gap ratio             (A1)
result.coverage      # coverage              (A3)
result.mttr          # estimated MTTR        (A20)
result.trace         # full trace            (A2)
```

`run` executes the pipeline stage by stage, applying projection, update,
inference, decision, governance, and effect in the order of Chapter 33.

## 34.8 `ZeroDay`

The zero-day operations of Part III.

```python
zd = asmr.ZeroDay(S_known)

zd.novelty(state)             # novelty score           (A22)
zd.structure_anomaly(state)   # structural anomaly      (A23)
zd.epistemic_uncertainty()    # epistemic uncertainty   (A24)
zd.minimax_action(A)          # minimax response        (A25)
zd.compensating_controls(v)   # workarounds             (A27)
zd.hunt(hypothesis)           # zero-day hunting        (A28)
zd.transfer(P_old)            # transfer learning       (A29)
zd.freeze(state)              # freeze system           (A30)
```

## 34.9 Implementation note

The API is specified, not shipped, in this edition. The distinction matters:
the *calculations* of this book are fully implemented and reproducible through
the Rust program, but the *interface* above is a specification of the objects
that realize the axioms, and its implementation is part of the roadmap of
Chapter 43. The reader who wants to verify a number should run the Rust
program; the reader who wants to build on the framework should implement the
API against the definitions in this chapter and Appendix A.
