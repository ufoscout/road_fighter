# Road Fighter
Road Fighter remake in Rust using Bevy ECS.

## Status

### Done
- [x] State machine: Disclaimer → Introduction → Menu → Playing
- [x] Map loading from `.mg2` files (all 6 levels)
- [x] GPU-chunked scrolling map renderer (background thread compositing)
- [x] Player car: movement, acceleration/braking, keyboard input
- [x] Wall collision detection (Avian2d physics)
- [x] Explosion animation (12-frame spritesheet)
- [x] Semaphore / traffic light at level start (blocks car until green)

### Missing
- [ ] Enemy cars (6 types: Normal, Racer, Fast, Slidder, Truck)
- [ ] Obstacles: water, oil slicks, rocks
- [ ] Fuel system: consumption over time + pickup objects
- [ ] HUD: fuel gauge, score, level indicator
- [ ] Score and time bonus tracking
- [ ] Level transition (interlevel) screen
- [ ] Game over state and restart
- [ ] Menu interaction (currently placeholder)
- [ ] Sound and music
- [ ] Tyre / skid marks
- [ ] 2-player support

## Credits
This port is heavily inspired by the https://github.com/ptitSeb/roadfighter project that was used as a source for all assets and for the game logic.
