## 🧠 Aggregate Definition: `Collection`

### 🏷 Aggregate Name

> `Collection` (Business-driven grouping of components)

### 🎯 Purpose

> Represent a logical grouping of components according to business needs (e.g., product lines, environments, releases, or risk scopes). Enables coordinated analysis, alerting, and reporting across multiple components.

### 🧱 Aggregate Identity

> `CollectionId = collection_name` (must be globally unique)

---

## 📩 Accepted Commands

|Command|Description|Preconditions|
|---|---|---|
|`CreateCollection`|Create a new, empty collection|Name must not exist|
|`AddComponentToCollection`|Add an existing component to the collection|Component must exist|
|`RemoveComponentFromCollection`|Remove a component from the collection|Component must already be included|
|`InitiateCollectionAnalysis`|Start a coordinated analysis of all components|At least one component must be present|

---

## 📤 Emitted Events

|Event|Description|Invariant Guaranteed|
|---|---|---|
|`CollectionCreated`|A new collection has been registered|Unique name enforced|
|`ComponentAddedToCollection`|A component was linked to the collection|Component exists and was not yet included|
|`ComponentRemovedFromCollection`|A component was removed from the collection|Component was previously linked|
|`CollectionAnalysisInitiated`|Analysis process has started over collection members|Non-empty membership validated|

---

## ⚖️ Invariants Enforced

- `CollectionId` must be unique system-wide
- Cannot include nonexistent components
- Cannot contain the same component more than once
- Analysis can only be initiated if the collection is non-empty
    

---

## 🔄 Lifecycle and States

| State        | Description                                       | Possible Transitions       |
| ------------ | ------------------------------------------------- | -------------------------- |
| `Initial`    | Collection created but still empty                | → `Active`                 |
| `Active`     | Contains one or more components, ready to analyze | → `InAnalysis`, `Archived` |
| `InAnalysis` | Ongoing bulk analysis on collection               | → `Active`                 |
| `Archived`   | Collection is closed and immutable                | Terminal state             |

---

## 🔗 External Dependencies

- **Component Inventory Context**: to validate that each added component exists
- **Collection Analysis Engine**: orchestrates parallel workflows on component subsets
- **BindingContext**: receives or emits data related to component vulnerability bindings

---

## 💡 Additional Notes

- The collection aggregate does not control component lifecycle or security state
- Provides a functional projection of the system for business, regulatory, or operational purposes
- May include business rules to prevent redundant analysis (e.g., minimum analysis interval)
- Can be integrated with CI/CD or release automation as part of larger workflows

---