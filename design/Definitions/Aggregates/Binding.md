## 🧠 Aggregate Definition: `Binding`

### 🏷 Aggregate Name

> `Binding` (Link between vulnerability, component, and execution context)

### 🎯 Purpose

> Represent and manage the contextualized application of a vulnerability (by CVE) to a specific versioned component, deployed in a particular execution context. Encapsulates classification logic, reacts to external changes (vulnerability updates, execution context shifts), and supports exception tracking.

### 🧱 Aggregate Identity

> `BindingId = component_id + execution_context_id + cve_id`

---

## 📩 Accepted Commands

| Command             | Description                                                   | Preconditions                                    |
| ------------------- | ------------------------------------------------------------- | ------------------------------------------------ |
| `RegisterBinding`   | Create a new binding from component, CVE and context          | The binding must not already exist               |
| `ReclassifyBinding` | Recalculate effective classification based on updated context | Valid CVSS vector and execution context present  |
| `AnnotateException` | Mark a binding as ignored or mitigated with justification     | The binding exists and justification is provided |

---

## 📤 Emitted Events

| Event                 | Description                                                | Invariant Guaranteed                             |
| --------------------- | ---------------------------------------------------------- | ------------------------------------------------ |
| `BindingRegistered`   | A contextual vulnerability-component binding was created   | Valid CVE, Component, and ExecutionContext exist |
| `BindingReclassified` | Classification has been re-evaluated based on new input    | Classification applied using rules and context   |
| `ExceptionDocumented` | Binding was marked mitigated or ignored with justification | Valid business rationale exists                  |

---

## ⚖️ Invariants Enforced

- A `Binding` must be uniquely defined by `component_id`, `execution_context_id`, and `cve_id`
- Classification cannot exist without a valid base CVSS vector (v3 or v4)
- Reclassification is only allowed when contextual data is available
- A CVE may not be bound multiple times to the same component + context

---

## 🔄 Lifecycle and States

| State        | Description                                   | Possible Transitions                         |
| ------------ | --------------------------------------------- | -------------------------------------------- |
| `Pending`    | Registered but not yet classified             | → `Classified`                               |
| `Classified` | Contextual severity classification is active  | → `Reclassified`, `Mitigated`, `Ignored`     |
| `Mitigated`  | Marked as mitigated with justification        | → `Reclassified` (if context or CVE changes) |
| `Ignored`    | Explicitly ignored based on business decision | → `Reclassified`                             |

---

## 🔗 External Dependencies

- **Vulnerability Catalog**: provides current CVSS vectors and metadata per CVE    
- **Execution Context Context**: defines the runtime configuration, exposure, criticality, policies
- **Component Inventory**: defines component identity, immutability, and SBOM
- **Classification Service**: applies the business rules for contextual risk evaluation

---

## 💡 Additional Notes

- `Binding` is a standalone aggregate to ensure concurrency, traceability, and isolated consistency.
- It may be instantiated reactively by a `BindingOrchestratorActor`, upon receipt of events from other contexts.
- Maintains complete trace of decisions, classifications, and exceptions.
- The context is essential: the same component and CVE may have multiple bindings across different environments.

---