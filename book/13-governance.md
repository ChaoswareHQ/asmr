# Chapter 13 — Governance and Policy (Axiom A12)

> *Part I — Foundations*

## 13.1 Motivation

Security actions have legal, contractual, and operational consequences. A
framework that does not distinguish "the system may act on its own" from "the
system must obtain a human's approval" cannot be deployed, because some actions
are simply not permitted to be automated. This chapter formalizes autonomy and
policy.

## 13.2 Axiom

**Axiom A12 (Governance and policy).** Actions carry autonomy levels, and
policy constrains which actions the system may take.

The autonomy levels form a finite set

$$
L  =  \lbrace \texttt{Prohibited},\ \texttt{HumanApproved},\ \texttt{Autonomous} \rbrace.
$$

## 13.3 Operations

**Governed action space.** The set of actions the defender may consider is the
set not prohibited by policy:

$$
A_D^{\mathrm{gov}}  =  \lbrace  a \in A_D \mid \ell(a) \ne \texttt{Prohibited}  \rbrace.
$$

**Governed policy.** The defender's policy is constrained to this set:

$$
\mu_D^{\mathrm{gov}} : I_t^D \to A_D^{\mathrm{gov}}.
$$

**Allowed predicate.** An action may be executed exactly when it is autonomous,
or it is human-approved and a human has approved it:

$$
\mathrm{Allowed}(a)  =  \bigl(\ell(a) = \texttt{Autonomous}\bigr)  \lor  \bigl(\ell(a) = \texttt{HumanApproved} \wedge \mathrm{approved}(a)\bigr).
$$

## 13.4 Worked example

Consider a bank's policy:

| Action | Autonomy | Approver |
|---|---|---|
| `block_ip` | Autonomous | — |
| `isolate_host` | HumanApproved | SOC lead |
| `isolate_dc` | HumanApproved | CISO |
| `delete_data` | Prohibited | Never |
| `disable_av` | Prohibited | Never |

At a state where a host shows a C2 connection, the governed action set consists
of `block_ip`, `alert`, and `collect_forensics`.

The actions `isolate_host`, `isolate_dc`, `delete_data`, and `disable_av` are
excluded — the first two because they require human approval that has not been
given, the last two because they are prohibited outright. The system may block
the IP autonomously, but it may not isolate a host without a human, and it may
never delete data or disable antivirus.

## 13.5 Consequences

Governance is the bridge between the optimal decision of Chapter 9 and the
legally and operationally permissible decision. It has two effects. First, it
*restricts*: the cost-minimizing action may be prohibited, in which case the
optimal *permissible* action is chosen instead. Second, it *delegates*: actions
of intermediate autonomy are escalated to a human, which introduces a latency
and a bottleneck that the framework must account for — an action that is
"optimal but pending approval" is, for the duration of the approval, no action
at all.

Governance also interacts with trust (Chapter 11). An action may be autonomous
in policy yet barred in practice because the trust in the evidence is below
threshold. The conjunction of the two — policy autonomy and evidential trust —
determines what the system may actually do.

## 13.6 What fails without A12

Without governance there is no compliance, no legal protection, and no human
oversight. A framework that treats all actions as equally automatable will,
sooner or later, automate an action that must not be automated. The
case studies of Part VI — in particular the decision to isolate in Chapter 36
and the freeze response in Chapter 31 — presuppose A12.
