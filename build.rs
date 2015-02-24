extern crate gcc;
//extern crate bindgen;

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

      for src in &input {
          conf.file(src);
      }

      conf.compile("libchipmunk.a");
}

/*
fn run_bindgen() {
    use bindgen::{BindgenOptions, Bindings, Logger};

    struct StdLogger;

    impl Logger for StdLogger {
        fn error(&self, msg: &str) {
            println!("ERROR: {}", msg);
        }
        fn warn(&self, msg: &str) {
            println!("WARN:  {}", msg);
        }
    }

    let bindings = Bindings::generate(BindgenOptions {
        match_pat: vec!["cp", "chipmunk"],

    }, Some(StdLogger), None);
}*/

fn main() {
//    run_bindgen();
    compile_chipmunk();
}
