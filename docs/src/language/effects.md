
Effects & Discharge
Every effectful operation returns Computation<T, ε>.
discharge expr with { confidence, taint, budget } { … } unwraps the value.
Capabilities required: cap::infer, cap::network, …
Sanitization: sanitize(val, policy) reduces taint.
