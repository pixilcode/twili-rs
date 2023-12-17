# 2. Use mutability to indicate IO

Date: 2023-12-17

## Status

Accepted

## Context

For the trait interfaces defined in the `command` module, which are currently
`TaskManager` and `Presenter`, only some of the trait methods take borrow `self`
mutably. However, I've found that I've had to keep changing methods from `&self`
to `&mut self` because they interact with I/O systems.

## Decision

For all traits that provide interfaces for systems that interact with the
outside world (such as `TaskManager` and `Presenter`), their methods will always
take `&mut self` as a parameter if `self` is needed, even if `self` doesn't
appear to need to be mutable.

## Consequences

The hope is that any unforseen interaction with the outside world will be
accounted for with the `mut` keyword. This way, we won't need to change it in
the future when the system has become more complex. This will make things such
as parallelization more difficult. However, that is very far in the future, so
we will prioritize current needs and save future optimization/development in
this area for the future.
