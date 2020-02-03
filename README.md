# raumlehre

![raumlehre logo](logo.svg?sanitize=true)

[![Build Status](https://travis-ci.org/j-browne/raumlehre.svg?branch=master)](https://travis-ci.org/j-browne/raumlehre)

A package for calculating geometric properties of physics detectors.

## Getting Started

raumlehre was written in rust, and the rust toolchain is required to build the
project. To install rust, see [rustup].

To build and install the raumlehre tools, run
```sh
git clone git@github.com:j-browne/raumlehre.git
cargo install --path raumlehre
```

### angles

```sh
angles <input_files>
```

### visualize

```sh
visualize <input_files> > <output_file>
gnuplot -e "call 'utilities/visualize.gpi' '<output_file>'" -
```

## Configuration Format

### Examples

```json
{
  "detectors": [
    {
      "template": "box"
    },
    {
      "coords": "PolarZ",
      "u_limits": [0, 1.5],
      "v_limits": [0, 360],
      "transformations": [
        { "Translation": [0, 0, 1] }
      ]
    },
    {
      "coords": "PolarZ",
      "u_limits": [0, 1.5],
      "v_limits": [0, 360],
      "transformations": [
        { "Translation": [0, 0, 1] }
      ],
      "shadows": [
        {
          "template": "box"
        }
      ]
    }
  ],
  "templates": {
    "box": {
      "surfaces": [
        {
          "coords": "CartesianZ",
          "u_limits": [0, 1],
          "v_limits": [-1, 1],
          "transformations": [
            { "Rotation": [0, -90, 0] },
            { "Translation": [-1, 0, 0] }
          ]
        },
        {
          "coords": "CartesianZ",
          "u_limits": [0, 1],
          "v_limits": [-1, 1],
          "transformations": [
            { "Rotation": [0, -90, 0] },
            { "Translation": [-1, 0, 0] },
            { "Rotation": [0, 0, 90] }
          ]
        },
        {
          "coords": "CartesianZ",
          "u_limits": [0, 1],
          "v_limits": [-1, 1],
          "transformations": [
            { "Rotation": [0, -90, 0] },
            { "Translation": [-1, 0, 0] },
            { "Rotation": [0, 0, 180] }
          ]
        },
        {
          "coords": "CartesianZ",
          "u_limits": [0, 1],
          "v_limits": [-1, 1],
          "transformations": [
            { "Rotation": [0, -90, 0] },
            { "Translation": [-1, 0, 0] },
            { "Rotation": [0, 0, 270] }
          ]
        }
      ]
    }
  }
}
```

[rustup]: https://rustup.rs
