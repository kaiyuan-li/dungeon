# Dungeon Crawler

## Learnings

### clippy for linter and suggestions

```
cargo clippy
```

### Derivatives

* `Clone` adds the `.clone()` method to a struct
* `Copy` changes the assign behavior from moving to copying.

### Prelude

```rs
// main.rs

// add the module to your project
mod map;
// declaring a new module in the code, which is visible through the crate
mod prelude {
    // anything using this module will also have bracket_lib::prelude
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub use crate::map::*;
}

// make the prelude available in main.rs
use prelude::*;


// map.rs
// to get hte constants in main.rs
use crate::prelude::*;
```

### Procedural macro

```rs
#[system]
pub fn player_input() {}
```

`#[system]` annotates the player_input function with a procedural macro named `system`. This macro transforms your function name into `player_input_system`, and wraps it with all of the extra code Legion requires to construct a system.
