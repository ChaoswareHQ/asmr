# Chapter 41 — Comparison with Other Frameworks

> *Part VIII — Assessment*

## 41.1 Basis of comparison

The comparison below is not a ranking but a mapping of scope. Each framework
listed addresses part of the problem of Chapter 1; none addresses the whole.
The columns record the kind of object each framework is, the scope it covers,
its chief strength, and its chief limitation.

| Framework | Kind | Scope | Strength | Limitation |
|---|---|---|---|---|
| ASMR | Mathematics | The whole pipeline | Rigor and unity | Complexity |
| MITRE ATT&CK | Taxonomy | Adversary behavior | Adoption and shared vocabulary | No mathematics; no response model |
| NIST CSF | Management framework | Organization | Usability | Deliberately non-technical |
| Zero Trust | Architecture | Network | Simplicity | Narrow scope |
| Game theory | Mathematics | Adversarial interaction | Depth | Narrow scope; no operations |
| Formal methods | Proof | Protocols | Rigor | Narrow scope; no monitoring |

## 41.2 What ASMR adds

The distinctive claim of ASMR is *unity*, and the comparison makes it concrete.

Against **MITRE ATT&CK**, ASMR is complementary rather than rival. ATT&CK names
the adversary's techniques; ASMR gives those techniques a home in a state space
and a trace. A MITRE technique identifier is a label for a particular event in
the sense of Chapter 3, and a kill chain is a trace. ASMR does not replace
ATT&CK; it supplies the mathematics that ATT&CK deliberately omits.

Against **NIST CSF**, ASMR is complementary in the same way. The CSF organizes
governance at the level of an organization; ASMR's governance axiom (Chapter 13)
is the technical instantiation of that organization's policy — the autonomy
levels and approval gates that make a CSF control executable by a system.

Against **Zero Trust**, ASMR provides the *reason* for the architecture. Zero
Trust asserts that no component should be trusted by default; ASMR's trust axiom
(Chapter 10) and composition algebra (Chapter 14) quantify that principle, by
tracking trust as a value that propagates through boundaries and by computing
the blast radius that segmentation reduces.

Against **game theory**, ASMR is an application rather than a competitor. The
game of Chapter 8 and its master equation (Chapter 32) are standard
game-theoretic objects; ASMR's contribution is to *embed* them in the
monitoring pipeline, so that the game is played over observations and
governed actions rather than in the abstract.

Against **formal methods**, ASMR shares the commitment to rigor but applies it
to a different object. Formal methods verify protocols; ASMR analyzes the
monitoring and response process itself. The two are compatible: a formally
verified protocol is one component whose state space and transitions are
particularly well-characterized, and it can be dropped into ASMR as a factor.

## 41.3 What ASMR does not claim

The comparison is not an argument that ASMR is *better* than the alternatives in
every respect. It is an argument that ASMR occupies a different position: it is
the framework that connects them. The cost of that position is complexity
(Chapter 42), and the value of that position is that a single set of axioms
can state, in one language, what the other frameworks state in several.
