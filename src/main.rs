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

//use agb::{display, syscall};
use agb::{include_aseprite,
    display::object::{Graphics, Tag}
};
use agb::interrupt::{Interrupt, add_interrupt_handler};
use agb::input::{Button, ButtonController};
use bare_metal::CriticalSection;

// Import the sprites in to this constant. This holds the sprite 
// and palette data in a way that is manageable by agb.
const GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");

// We define some easy ways of referencing the sprites
/*const PADDLE_END: &Tag = GRAPHICS.tags().get("Paddle End");
const PADDLE_MID: &Tag = GRAPHICS.tags().get("Paddle Mid");*/
const BALL: &Tag = GRAPHICS.tags().get("Ball");

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {    
    // Get the OAM manager
    let object = gba.display.object.get();

    // vblank interrupt handler
    let _a = add_interrupt_handler(Interrupt::VBlank, |_: CriticalSection| {
        agb::println!("Woah there! There's been a vblank!");
    });

    let mut input = ButtonController::new();

    // todo: encapsulate ball in a struct
    // Create an object with the ball sprite
    let mut ball = object.object_sprite(BALL.sprite(0));

    let mut ball_x = 50;
    let mut ball_y = 50;
    let x_velocity = 1;
    let y_velocity = 1;
    
    loop {
        // handle input to move ball
        input.update();
        if input.is_pressed(Button::UP) && ball_y > 0 {
            ball_y -= y_velocity;
        }
        if input.is_pressed(Button::DOWN) && ball_y < agb::display::HEIGHT - 16 {
            ball_y += y_velocity;
        }
        if input.is_pressed(Button::LEFT) && ball_x > 0 {
            ball_x -= x_velocity;
        }
        if input.is_pressed(Button::RIGHT) && ball_x < agb::display::WIDTH - 16 {
            ball_x += x_velocity;
        }
    
        // Set the position of the ball to match our new calculated position
        ball.set_x(ball_x as u16).set_y(ball_y as u16);
    
        // Wait for vblank, then commit the objects to the screen
        // todo: don't busy wait for vblank, use interrupt
        agb::display::busy_wait_for_vblank();
        object.commit();
    }
}
