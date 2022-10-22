// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't really have an operating
// system, so most of the content of the standard library doesn't apply.
//
// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{display, syscall};
use agb::{include_aseprite,
    display::object::{Graphics, Tag}
};

// Import the sprites in to this constant. This holds the sprite 
// and palette data in a way that is manageable by agb.
const GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");

// We define some easy ways of referencing the sprites
const PADDLE_END: &Tag = GRAPHICS.tags().get("Paddle End");
const PADDLE_MID: &Tag = GRAPHICS.tags().get("Paddle Mid");
const BALL: &Tag = GRAPHICS.tags().get("Ball");

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    /*let mut bitmap = gba.display.video.bitmap3();

    for x in 0..display::WIDTH {
        let y = syscall::sqrt(x << 6);
        let y = (display::HEIGHT - y).clamp(0, display::HEIGHT - 1);
        bitmap.draw_point(x, y, 0x001F);
    }*/
    
    // Get the OAM manager
    let object = gba.display.object.get();

    // Create an object with the ball sprite
    let mut ball = object.object_sprite(BALL.sprite(0));

    // Place this at some point on the screen, (50, 50) for example
    ball.set_x(50).set_y(50).show();

    // Now commit the object controller so this change is reflected on the screen, 
    // this should normally be done in vblank but it'll work just fine here for now
    object.commit();

    loop {
        //syscall::halt();
    }
}
