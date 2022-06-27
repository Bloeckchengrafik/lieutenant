# Lieutenant

Command dispatcher for Rust based on Mojang's Brigadier

# Table of Contents

- [Installation](#installation)
- [A simple example](#a-simple-example)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

# Installation

If you want to use Lieutenant in your own project, just add this line to the dependency section in your `Cargo.toml`

```toml
lieutenant = { git = "https://github.com/feather-rs/lieutenant" }
```

# A simple example

```rust
use crate::command::builder::{literal, space, CommandBuilder};
use crate::command::Command;

fn main() {
    let command = literal("/").space().arg::<u32>();
    let x = command.on_call(|x| {
        move |game_state, _foo| {
            println!("hi {} the gamestate was {}", x, game_state);
            42
        }
    });

    let r = x.call((0, "test"), "/ 100 ").unwrap();

    assert!(r == 42);
}
```

This outputs `hi 100 the gamestate was 0`.

# Usage

To use the basic command system, import

```rust
use crate::command::builder::{literal, space, CommandBuilder};
use crate::command::Command;
```

at the top of your file. Then, you can start building commands.

Every Command must start with either a String Literal or a Space

- For a literal, use `let command = literal("<Literal>")`
- For a space, use `let command = space()`

Other literals, spaces and arguments can now be chained after that initial statement.

Arguments can be added with the `arg::<Type>()` function. Currently, `u32` and `String` are supported as argument types.
Additionally, there is a `StringWildcard` type you can use to catch a String with spaces in it. After this, you can't add any other argument types.

You can also add optional arguments (`opt_arg::<Type>()`) or spaces (`opt_space()`).
Note that when using `opt_arg`, the  data type in the closure is `Option<(Type,)>`.

The `command.on_call`-Method uses a closure as an argument that will be executed once the command is called with the
arguments.
With `move |game_state, ...|`, one can access the game state given by the call method.

A command can be called by the return value of the `on_call` method. Here, a Game State can be passed to the closure.
The second argument is the command to parse.
The Return value of the closure or an error is returned here.

# Contributing

Check out our [issue tracker](https://github.com/feather-rs/lieutenant/issues) to find out what needs to be worked on.
Feel free to join our [Discord](https://discordapp.com/invite/4eYmK69) and ask questions whenever you need. Thanks for
your interest in contributing!

# License

This project is licenced under the MIT and Apache licence.

See [MIT](LICENSE-MIT.md) and [Apache](LICENSE-APACHE.md) for more information.
