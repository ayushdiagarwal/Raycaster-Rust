
# Raycaster

This is an implementation of a simple raycaster which translates a 2D array map into a pseudo 3D scene inspired by early Wolfenstein Games.


## Demo

<p align="center">
  <img src="Demo/demo.gif" alt="Demo GIF"/>
</p>

## Explanation
In this raycaster, simple trigonometry is used to efficiently create a pseudo 3D scene.

### To check the horizontal and vertical intersection of the ray with the blocks

#### Intersection


#### To translate the 2D scene into a pseudo 3D projection


## Installation

Install rust and cargo
Install SDL2 for rust


```bash
  cargo build
  cargo run
```


## To-Do

- [x] Add collision detection
- [x] Add 3d scene
- [x] Add circle instead of rect for player
- [x] Remove fishbowl effect
- [x] Player doesn't move at the very start
- [x] Improve key input so that it doesn't lag
- [ ] Refactor Code
- [ ] Add mouse input
- [ ] Why does removing the background colour messes things up?
- [ ] Add maths explanation to readme
