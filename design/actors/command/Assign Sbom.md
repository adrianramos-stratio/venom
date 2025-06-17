---
description: Attaches a Software Bill of Materials (SBOM) to a component, providing a complete, immutable list of its dependencies. This operation must occur only once and serves as the foundation for vulnerability scanning.
tags:
  - command
target_aggregate: "[[glossary/aggregate/Component|Component]]"
preconditions:
  - The component must be in the Registered state.
  - No SBOM must have been assigned previously.
---
