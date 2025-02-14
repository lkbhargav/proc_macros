## Following proc-macros are exposed with this package

1. **FieldCounter**: Implements a `field_count` method on a enum / struct types so that we get a count of variants / fields available in either of those.

2. **Random**: Implements a `random` methods on a enum so we can call it to get a random enum variant. Add the following imports for the Random to work

`
  use rand::seq::IndexedRandom;
`

3. **ValueAssigner**: Implements a `get_value` and `get_type` methods on a enum so we can get a usize value of the variant.