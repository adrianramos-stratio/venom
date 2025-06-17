## 🧠 Aggregate Definition: `ExecutionContext`

### 🏷 Aggregate Name

> `ExecutionContext` (Runtime configuration and policy scope for a deployed component)

### 🎯 Purpose

> Represent the environment-specific runtime configuration where a given `Component` is deployed. Encapsulates metadata that affects vulnerability relevance and classification, including namespaces, policies, network exposure, and user roles.

### 🧱 Aggregate Identity

> `ExecutionContextId = component_id + namespace + environment`

---

## 📩 Accepted Commands

|Command|Description|Preconditions|
|---|---|---|
|`DefineExecutionContext`|Create a new execution context for a given component|Component must exist|
|`UpdateExecutionContext`|Modify the runtime metadata of the context|Context must already exist|
|`DeprecateExecutionContext`|Mark the context as deprecated and exclude from evaluation|Context must be active|

---

## 📤 Emitted Events

|Event|Description|Invariant Guaranteed|
|---|---|---|
|`ExecutionContextDefined`|A new context has been declared|Component is known and metadata is valid|
|`ExecutionContextUpdated`|The runtime configuration has changed|Context existed and changes are consistent|
|`ExecutionContextDeprecated`|Context has been deprecated and excluded from future analysis|Context was previously valid and in use|

---

## ⚖️ Invariants Enforced

- Each context must reference an existing and SBOM-assigned `Component`
    
- A `Component` can have multiple execution contexts
    
- Contexts must define at minimum: namespace, network profile, isolation/policy set
    
- Contexts cannot overlap in identity (same component + same env + same namespace)
    

---

## 🔄 Lifecycle and States

|State|Description|Possible Transitions|
|---|---|---|
|`Declared`|Initial creation with full metadata|→ `Updated`, `Deprecated`|
|`Updated`|Runtime parameters changed|→ `Deprecated`|
|`Deprecated`|Context is frozen and excluded from analysis|Terminal state|

---

## 🔗 External Dependencies

- **Component Inventory Context**: ensures component identity and SBOM availability
- **Policy Engine (e.g., Kyverno, Calico)**: source of applicable runtime constraints
- **BindingOrchestratorActor**: listens to context changes to trigger reclassification

---

## 💡 Additional Notes

- `ExecutionContext` is a reactive aggregate with runtime-specific impact
- Does not own the component or vulnerabilities; acts as qualifying context for `Binding`
- Multiple execution contexts can exist per component and must be evaluated independently
- Updates to this context can result in vulnerability reclassification in bindings
    

---