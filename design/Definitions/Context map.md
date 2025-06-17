## 🗺️ Context Map — Vulnerability Management System

This map visualizes the bounded contexts and their relationships within the system, based on the principles of Domain-Driven Design (DDD) and reactive architecture.

---

### 🧱 Bounded Contexts

1. **Component Inventory Context**
    
    - Governs the registration and immutability of components (Docker images)
        
    - Aggregate: `Component`
        
    - Owns SBOM assignment and component deprecation
        
2. **Execution Context Context**
    
    - Manages runtime environments where components are deployed
        
    - Aggregate: `ExecutionContext`
        
    - Defines namespace, ports, security policies, etc.
        
3. **Vulnerability Catalog Context**
    
    - Maintains a local mirror of CVEs from external sources (Grype, NVD)
        
    - Entity: `Vulnerability`
        
    - Emits updates for vulnerability changes
        
4. **Binding Context**
    
    - Models the impact of a vulnerability on a component in a specific execution context
        
    - Aggregate: `Binding`
        
    - Applies contextual classification logic
        
5. **Collection Context**
    
    - Groups components for business-driven operations (e.g., releases, products)
        
    - Aggregate: `Collection`
        
    - Triggers batch analysis over component sets
        
6. **Reporting Context**
    
    - Projects read-only views of events and state for audits, alerts, dashboards
        
    - Entities: `Report`, `Snapshot`, `Alert`
        
    - Built through event listeners
        

---

### 🔄 Interactions (Event-Driven Relations)

- `ComponentInventoryContext` emits `ComponentRegistered`, `SbomAssigned` → consumed by:
    
    - `SbomGeneratorActor`
        
    - `BindingOrchestratorActor`
        
    - `CollectionContext`
        
- `ExecutionContextContext` emits `ExecutionContextDefined`, `ExecutionContextUpdated` → consumed by:
    
    - `BindingOrchestratorActor`
        
- `VulnerabilityCatalogContext` emits `VulnerabilityUpdated`, `VulnerabilityRegistered` → consumed by:
    
    - `BindingOrchestratorActor`
        
- `BindingOrchestratorActor` triggers commands on `BindingContext`
    
- `CollectionContext` consumes component events for validation and triggers collection-wide analysis
    
- `ReportingContext` consumes all public events to build projections
    

---

### 🧭 Notes on Design

- **No shared aggregates or synchronous calls** across contexts
    
- **Integration is asynchronous**, using domain events and actors
    
- **Process logic lives outside domain** in orchestrators like `BindingOrchestratorActor`
    
- **External systems** (e.g., Grype, CI/CD) integrate via ingestion or triggering commands
    

---