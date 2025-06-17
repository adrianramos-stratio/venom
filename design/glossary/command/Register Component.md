---
description: Registers a new Docker-based component in the system inventory, uniquely identified by its registry, namespace, name, tag, and digest. This establishes its eligibility for SBOM and vulnerability analysis.
tags:
  - command
target_aggregate: "[[glossary/aggregate/Component|Component]]"
preconditions:
  - The component ID must be globally unique.
  - The component must not already exist in the inventory.
---
