# Cost of type casting

This little experiment aims to explore whether there is a performance cost when type coercion is invoked explicitly by a cast, once a variable already has a defined type.

Initial results show that there is no significant performance penalty when casting unsigned integer types to other unsigned integer types.

```console
$ cargo bench
<...>
running 6 tests
test tests::bench_few_cast_u8_to_usize  ... bench:           0 ns/iter (+/- 0)
test tests::bench_few_initialise_usize  ... bench:           0 ns/iter (+/- 0)
test tests::bench_many_cast_u8_to_usize ... bench:   5,660,225 ns/iter (+/- 37,492)
test tests::bench_many_initialise_usize ... bench:   5,648,124 ns/iter (+/- 56,344)
test tests::bench_some_cast_u8_to_usize ... bench:       8,521 ns/iter (+/- 100)
test tests::bench_some_initialise_usize ... bench:       8,529 ns/iter (+/- 95)

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured; 0 filtered out; finished in 4.58s
```