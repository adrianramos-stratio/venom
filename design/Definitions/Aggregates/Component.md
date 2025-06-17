## 🧠 Aggregate Definition: `Component`

### 🏷 Aggregate Name

> `Component` (Docker image managed within the system)

### 🎯 Purpose

> Represent a software component (Docker image) identified by registry, name, tag, and digest. Maintains its immutable SBOM and defines the source of truth for its identity. It is the foundational artifact for downstream binding, execution, and analysis.

### 🧱 Aggregate Identity

> `ComponentId = registry + name + tag + digest`

---

## 📩 Accepted Commands

|Command|Description|Preconditions|
|---|---|---|
|`RegisterComponent`|Registers a new component into the inventory|Must not already exist|
|`AssignSbom`|Assigns a static SBOM document to this component|Component must be registered|
|`DeprecateComponent`|Marks the component as no longer monitored|Component must exist and be active|

---

## 📤 Emitted Events

|Event|Description|Invariant Guaranteed|
|---|---|---|
|`ComponentRegistered`|Component was added to inventory|Unique ID and structural validity ensured|
|`SbomAssigned`|SBOM document has been linked to the component|SBOM integrity and immutability enforced|
|`ComponentDeprecated`|Component was archived and excluded from active flows|Component exists and is no longer mutable|

---

## ⚖️ Invariants Enforced

- A `Component` must be uniquely defined by `(registry, name, tag, digest)`
- SBOMs are immutable and can only be assigned once
- Bindings are only permitted if an SBOM has been assigned
- Once deprecated, a component cannot be modified or reactivated

---

## 🔄 Lifecycle and States

|State|Description|Possible Transitions|
|---|---|---|
|`Initial`|Component is created but has no SBOM assigned|→ `WithSbom`|
|`WithSbom`|Component has a linked SBOM but is not yet analyzed|→ `Deprecated`|
|`Deprecated`|Component is archived, excluded from processing|Terminal state|

---

## 🔗 External Dependencies

- **SBOM Generator**: to produce and assign immutable SBOMs
- **Vulnerability Scanner (e.g., Grype)**: to analyze SBOM and detect CVEs
- **Vulnerability Catalog**: to provide authoritative CVE metadata
- **BindingOrchestratorActor**: listens to `SbomAssigned` to generate bindings

---

## 💡 Additional Notes

- `Component` is a central aggregate in the system, but it remains immutable in content
- Does not manage contextual or runtime deployments (those reside in ExecutionContext)
- Is referenced by collections but not owned by them
- Triggers downstream workflows through event propagation (not synchronous calls)

---