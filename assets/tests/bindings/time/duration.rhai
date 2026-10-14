// constructors
let d = Duration.from_secs.call(1.5);
assert(d.as_secs.call() == 1.5, "from_secs");
assert(Duration.from_millis.call(1500) == d, "from_millis");
assert(Duration.from_micros.call(1500000) == d, "from_micros");
assert(Duration.from_nanos.call(1500000000) == d, "from_nanos");
assert(Duration.zero.call().is_zero.call(), "zero");

// conversions
assert(d.as_secs_f64.call() == 1.5, "as_secs_f64");
assert(d.as_millis.call() == 1500, "as_millis");
assert(!d.is_zero.call(), "is_zero is false for a positive duration");

// arithmetic
assert(d + Duration.from_millis.call(500) == Duration.from_secs.call(2.0), "add");
assert(d - Duration.from_secs.call(2.0) == Duration.zero.call(), "sub saturates at zero");
assert(d * 2.0 == Duration.from_secs.call(3.0), "mul");
assert(d / 3.0 == Duration.from_millis.call(500), "div");

// comparison
assert(d.eq.call(Duration.from_millis.call(1500)), "eq");
assert(d.lt.call(Duration.from_secs.call(2.0)), "lt");
assert(!d.lt.call(Duration.zero.call()), "lt is false for the longer duration");
