# Lune

**Lune** is a small crate used to compute the moon phases.

Most of the computation are based on:
`Jean Meeus - Astronomical Algorithm 2nd Edition 1998` chapters 7, 47 and 49.

## Installation

You can install `lune` by cloning the repository and installing the crate with [cargo](https://rust-lang.org/tools/install/).

```
git clone https://codeberg.org/PiaCOS/lune.git
cd lune
cargo install --path .
```

You may need to add `cargo` to your $PATH.

## Usage:

It comes with a basic cli which can be used as such:

`summary` to print a small summary of the moon cycle.

```
> lune

Phase: Waning Crescent
Illumination: 3.8%
New Moon is in 2 days
Last Quarter was 6 days ago
```

`current` to print the current phase and illumination.

```
> lune current

Waning Crescent (3.8%)
```

`phases` to print the previous and next phases.

```
> lune phases

Last Quarter +6, New Moon -2
```

`next` to print the abbreviated next phases.

```
> lune next

NM -2
```

`prev` to print the abbreviated previous phases.

```
> lune prev

LQ +6
```

~ Let's all love Lune ~
