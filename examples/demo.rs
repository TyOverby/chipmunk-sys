extern crate chipmunk_sys;

use chipmunk_sys::*;

fn main() { unsafe {
    let gravity = cpv(0.0, -100.0);
    let zero = cpv(0.0, 0.0);

    let space = cpSpaceNew();
    cpSpaceSetGravity(space, gravity);
    let ground = cpSegmentShapeNew((*space).staticBody,
                                   cpv(-20.0, 5.0),
                                   cpv(20.0, -5.0),
                                   0.0);
  cpShapeSetFriction(ground, 1.0);
  cpSpaceAddShape(space, ground);

  let radius = 5.0;
  let mass = 1.0;

  let moment = cpMomentForCircle(mass, 0.0, radius, zero);

  let ball_body = cpSpaceAddBody(space, cpBodyNew(mass, moment));
  cpBodySetPosition(ball_body, cpv(0.0, 15.0));

  let ball_shape = cpSpaceAddShape(space, cpCircleShapeNew(ball_body, radius, zero));
  cpShapeSetFriction(ball_shape, 0.7);

  let time_step = 1.0 / 60.0;
  for i in 0 .. 60 {
      let time = time_step * i as f32;
      let pos = cpBodyGetPosition(ball_body);
      let vel = cpBodyGetVelocity(ball_body);
      println!("Time: {:?}, Pos: {:?}, Vel: {:?}", time, pos, vel);
      cpSpaceStep(space, time_step as f64);
  }

}}
