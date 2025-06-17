---
description: Assigns a runtime execution context to a component, specifying its operational scope, exposure level, and associated security policies. This operation is only allowed once per component unless replaced.
tags:
  - command
target_aggregate: "[[glossary/aggregate/Component|Component]]"
preconditions:
  - The component must be in the Registered state.
  - The component must not already have an execution context assigned.
---
