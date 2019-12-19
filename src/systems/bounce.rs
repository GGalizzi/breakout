use crate::game_objects::{Ball, Paddle};
use crate::config::ArenaConfig;

use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage, World},
};

/// This system is responsible for moving all the paddles according to the user
/// provided input.
#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, ArenaConfig>,
    );

    fn run(&mut self, (mut balls, transforms, paddles, arena_config): Self::SystemData) {
        // Iterate over all paddles and move them according to the input the user
        // provided.
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            let (arena_width, arena_height) = (arena_config.width, arena_config.height);

            // Bounce at the top of the arena.
            if ball_y >= arena_height - ball.radius && ball.velocity[1] > 0.0 {
                ball.velocity[1] = -ball.velocity[1];
            }

            // Boiunce off the sides
            if (ball_x >= arena_width - ball.radius && ball.velocity[0] > 0.0)
                || (ball_x <= ball.radius && ball.velocity[0] < 0.0)
            {
                ball.velocity[0] = -ball.velocity[0];
            }

            // Bounce at the paddle.
            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x;
                let paddle_y = paddle_transform.translation().y;

                // To determine whether the ball has collided with a paddle, we create a larger
                // rectangle around the current one, by subtracting the ball radius from the
                // lowest coordinates, and adding the ball radius to the highest ones. The ball
                // is then within the paddle if its center is within the larger wrapper
                // rectangle.
                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - (paddle.width * 0.5) - ball.radius,
                    paddle_y - (paddle.height * 0.5) - ball.radius,
                    paddle_x + (paddle.width * 0.5) + ball.radius,
                    paddle_y + (paddle.height * 0.5) + ball.radius,
                ) {
                    if ball.velocity[1] < 0.0
                    {
                        ball.velocity[1] = -ball.velocity[1];
                    }
                }
            }
        }
    }
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}