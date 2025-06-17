## 🧭 BindingOrchestratorActor

### 🧠 Propósito

Coordinar la creación y actualización de `Bindings` combinando información proveniente de múltiples bounded contexts: `ComponentInventory`, `ExecutionContext`, y `VulnerabilityCatalog`. Este actor reside en la capa de **aplicación reactiva** y **no pertenece a ningún bounded context** de dominio. Su responsabilidad es exclusivamente técnica y de integración.

---

### 🎯 Responsabilidades

- Escuchar eventos provenientes de:
    - `ComponentInventory`: `ComponentRegistered`, `SbomAssigned`
    - `ExecutionContext`: `ExecutionContextDefined`, `ExecutionContextUpdated`
    - `VulnerabilityCatalog`: `VulnerabilityRegistered`, `VulnerabilityUpdated`
        
- Coordinar combinaciones `Component + Vulnerability + ExecutionContext`
- Emitir comandos al `BindingContext` para:
    - Crear nuevos `Bindings`
    - Reclasificar `Bindings` existentes
        

---

### 🔄 Eventos consumidos y acciones

|Evento|Acción|
|---|---|
|`ComponentRegistered`|Esperar SBOM y contextos. No hace nada por sí solo.|
|`SbomAssigned`|Detectar vulnerabilidades → generar `Binding + Context` candidatos|
|`ExecutionContextDefined`|Aplicar a cada vulnerabilidad del componente → crear bindings|
|`ExecutionContextUpdated`|Reanalizar todos los bindings del componente en ese contexto|
|`VulnerabilityRegistered`|Evaluar afectación en componentes con SBOM → generar bindings|
|`VulnerabilityUpdated`|Reanalizar bindings que referencian esa CVE|

---

### 📦 Comandos emitidos

- `RegisterBinding { component_id, context_id, cve_id }`
- `ReclassifyBinding { binding_id }`
- `DocumentException { binding_id, justification }`
    

---

### 🏗️ Ubicación arquitectónica

- **Capa**: Aplicación reactiva (coordinación)
    
- **Tipo**: Actor supervisor/orquestador
    
- **Modelo**: Stateless o stateful con cache limitada si es necesario
    

---

### 📚 Notas de implementación

- Este actor debe operar sobre un modelo de suscripción persistente a eventos (Kafka, NATS, Akka/Eventuate, etc.)
    
- No debe contener lógica de negocio del dominio (esa reside en el `BindingContext`)
    
- Puede estar desacoplado mediante colas o bus de eventos
    

---

### 🔒 Invariante clave

> “Un binding solo existe si existe la combinación de un componente con un SBOM válido, al menos un contexto de ejecución, y una vulnerabilidad reconocida.”

---