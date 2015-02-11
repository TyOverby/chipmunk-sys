extern crate "chipmunk-sys" as chipsys;

use chipsys::*;

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

  let ballbody = cpSpaceAddBody(space, cpBodyNew(mass, moment));
  cpBodySetPosition(ballbody, cpv(0.0, 15.0));

  let ballShape = cpSpaceAddShape(space, cpCircleShapeNew(ballbody, radius, zero));

}}
