# Dicey v0.0.1

Dicey is an extremely simple Rust dice roller. In contrast with the other Rust
dice roller crates I've seen, Dicey does not parse any strings or require
unwrapping values.

The basic idea is if you're making a thing in Rust you can use your tabletop
experience to get suitable random odds by using this in the source code.

## Usage

``` rust
use dicey::roll;

roll(1).d(20); // generic d() = 1d20
roll(6).d(6); // generic d() = 6d6
roll(1).advantage(20); // 1d20 with advantage, = max(roll(1).d(20), roll(1).d(20))
roll(3).advantage(20); // 3d20 with advantage, = max(roll(3).d(20), roll(3).d(20))
roll(1).disadvantage(20); // 1d20 with disadvantage = max(roll(1).d(20), roll(1).d(20))
// useful for comparisons
roll(1).crit_fail(20); // value corresponding to crit fail "all ones", = 1
roll(2).crit_fail(20); // value corresponding to crit fail "all ones", = 2
roll(1).crit_success(20); // value corresponding to crit success = 20
roll(2).crit_success(20); // value corresponding to crit success = 40
// eg:
if roll(1).d(20) == roll(1).crit_success(20) {
  massive_damage();
}

// convenience functions
roll(1).d4();
roll(1).d6();
roll(1).d8();
roll(1).d10();
roll(1).d12();
roll(1).d20();

```

## Quirks

### Zeroes

Not needing to unwrap values is great, and mostly works as expected since Dicey
only deals in unsigned numbers. The only odd bit comes with zeroes, and Dicey
deals with this by just always returning 0. In my opinion this makes sense.

```
roll(0).d(6) // -> 0
roll(1).d(0) // -> 0
```

### Coins

The previous example is the *only* time Dicey returns zeroes - d10s are 1-10. This
means if you're using a "roll(1).d(2)" as a "coin" for a 50/50 chance, it will return
1 or 2, *not a 1 or 0*.

## The Future

### Seedable RNG

Probably should support seeding the Rng. This isn't difficult but I've got to
do some thinking about the most ergonomic approach.

### Generic unsigned

No reason why it needs to be `usize` - Dicey should be workable with all unsigned
number types.
