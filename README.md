# Results (On my machine)

## These functions return a simple vec directly:

function_that_returns_vec() - 122ns
function_that_returns_single_vec() - 57ns

## These functions return an enum that tries to avoid allocations:

function_that_returns_enum() - 224ns
function_that_returns_only_single_enum() - 76ns