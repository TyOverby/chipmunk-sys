extern crate gcc;

use std::env;

fn compile_chipmunk() {
    let input = [
        "chipmunk/src/chipmunk.c",
        "chipmunk/src/cpArbiter.c",
        "chipmunk/src/cpArray.c",
        "chipmunk/src/cpBBTree.c",
        "chipmunk/src/cpBody.c",
        "chipmunk/src/cpCollision.c",
        "chipmunk/src/cpConstraint.c",
        "chipmunk/src/cpDampedRotarySpring.c",
        "chipmunk/src/cpDampedSpring.c",
        "chipmunk/src/cpGearJoint.c",
        "chipmunk/src/cpGrooveJoint.c",
        "chipmunk/src/cpHashSet.c",
        "chipmunk/src/cpHastySpace.c",
        "chipmunk/src/cpMarch.c",
        "chipmunk/src/cpPinJoint.c",
        "chipmunk/src/cpPivotJoint.c",
        "chipmunk/src/cpPolyShape.c",
        "chipmunk/src/cpPolyline.c",
        "chipmunk/src/cpRatchetJoint.c",
        "chipmunk/src/cpRotaryLimitJoint.c",
        "chipmunk/src/cpShape.c",
        "chipmunk/src/cpSimpleMotor.c",
        "chipmunk/src/cpSlideJoint.c",
        "chipmunk/src/cpSpace.c",
        "chipmunk/src/cpSpaceComponent.c",
        "chipmunk/src/cpSpaceDebug.c",
        "chipmunk/src/cpSpaceHash.c",
        "chipmunk/src/cpSpaceQuery.c",
        "chipmunk/src/cpSpaceStep.c",
        "chipmunk/src/cpSpatialIndex.c",
        "chipmunk/src/cpSweep1D.c",
    ];

      let mut conf = gcc::Config::new();

      conf.include("chipmunk/include/");
      conf.flag("-std=c99");

      if let Ok(profile) = env::var("PROFILE") {
          if "debug" == profile {
            conf.flag("-g");
          }
          if "release" == profile {
              conf.flag("-DNDEBUG");
          }
      }

      for src in &input {
          conf.file(src);
      }

      conf.compile("libchipmunk.a");
}

fn main() {
    compile_chipmunk();
}
