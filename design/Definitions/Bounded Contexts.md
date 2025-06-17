## 🔍 Bounded Contexts of the Vulnerability Management System

This document defines the bounded contexts identified in the system, following the principles of Domain-Driven Design and best practices from the project's core references (Vernon, Reactive DDD, Actor Model).

---

### 1. 📦 **Component Inventory Context**

**Responsibility**: Govern the lifecycle of software components (Docker images), including their registration, metadata management, and immutable SBOM assignment.

**Aggregate Root**: `Component`

**Domain Events Published**: `ComponentRegistered`, `SbomAssigned`, `ComponentDeprecated`

**Exposed Interfaces**:

- `POST /components`
    

**Notes**:

- A `Component` represents a static artifact uniquely identified by registry, name, tag, and digest.
    
- SBOMs are immutable and tightly bound to a component version. Updates imply creation of a new component.
    
- This context **does not manage runtime deployment configurations** (ExecutionContexts).
    
- All decisions regarding security or usage are deferred to other contexts.
    

---

### 2. 🌐 **Execution Context Context**

**Responsibility**: Manage the runtime configurations and policies under which components are executed. Each `ExecutionContext` defines the environment for one deployment instance of a component.

**Aggregate Root**: `ExecutionContext`

**Domain Events Published**: `ExecutionContextDefined`, `ExecutionContextUpdated`

**Domain Events Consumed**:

- `ComponentRegistered` (validation that the target component exists)
    

**Notes**:

- A component may be deployed under multiple execution contexts (e.g., different namespaces or environments).
    
- An `ExecutionContext` includes port exposure, RBAC, isolation policies, namespace constraints, and network policies (e.g., Kyverno, Capsule, Calico).
    
- Changes in an `ExecutionContext` are reactive triggers for downstream reclassification of bindings.
    
- No security classification is performed within this context.
    

---

### 3. 🔒 **Vulnerability Catalog Context**

**Responsibility**: Maintain a synchronized cache of authoritative vulnerability information (e.g., CVEs), sourced from external systems such as Grype, NVD, or vendor feeds.

**Entity**: `Vulnerability`

**Service/Actor**: `VulnerabilityCatalog`

**Domain Events Published**: `VulnerabilityRegistered`, `VulnerabilityUpdated`

**Internal Interfaces**:

- Query service by `cve_id`
    

**Notes**:

- This context is **external to the business domain**. It exists solely to propagate CVE data reactively.
    
- It **does not evaluate**, classify or bind vulnerabilities—only reports them as-is.
    
- Designed for high-throughput ingestion and minimal internal modeling.
    

---

### 4. 🔗 **Binding Context**

**Responsibility**: Represent and manage the security relationship between a `Component`, a `Vulnerability`, and a specific `ExecutionContext`. This defines the system’s primary analytical and decision-making unit.

**Aggregate Root**: `Binding`

**Domain Events Published**: `BindingRegistered`, `BindingReclassified`, `ExceptionDocumented`

**Domain Events Consumed**:

- `VulnerabilityUpdated` (from Vulnerability Catalog)
    
- `SbomAssigned` (from Component Inventory)
    
- `ExecutionContextUpdated` (from Execution Context)
    

**Notes**:

- The `Binding` evaluates the _effective impact_ of a vulnerability on a deployed component.
    
- Contextual classification (e.g., CVSS customization, exception registration) lives here.
    
- This context contains the core business logic for reactive security intelligence.
    
- Does not interact directly with component lifecycle or CVE ingestion.
    

---

### 5. 📋 **Collection Context**

**Responsibility**: Organize components into functional or organizational groups (e.g., product lines, systems, teams) to enable scoped analysis and bulk operations.

**Aggregate Root**: `Collection`

**Domain Events Published**: `CollectionCreated`, `ComponentAddedToCollection`, `CollectionAnalysisInitiated`

**Domain Events Consumed**:

- `ComponentRegistered` (to validate existence)
    

**Notes**:

- Collections have no ownership or state over the components they contain.
    
- They serve business, compliance, and operational segmentation purposes.
    
- Collection-based analysis may trigger mass binding evaluations but does not alter underlying data.
    

---

### 6. 📈 **Reporting Context**

**Responsibility**: Project, materialize, and expose aggregated data from other contexts. Provides dashboards, audit trails, alerts, compliance snapshots, and external reporting.

**Derived Entities**: `Report`, `Alert`, `Snapshot`

**Domain Events Consumed**: All public domain events across bounded contexts

**Notes**:

- Read-only and stateless in terms of business logic.
    
- Implemented using projections, listeners, and eventual consistency patterns.
    
- Designed for performance, traceability, and business observability.
    

---

### 🎨 Modeling Principles Applied

- Each bounded context defines a strict boundary of responsibility and consistency.
    
- No business logic is shared across contexts.
    
- All coordination occurs through **domain events and reactive orchestration**.
    
- Aggregates are isolated, rule-driven, and encapsulate their own invariants.
    
- Application services and process managers live **outside** these contexts.
    

---