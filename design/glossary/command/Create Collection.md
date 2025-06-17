---
description: Creates a logical collection of components based on business, release, or deployment criteria. Each collection is immutable and must contain only valid, non-deprecated components.
tags:
  - command
target_aggregate: "[[glossary/aggregate/Collection|Collection]]"
preconditions:
  - The collection ID must be unique.
  - All component IDs in the collection must refer to existing components.
  - No component in the collection may be deprecated.
---
