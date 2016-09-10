#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate libc;

pub type cpFloat = ::libc::c_double;
pub type cpHashValue = usize;
pub type cpCollisionID = usize;
pub type cpBool = ::libc::c_uchar;
pub type cpDataPointer = *mut ::libc::c_void;
pub type cpCollisionType = usize;
pub type cpGroup = usize;
pub type cpBitmask = ::libc::c_uint;
pub type cpTimestamp = ::libc::c_uint;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_cpVect {
    pub x: cpFloat,
    pub y: cpFloat,
}
impl ::std::default::Default for Struct_cpVect {
    fn default() -> Struct_cpVect { unsafe { ::std::mem::zeroed() } }
}
pub type cpVect = Struct_cpVect;
impl Struct_cpVect {
    #[deprecated(note="Use v.into() or From::from(v) instead of v.to_tuple().")]
    pub fn to_tuple(&self) -> (cpFloat, cpFloat) {
        (self.x, self.y)
    }
}

pub fn cpv(x: cpFloat, y: cpFloat) -> cpVect {
    cpVect{x: x, y:y}
}

impl From<cpVect> for (cpFloat, cpFloat) {
    fn from(vect: cpVect) -> (cpFloat, cpFloat) {
        (vect.x, vect.y)
    }
}

impl<'a> From<&'a cpVect> for (cpFloat, cpFloat) {
    fn from(vect: &'a cpVect) -> (cpFloat, cpFloat) {
        (vect.x, vect.y)
    }
}

impl From<(cpFloat, cpFloat)> for cpVect {
    fn from(tuple: (cpFloat, cpFloat)) -> cpVect {
        cpVect { x: tuple.0, y: tuple.1 }
    }
}

#[cfg(test)]
#[test]
fn test_cpvect_from_into() {
    let v = cpVect::from((1.2, 3.4));
    assert_eq!(1.2, v.x);
    assert_eq!(3.4, v.y);
    assert_eq!((1.2, 3.4), From::from(v));
    assert_eq!((1.2, 3.4), From::from(&v));

    let v2: cpVect = (5.6, 7.8).into();
    assert_eq!(5.6, v2.x);
    assert_eq!(7.8, v2.y);
    assert_eq!((5.6, 7.8), v2.into());
    assert_eq!((5.6, 7.8), (&v2).into());
}


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_cpTransform {
    pub a: cpFloat,
    pub b: cpFloat,
    pub c: cpFloat,
    pub d: cpFloat,
    pub tx: cpFloat,
    pub ty: cpFloat,
}
impl ::std::default::Default for Struct_cpTransform {
    fn default() -> Struct_cpTransform { unsafe { ::std::mem::zeroed() } }
}
pub type cpTransform = Struct_cpTransform;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_cpMat2x2 {
    pub a: cpFloat,
    pub b: cpFloat,
    pub c: cpFloat,
    pub d: cpFloat,
}
impl ::std::default::Default for Struct_cpMat2x2 {
    fn default() -> Struct_cpMat2x2 { unsafe { ::std::mem::zeroed() } }
}
pub type cpMat2x2 = Struct_cpMat2x2;
pub type cpArray = Struct_cpArray;
pub enum Struct_cpHashSet { }
pub type cpHashSet = Struct_cpHashSet;
pub type cpBody = Struct_cpBody;
pub type cpShape = Struct_cpShape;
pub type cpCircleShape = Struct_cpCircleShape;
pub type cpSegmentShape = Struct_cpSegmentShape;
pub type cpPolyShape = Struct_cpPolyShape;
pub type cpConstraint = Struct_cpConstraint;
pub type cpPinJoint = Struct_cpPinJoint;
pub type cpSlideJoint = Struct_cpSlideJoint;
pub type cpPivotJoint = Struct_cpPivotJoint;
pub type cpGrooveJoint = Struct_cpGrooveJoint;
pub type cpDampedSpring = Struct_cpDampedSpring;
pub type cpDampedRotarySpring = Struct_cpDampedRotarySpring;
pub type cpRotaryLimitJoint = Struct_cpRotaryLimitJoint;
pub type cpRatchetJoint = Struct_cpRatchetJoint;
pub type cpGearJoint = Struct_cpGearJoint;
pub enum Struct_cpSimpleMotorJoint { }
pub type cpSimpleMotorJoint = Struct_cpSimpleMotorJoint;
pub type cpCollisionHandler = Struct_cpCollisionHandler;
pub type cpContactPointSet = Struct_cpContactPointSet;
pub type cpArbiter = Struct_cpArbiter;
pub type cpSpace = Struct_cpSpace;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_cpBB {
    pub l: cpFloat,
    pub b: cpFloat,
    pub r: cpFloat,
    pub t: cpFloat,
}
impl ::std::default::Default for Struct_cpBB {
    fn default() -> Struct_cpBB { unsafe { ::std::mem::zeroed() } }
}
pub type cpBB = Struct_cpBB;
pub type cpSpatialIndexBBFunc =
    ::std::option::Option<extern "C" fn(obj: *mut ::libc::c_void) -> cpBB>;
pub type cpSpatialIndexIteratorFunc =
    ::std::option::Option<extern "C" fn
                              (obj: *mut ::libc::c_void,
                               data: *mut ::libc::c_void) -> ()>;
pub type cpSpatialIndexQueryFunc =
    ::std::option::Option<extern "C" fn
                              (obj1: *mut ::libc::c_void,
                               obj2: *mut ::libc::c_void, id: cpCollisionID,
                               data: *mut ::libc::c_void) -> cpCollisionID>;
pub type cpSpatialIndexSegmentQueryFunc =
    ::std::option::Option<extern "C" fn
                              (obj1: *mut ::libc::c_void,
                               obj2: *mut ::libc::c_void,
                               data: *mut ::libc::c_void) -> cpFloat>;
pub type cpSpatialIndexClass = Struct_cpSpatialIndexClass;
pub type cpSpatialIndex = Struct_cpSpatialIndex;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpSpatialIndex {
    pub klass: *mut cpSpatialIndexClass,
    pub bbfunc: cpSpatialIndexBBFunc,
    pub staticIndex: *mut cpSpatialIndex,
    pub dynamicIndex: *mut cpSpatialIndex,
}
impl ::std::default::Default for Struct_cpSpatialIndex {
    fn default() -> Struct_cpSpatialIndex { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct_cpSpaceHash { }
pub type cpSpaceHash = Struct_cpSpaceHash;
pub enum Struct_cpBBTree { }
pub type cpBBTree = Struct_cpBBTree;
pub type cpBBTreeVelocityFunc =
    ::std::option::Option<extern "C" fn(obj: *mut ::libc::c_void) -> cpVect>;
pub enum Struct_cpSweep1D { }
pub type cpSweep1D = Struct_cpSweep1D;
pub type cpSpatialIndexDestroyImpl =
    ::std::option::Option<extern "C" fn(index: *mut cpSpatialIndex) -> ()>;
pub type cpSpatialIndexCountImpl =
    ::std::option::Option<extern "C" fn(index: *mut cpSpatialIndex)
                              -> ::libc::c_int>;
pub type cpSpatialIndexEachImpl =
    ::std::option::Option<extern "C" fn
                              (index: *mut cpSpatialIndex,
                               func: cpSpatialIndexIteratorFunc,
                               data: *mut ::libc::c_void) -> ()>;
pub type cpSpatialIndexContainsImpl =
    ::std::option::Option<extern "C" fn
                              (index: *mut cpSpatialIndex,
                               obj: *mut ::libc::c_void, hashid: cpHashValue)
                              -> cpBool>;
pub type cpSpatialIndexInsertImpl =
    ::std::option::Option<extern "C" fn
                              (index: *mut cpSpatialIndex,
                               obj: *mut ::libc::c_void, hashid: cpHashValue)
                              -> ()>;
pub type cpSpatialIndexRemoveImpl =
    ::std::option::Option<extern "C" fn
                              (index: *mut cpSpatialIndex,
                               obj: *mut ::libc::c_void, hashid: cpHashValue)
                              -> ()>;
pub type cpSpatialIndexReindexImpl =
    ::std::option::Option<extern "C" fn(index: *mut cpSpatialIndex) -> ()>;
pub type cpSpatialIndexReindexObjectImpl =
    ::std::option::Option<extern "C" fn
                              (index: *mut cpSpatialIndex,
                               obj: *mut ::libc::c_void, hashid: cpHashValue)
                              -> ()>;
pub type cpSpatialIndexReindexQueryImpl =
    ::std::option::Option<extern "C" fn
                              (index: *mut cpSpatialIndex,
                               func: cpSpatialIndexQueryFunc,
                               data: *mut ::libc::c_void) -> ()>;
pub type cpSpatialIndexQueryImpl =
    ::std::option::Option<extern "C" fn
                              (index: *mut cpSpatialIndex,
                               obj: *mut ::libc::c_void, bb: cpBB,
                               func: cpSpatialIndexQueryFunc,
                               data: *mut ::libc::c_void) -> ()>;
pub type cpSpatialIndexSegmentQueryImpl =
    ::std::option::Option<extern "C" fn
                              (index: *mut cpSpatialIndex,
                               obj: *mut ::libc::c_void, a: cpVect, b: cpVect,
                               t_exit: cpFloat,
                               func: cpSpatialIndexSegmentQueryFunc,
                               data: *mut ::libc::c_void) -> ()>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpSpatialIndexClass {
    pub destroy: cpSpatialIndexDestroyImpl,
    pub count: cpSpatialIndexCountImpl,
    pub each: cpSpatialIndexEachImpl,
    pub contains: cpSpatialIndexContainsImpl,
    pub insert: cpSpatialIndexInsertImpl,
    pub remove: cpSpatialIndexRemoveImpl,
    pub reindex: cpSpatialIndexReindexImpl,
    pub reindexObject: cpSpatialIndexReindexObjectImpl,
    pub reindexQuery: cpSpatialIndexReindexQueryImpl,
    pub query: cpSpatialIndexQueryImpl,
    pub segmentQuery: cpSpatialIndexSegmentQueryImpl,
}
impl ::std::default::Default for Struct_cpSpatialIndexClass {
    fn default() -> Struct_cpSpatialIndexClass {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_cpContactPointSet {
    pub count: ::libc::c_int,
    pub normal: cpVect,
    pub points: [Struct_Unnamed1; 2usize],
}
impl ::std::default::Default for Struct_cpContactPointSet {
    fn default() -> Struct_cpContactPointSet {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_Unnamed1 {
    pub pointA: cpVect,
    pub pointB: cpVect,
    pub distance: cpFloat,
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Struct_Unnamed1 { unsafe { ::std::mem::zeroed() } }
}
pub type Enum_cpBodyType = ::libc::c_uint;
pub const CP_BODY_TYPE_DYNAMIC: ::libc::c_uint = 0;
pub const CP_BODY_TYPE_KINEMATIC: ::libc::c_uint = 1;
pub const CP_BODY_TYPE_STATIC: ::libc::c_uint = 2;
pub type cpBodyType = Enum_cpBodyType;
pub type cpBodyVelocityFunc =
    ::std::option::Option<extern "C" fn
                              (body: *mut cpBody, gravity: cpVect,
                               damping: cpFloat, dt: cpFloat) -> ()>;
pub type cpBodyPositionFunc =
    ::std::option::Option<extern "C" fn(body: *mut cpBody, dt: cpFloat)
                              -> ()>;
pub type cpBodyShapeIteratorFunc =
    ::std::option::Option<extern "C" fn
                              (body: *mut cpBody, shape: *mut cpShape,
                               data: *mut ::libc::c_void) -> ()>;
pub type cpBodyConstraintIteratorFunc =
    ::std::option::Option<extern "C" fn
                              (body: *mut cpBody,
                               constraint: *mut cpConstraint,
                               data: *mut ::libc::c_void) -> ()>;
pub type cpBodyArbiterIteratorFunc =
    ::std::option::Option<extern "C" fn
                              (body: *mut cpBody, arbiter: *mut cpArbiter,
                               data: *mut ::libc::c_void) -> ()>;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_cpPointQueryInfo {
    pub shape: *const cpShape,
    pub point: cpVect,
    pub distance: cpFloat,
    pub gradient: cpVect,
}
impl ::std::default::Default for Struct_cpPointQueryInfo {
    fn default() -> Struct_cpPointQueryInfo {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type cpPointQueryInfo = Struct_cpPointQueryInfo;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_cpSegmentQueryInfo {
    pub shape: *const cpShape,
    pub point: cpVect,
    pub normal: cpVect,
    pub alpha: cpFloat,
}
impl ::std::default::Default for Struct_cpSegmentQueryInfo {
    fn default() -> Struct_cpSegmentQueryInfo {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type cpSegmentQueryInfo = Struct_cpSegmentQueryInfo;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_cpShapeFilter {
    pub group: cpGroup,
    pub categories: cpBitmask,
    pub mask: cpBitmask,
}
impl ::std::default::Default for Struct_cpShapeFilter {
    fn default() -> Struct_cpShapeFilter { unsafe { ::std::mem::zeroed() } }
}
pub type cpShapeFilter = Struct_cpShapeFilter;
pub type cpConstraintPreSolveFunc =
    ::std::option::Option<extern "C" fn
                              (constraint: *mut cpConstraint,
                               space: *mut cpSpace) -> ()>;
pub type cpConstraintPostSolveFunc =
    ::std::option::Option<extern "C" fn
                              (constraint: *mut cpConstraint,
                               space: *mut cpSpace) -> ()>;
pub type cpDampedSpringForceFunc =
    ::std::option::Option<extern "C" fn
                              (spring: *mut cpConstraint, dist: cpFloat)
                              -> cpFloat>;
pub type cpDampedRotarySpringTorqueFunc =
    ::std::option::Option<extern "C" fn
                              (spring: *mut Struct_cpConstraint,
                               relativeAngle: cpFloat) -> cpFloat>;
pub type cpSimpleMotor = Struct_cpSimpleMotor;
pub type cpCollisionBeginFunc =
    ::std::option::Option<extern "C" fn
                              (arb: *mut cpArbiter, space: *mut cpSpace,
                               userData: cpDataPointer) -> cpBool>;
pub type cpCollisionPreSolveFunc =
    ::std::option::Option<extern "C" fn
                              (arb: *mut cpArbiter, space: *mut cpSpace,
                               userData: cpDataPointer) -> cpBool>;
pub type cpCollisionPostSolveFunc =
    ::std::option::Option<extern "C" fn
                              (arb: *mut cpArbiter, space: *mut cpSpace,
                               userData: cpDataPointer) -> ()>;
pub type cpCollisionSeparateFunc =
    ::std::option::Option<extern "C" fn
                              (arb: *mut cpArbiter, space: *mut cpSpace,
                               userData: cpDataPointer) -> ()>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpCollisionHandler {
    pub typeA: cpCollisionType,
    pub typeB: cpCollisionType,
    pub beginFunc: cpCollisionBeginFunc,
    pub preSolveFunc: cpCollisionPreSolveFunc,
    pub postSolveFunc: cpCollisionPostSolveFunc,
    pub separateFunc: cpCollisionSeparateFunc,
    pub userData: cpDataPointer,
}
impl ::std::default::Default for Struct_cpCollisionHandler {
    fn default() -> Struct_cpCollisionHandler {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type cpPostStepFunc =
    ::std::option::Option<extern "C" fn
                              (space: *mut cpSpace, key: *mut ::libc::c_void,
                               data: *mut ::libc::c_void) -> ()>;
pub type cpSpacePointQueryFunc =
    ::std::option::Option<extern "C" fn
                              (shape: *mut cpShape, point: cpVect,
                               distance: cpFloat, gradient: cpVect,
                               data: *mut ::libc::c_void) -> ()>;
pub type cpSpaceSegmentQueryFunc =
    ::std::option::Option<extern "C" fn
                              (shape: *mut cpShape, point: cpVect,
                               normal: cpVect, alpha: cpFloat,
                               data: *mut ::libc::c_void) -> ()>;
pub type cpSpaceBBQueryFunc =
    ::std::option::Option<extern "C" fn
                              (shape: *mut cpShape, data: *mut ::libc::c_void)
                              -> ()>;
pub type cpSpaceShapeQueryFunc =
    ::std::option::Option<extern "C" fn
                              (shape: *mut cpShape,
                               points: *mut cpContactPointSet,
                               data: *mut ::libc::c_void) -> ()>;
pub type cpSpaceBodyIteratorFunc =
    ::std::option::Option<extern "C" fn
                              (body: *mut cpBody, data: *mut ::libc::c_void)
                              -> ()>;
pub type cpSpaceShapeIteratorFunc =
    ::std::option::Option<extern "C" fn
                              (shape: *mut cpShape, data: *mut ::libc::c_void)
                              -> ()>;
pub type cpSpaceConstraintIteratorFunc =
    ::std::option::Option<extern "C" fn
                              (constraint: *mut cpConstraint,
                               data: *mut ::libc::c_void) -> ()>;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_cpSpaceDebugColor {
    pub r: ::libc::c_float,
    pub g: ::libc::c_float,
    pub b: ::libc::c_float,
    pub a: ::libc::c_float,
}
impl ::std::default::Default for Struct_cpSpaceDebugColor {
    fn default() -> Struct_cpSpaceDebugColor {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type cpSpaceDebugColor = Struct_cpSpaceDebugColor;
pub type cpSpaceDebugDrawCircleImpl =
    ::std::option::Option<extern "C" fn
                              (pos: cpVect, angle: cpFloat, radius: cpFloat,
                               outlineColor: cpSpaceDebugColor,
                               fillColor: cpSpaceDebugColor,
                               data: cpDataPointer) -> ()>;
pub type cpSpaceDebugDrawSegmentImpl =
    ::std::option::Option<extern "C" fn
                              (a: cpVect, b: cpVect, color: cpSpaceDebugColor,
                               data: cpDataPointer) -> ()>;
pub type cpSpaceDebugDrawFatSegmentImpl =
    ::std::option::Option<extern "C" fn
                              (a: cpVect, b: cpVect, radius: cpFloat,
                               outlineColor: cpSpaceDebugColor,
                               fillColor: cpSpaceDebugColor,
                               data: cpDataPointer) -> ()>;
pub type cpSpaceDebugDrawPolygonImpl =
    ::std::option::Option<extern "C" fn
                              (count: ::libc::c_int, verts: *const cpVect,
                               radius: cpFloat,
                               outlineColor: cpSpaceDebugColor,
                               fillColor: cpSpaceDebugColor,
                               data: cpDataPointer) -> ()>;
pub type cpSpaceDebugDrawDotImpl =
    ::std::option::Option<extern "C" fn
                              (size: cpFloat, pos: cpVect,
                               color: cpSpaceDebugColor, data: cpDataPointer)
                              -> ()>;
pub type cpSpaceDebugDrawColorForShapeImpl =
    ::std::option::Option<extern "C" fn
                              (shape: *mut cpShape, data: cpDataPointer)
                              -> cpSpaceDebugColor>;
pub type Enum_cpSpaceDebugDrawFlags = ::libc::c_uint;
pub const CP_SPACE_DEBUG_DRAW_SHAPES: ::libc::c_uint = 1;
pub const CP_SPACE_DEBUG_DRAW_CONSTRAINTS: ::libc::c_uint = 2;
pub const CP_SPACE_DEBUG_DRAW_COLLISION_POINTS: ::libc::c_uint = 4;
pub type cpSpaceDebugDrawFlags = Enum_cpSpaceDebugDrawFlags;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpSpaceDebugDrawOptions {
    pub drawCircle: cpSpaceDebugDrawCircleImpl,
    pub drawSegment: cpSpaceDebugDrawSegmentImpl,
    pub drawFatSegment: cpSpaceDebugDrawFatSegmentImpl,
    pub drawPolygon: cpSpaceDebugDrawPolygonImpl,
    pub drawDot: cpSpaceDebugDrawDotImpl,
    pub flags: cpSpaceDebugDrawFlags,
    pub shapeOutlineColor: cpSpaceDebugColor,
    pub colorForShape: cpSpaceDebugDrawColorForShapeImpl,
    pub constraintColor: cpSpaceDebugColor,
    pub collisionPointColor: cpSpaceDebugColor,
    pub data: cpDataPointer,
}
impl ::std::default::Default for Struct_cpSpaceDebugDrawOptions {
    fn default() -> Struct_cpSpaceDebugDrawOptions {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type cpSpaceDebugDrawOptions = Struct_cpSpaceDebugDrawOptions;
pub type cpSpacePointQueryBlock = ::libc::c_void;
pub type cpSpaceSegmentQueryBlock = ::libc::c_void;
pub type cpSpaceBBQueryBlock = ::libc::c_void;
pub type cpSpaceShapeQueryBlock = ::libc::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpArray {
    pub num: ::libc::c_int,
    pub max: ::libc::c_int,
    pub arr: *mut *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_cpArray {
    fn default() -> Struct_cpArray { unsafe { ::std::mem::zeroed() } }
}
pub type cpHashSetEqlFunc =
    ::std::option::Option<extern "C" fn
                              (ptr: *mut ::libc::c_void,
                               elt: *mut ::libc::c_void) -> cpBool>;
pub type cpHashSetTransFunc =
    ::std::option::Option<extern "C" fn
                              (ptr: *mut ::libc::c_void,
                               data: *mut ::libc::c_void)
                              -> *mut ::libc::c_void>;
pub type cpHashSetIteratorFunc =
    ::std::option::Option<extern "C" fn
                              (elt: *mut ::libc::c_void,
                               data: *mut ::libc::c_void) -> ()>;
pub type cpHashSetFilterFunc =
    ::std::option::Option<extern "C" fn
                              (elt: *mut ::libc::c_void,
                               data: *mut ::libc::c_void) -> cpBool>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpBody {
    pub velocity_func: cpBodyVelocityFunc,
    pub position_func: cpBodyPositionFunc,
    pub m: cpFloat,
    pub m_inv: cpFloat,
    pub i: cpFloat,
    pub i_inv: cpFloat,
    pub cog: cpVect,
    pub p: cpVect,
    pub v: cpVect,
    pub f: cpVect,
    pub a: cpFloat,
    pub w: cpFloat,
    pub t: cpFloat,
    pub transform: cpTransform,
    pub userData: cpDataPointer,
    pub v_bias: cpVect,
    pub w_bias: cpFloat,
    pub space: *mut cpSpace,
    pub shapeList: *mut cpShape,
    pub arbiterList: *mut cpArbiter,
    pub constraintList: *mut cpConstraint,
    pub sleeping: Struct_Unnamed2,
}
impl ::std::default::Default for Struct_cpBody {
    fn default() -> Struct_cpBody { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Struct_Unnamed2 {
    pub root: *mut cpBody,
    pub next: *mut cpBody,
    pub idleTime: cpFloat,
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Struct_Unnamed2 { unsafe { ::std::mem::zeroed() } }
}
pub type Enum_cpArbiterState = ::libc::c_uint;
pub const CP_ARBITER_STATE_FIRST_COLLISION: ::libc::c_uint = 0;
pub const CP_ARBITER_STATE_NORMAL: ::libc::c_uint = 1;
pub const CP_ARBITER_STATE_IGNORE: ::libc::c_uint = 2;
pub const CP_ARBITER_STATE_CACHED: ::libc::c_uint = 3;
pub const CP_ARBITER_STATE_INVALIDATED: ::libc::c_uint = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpArbiterThread {
    pub next: *mut Struct_cpArbiter,
    pub prev: *mut Struct_cpArbiter,
}
impl ::std::default::Default for Struct_cpArbiterThread {
    fn default() -> Struct_cpArbiterThread { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpContact {
    pub r1: cpVect,
    pub r2: cpVect,
    pub nMass: cpFloat,
    pub tMass: cpFloat,
    pub bounce: cpFloat,
    pub jnAcc: cpFloat,
    pub jtAcc: cpFloat,
    pub jBias: cpFloat,
    pub bias: cpFloat,
    pub hash: cpHashValue,
}
impl ::std::default::Default for Struct_cpContact {
    fn default() -> Struct_cpContact { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpCollisionInfo {
    pub a: *const cpShape,
    pub b: *const cpShape,
    pub id: cpCollisionID,
    pub n: cpVect,
    pub count: ::libc::c_int,
    pub arr: *mut Struct_cpContact,
}
impl ::std::default::Default for Struct_cpCollisionInfo {
    fn default() -> Struct_cpCollisionInfo { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpArbiter {
    pub e: cpFloat,
    pub u: cpFloat,
    pub surface_vr: cpVect,
    pub data: cpDataPointer,
    pub a: *const cpShape,
    pub b: *const cpShape,
    pub body_a: *mut cpBody,
    pub body_b: *mut cpBody,
    pub thread_a: Struct_cpArbiterThread,
    pub thread_b: Struct_cpArbiterThread,
    pub count: ::libc::c_int,
    pub contacts: *mut Struct_cpContact,
    pub n: cpVect,
    pub handler: *mut cpCollisionHandler,
    pub handlerA: *mut cpCollisionHandler,
    pub handlerB: *mut cpCollisionHandler,
    pub swapped: cpBool,
    pub stamp: cpTimestamp,
    pub state: Enum_cpArbiterState,
}
impl ::std::default::Default for Struct_cpArbiter {
    fn default() -> Struct_cpArbiter { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpShapeMassInfo {
    pub m: cpFloat,
    pub i: cpFloat,
    pub cog: cpVect,
    pub area: cpFloat,
}
impl ::std::default::Default for Struct_cpShapeMassInfo {
    fn default() -> Struct_cpShapeMassInfo { unsafe { ::std::mem::zeroed() } }
}
pub type Enum_cpShapeType = ::libc::c_uint;
pub const CP_CIRCLE_SHAPE: ::libc::c_uint = 0;
pub const CP_SEGMENT_SHAPE: ::libc::c_uint = 1;
pub const CP_POLY_SHAPE: ::libc::c_uint = 2;
pub const CP_NUM_SHAPES: ::libc::c_uint = 3;
pub type cpShapeType = Enum_cpShapeType;
pub type cpShapeCacheDataImpl =
    ::std::option::Option<extern "C" fn
                              (shape: *mut cpShape, transform: cpTransform)
                              -> cpBB>;
pub type cpShapeDestroyImpl =
    ::std::option::Option<extern "C" fn(shape: *mut cpShape) -> ()>;
pub type cpShapePointQueryImpl =
    ::std::option::Option<extern "C" fn
                              (shape: *const cpShape, p: cpVect,
                               info: *mut cpPointQueryInfo) -> ()>;
pub type cpShapeSegmentQueryImpl =
    ::std::option::Option<extern "C" fn
                              (shape: *const cpShape, a: cpVect, b: cpVect,
                               radius: cpFloat, info: *mut cpSegmentQueryInfo)
                              -> ()>;
pub type cpShapeClass = Struct_cpShapeClass;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpShapeClass {
    pub _type: cpShapeType,
    pub cacheData: cpShapeCacheDataImpl,
    pub destroy: cpShapeDestroyImpl,
    pub pointQuery: cpShapePointQueryImpl,
    pub segmentQuery: cpShapeSegmentQueryImpl,
}
impl ::std::default::Default for Struct_cpShapeClass {
    fn default() -> Struct_cpShapeClass { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpShape {
    pub klass: *const cpShapeClass,
    pub space: *mut cpSpace,
    pub body: *mut cpBody,
    pub massInfo: Struct_cpShapeMassInfo,
    pub bb: cpBB,
    pub sensor: cpBool,
    pub e: cpFloat,
    pub u: cpFloat,
    pub surfaceV: cpVect,
    pub userData: cpDataPointer,
    pub _type: cpCollisionType,
    pub filter: cpShapeFilter,
    pub next: *mut cpShape,
    pub prev: *mut cpShape,
    pub hashid: cpHashValue,
}
impl ::std::default::Default for Struct_cpShape {
    fn default() -> Struct_cpShape { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpCircleShape {
    pub shape: cpShape,
    pub c: cpVect,
    pub tc: cpVect,
    pub r: cpFloat,
}
impl ::std::default::Default for Struct_cpCircleShape {
    fn default() -> Struct_cpCircleShape { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpSegmentShape {
    pub shape: cpShape,
    pub a: cpVect,
    pub b: cpVect,
    pub n: cpVect,
    pub ta: cpVect,
    pub tb: cpVect,
    pub tn: cpVect,
    pub r: cpFloat,
    pub a_tangent: cpVect,
    pub b_tangent: cpVect,
}
impl ::std::default::Default for Struct_cpSegmentShape {
    fn default() -> Struct_cpSegmentShape { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpSplittingPlane {
    pub v0: cpVect,
    pub n: cpVect,
}
impl ::std::default::Default for Struct_cpSplittingPlane {
    fn default() -> Struct_cpSplittingPlane {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpPolyShape {
    pub shape: cpShape,
    pub r: cpFloat,
    pub count: ::libc::c_int,
    pub planes: *mut Struct_cpSplittingPlane,
    pub _planes: [Struct_cpSplittingPlane; 12usize],
}
impl ::std::default::Default for Struct_cpPolyShape {
    fn default() -> Struct_cpPolyShape { unsafe { ::std::mem::zeroed() } }
}
pub type cpConstraintPreStepImpl =
    ::std::option::Option<extern "C" fn
                              (constraint: *mut cpConstraint, dt: cpFloat)
                              -> ()>;
pub type cpConstraintApplyCachedImpulseImpl =
    ::std::option::Option<extern "C" fn
                              (constraint: *mut cpConstraint,
                               dt_coef: cpFloat) -> ()>;
pub type cpConstraintApplyImpulseImpl =
    ::std::option::Option<extern "C" fn
                              (constraint: *mut cpConstraint, dt: cpFloat)
                              -> ()>;
pub type cpConstraintGetImpulseImpl =
    ::std::option::Option<extern "C" fn(constraint: *mut cpConstraint)
                              -> cpFloat>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpConstraintClass {
    pub preStep: cpConstraintPreStepImpl,
    pub applyCachedImpulse: cpConstraintApplyCachedImpulseImpl,
    pub applyImpulse: cpConstraintApplyImpulseImpl,
    pub getImpulse: cpConstraintGetImpulseImpl,
}
impl ::std::default::Default for Struct_cpConstraintClass {
    fn default() -> Struct_cpConstraintClass {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type cpConstraintClass = Struct_cpConstraintClass;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpConstraint {
    pub klass: *const cpConstraintClass,
    pub space: *mut cpSpace,
    pub a: *mut cpBody,
    pub b: *mut cpBody,
    pub next_a: *mut cpConstraint,
    pub next_b: *mut cpConstraint,
    pub maxForce: cpFloat,
    pub errorBias: cpFloat,
    pub maxBias: cpFloat,
    pub collideBodies: cpBool,
    pub preSolve: cpConstraintPreSolveFunc,
    pub postSolve: cpConstraintPostSolveFunc,
    pub userData: cpDataPointer,
}
impl ::std::default::Default for Struct_cpConstraint {
    fn default() -> Struct_cpConstraint { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpPinJoint {
    pub constraint: cpConstraint,
    pub anchorA: cpVect,
    pub anchorB: cpVect,
    pub dist: cpFloat,
    pub r1: cpVect,
    pub r2: cpVect,
    pub n: cpVect,
    pub nMass: cpFloat,
    pub jnAcc: cpFloat,
    pub bias: cpFloat,
}
impl ::std::default::Default for Struct_cpPinJoint {
    fn default() -> Struct_cpPinJoint { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpSlideJoint {
    pub constraint: cpConstraint,
    pub anchorA: cpVect,
    pub anchorB: cpVect,
    pub min: cpFloat,
    pub max: cpFloat,
    pub r1: cpVect,
    pub r2: cpVect,
    pub n: cpVect,
    pub nMass: cpFloat,
    pub jnAcc: cpFloat,
    pub bias: cpFloat,
}
impl ::std::default::Default for Struct_cpSlideJoint {
    fn default() -> Struct_cpSlideJoint { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpPivotJoint {
    pub constraint: cpConstraint,
    pub anchorA: cpVect,
    pub anchorB: cpVect,
    pub r1: cpVect,
    pub r2: cpVect,
    pub k: cpMat2x2,
    pub jAcc: cpVect,
    pub bias: cpVect,
}
impl ::std::default::Default for Struct_cpPivotJoint {
    fn default() -> Struct_cpPivotJoint { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpGrooveJoint {
    pub constraint: cpConstraint,
    pub grv_n: cpVect,
    pub grv_a: cpVect,
    pub grv_b: cpVect,
    pub anchorB: cpVect,
    pub grv_tn: cpVect,
    pub clamp: cpFloat,
    pub r1: cpVect,
    pub r2: cpVect,
    pub k: cpMat2x2,
    pub jAcc: cpVect,
    pub bias: cpVect,
}
impl ::std::default::Default for Struct_cpGrooveJoint {
    fn default() -> Struct_cpGrooveJoint { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpDampedSpring {
    pub constraint: cpConstraint,
    pub anchorA: cpVect,
    pub anchorB: cpVect,
    pub restLength: cpFloat,
    pub stiffness: cpFloat,
    pub damping: cpFloat,
    pub springForceFunc: cpDampedSpringForceFunc,
    pub target_vrn: cpFloat,
    pub v_coef: cpFloat,
    pub r1: cpVect,
    pub r2: cpVect,
    pub nMass: cpFloat,
    pub n: cpVect,
    pub jAcc: cpFloat,
}
impl ::std::default::Default for Struct_cpDampedSpring {
    fn default() -> Struct_cpDampedSpring { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpDampedRotarySpring {
    pub constraint: cpConstraint,
    pub restAngle: cpFloat,
    pub stiffness: cpFloat,
    pub damping: cpFloat,
    pub springTorqueFunc: cpDampedRotarySpringTorqueFunc,
    pub target_wrn: cpFloat,
    pub w_coef: cpFloat,
    pub iSum: cpFloat,
    pub jAcc: cpFloat,
}
impl ::std::default::Default for Struct_cpDampedRotarySpring {
    fn default() -> Struct_cpDampedRotarySpring {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpRotaryLimitJoint {
    pub constraint: cpConstraint,
    pub min: cpFloat,
    pub max: cpFloat,
    pub iSum: cpFloat,
    pub bias: cpFloat,
    pub jAcc: cpFloat,
}
impl ::std::default::Default for Struct_cpRotaryLimitJoint {
    fn default() -> Struct_cpRotaryLimitJoint {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpRatchetJoint {
    pub constraint: cpConstraint,
    pub angle: cpFloat,
    pub phase: cpFloat,
    pub ratchet: cpFloat,
    pub iSum: cpFloat,
    pub bias: cpFloat,
    pub jAcc: cpFloat,
}
impl ::std::default::Default for Struct_cpRatchetJoint {
    fn default() -> Struct_cpRatchetJoint { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpGearJoint {
    pub constraint: cpConstraint,
    pub phase: cpFloat,
    pub ratio: cpFloat,
    pub ratio_inv: cpFloat,
    pub iSum: cpFloat,
    pub bias: cpFloat,
    pub jAcc: cpFloat,
}
impl ::std::default::Default for Struct_cpGearJoint {
    fn default() -> Struct_cpGearJoint { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpSimpleMotor {
    pub constraint: cpConstraint,
    pub rate: cpFloat,
    pub iSum: cpFloat,
    pub jAcc: cpFloat,
}
impl ::std::default::Default for Struct_cpSimpleMotor {
    fn default() -> Struct_cpSimpleMotor { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct_cpContactBufferHeader { }
pub type cpContactBufferHeader = Struct_cpContactBufferHeader;
pub type cpSpaceArbiterApplyImpulseFunc =
    ::std::option::Option<extern "C" fn(arb: *mut cpArbiter) -> ()>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpSpace {
    pub iterations: ::libc::c_int,
    pub gravity: cpVect,
    pub damping: cpFloat,
    pub idleSpeedThreshold: cpFloat,
    pub sleepTimeThreshold: cpFloat,
    pub collisionSlop: cpFloat,
    pub collisionBias: cpFloat,
    pub collisionPersistence: cpTimestamp,
    pub userData: cpDataPointer,
    pub stamp: cpTimestamp,
    pub curr_dt: cpFloat,
    pub dynamicBodies: *mut cpArray,
    pub staticBodies: *mut cpArray,
    pub rousedBodies: *mut cpArray,
    pub sleepingComponents: *mut cpArray,
    pub shapeIDCounter: cpHashValue,
    pub staticShapes: *mut cpSpatialIndex,
    pub dynamicShapes: *mut cpSpatialIndex,
    pub constraints: *mut cpArray,
    pub arbiters: *mut cpArray,
    pub contactBuffersHead: *mut cpContactBufferHeader,
    pub cachedArbiters: *mut cpHashSet,
    pub pooledArbiters: *mut cpArray,
    pub allocatedBuffers: *mut cpArray,
    pub locked: ::libc::c_uint,
    pub usesWildcards: cpBool,
    pub collisionHandlers: *mut cpHashSet,
    pub defaultHandler: cpCollisionHandler,
    pub skipPostStep: cpBool,
    pub postStepCallbacks: *mut cpArray,
    pub staticBody: *mut cpBody,
    pub _staticBody: cpBody,
}
impl ::std::default::Default for Struct_cpSpace {
    fn default() -> Struct_cpSpace { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_cpPostStepCallback {
    pub func: cpPostStepFunc,
    pub key: *mut ::libc::c_void,
    pub data: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_cpPostStepCallback {
    fn default() -> Struct_cpPostStepCallback {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type cpPostStepCallback = Struct_cpPostStepCallback;
extern "C" {
    pub static mut cpVersionString: *const ::libc::c_char;
    pub static mut cpCollisionHandlerDoNothing: cpCollisionHandler;
}
extern "C" {
    pub fn cpMessage(condition: *const ::libc::c_char,
                     file: *const ::libc::c_char, line: ::libc::c_int,
                     isError: ::libc::c_int, isHardError: ::libc::c_int,
                     message: *const ::libc::c_char, ...) -> ();
    pub fn cpSpaceHashAlloc() -> *mut cpSpaceHash;
    pub fn cpSpaceHashInit(hash: *mut cpSpaceHash, celldim: cpFloat,
                           numcells: ::libc::c_int,
                           bbfunc: cpSpatialIndexBBFunc,
                           staticIndex: *mut cpSpatialIndex)
     -> *mut cpSpatialIndex;
    pub fn cpSpaceHashNew(celldim: cpFloat, cells: ::libc::c_int,
                          bbfunc: cpSpatialIndexBBFunc,
                          staticIndex: *mut cpSpatialIndex)
     -> *mut cpSpatialIndex;
    pub fn cpSpaceHashResize(hash: *mut cpSpaceHash, celldim: cpFloat,
                             numcells: ::libc::c_int) -> ();
    pub fn cpBBTreeAlloc() -> *mut cpBBTree;
    pub fn cpBBTreeInit(tree: *mut cpBBTree, bbfunc: cpSpatialIndexBBFunc,
                        staticIndex: *mut cpSpatialIndex)
     -> *mut cpSpatialIndex;
    pub fn cpBBTreeNew(bbfunc: cpSpatialIndexBBFunc,
                       staticIndex: *mut cpSpatialIndex)
     -> *mut cpSpatialIndex;
    pub fn cpBBTreeOptimize(index: *mut cpSpatialIndex) -> ();
    pub fn cpBBTreeSetVelocityFunc(index: *mut cpSpatialIndex,
                                   func: cpBBTreeVelocityFunc) -> ();
    pub fn cpSweep1DAlloc() -> *mut cpSweep1D;
    pub fn cpSweep1DInit(sweep: *mut cpSweep1D, bbfunc: cpSpatialIndexBBFunc,
                         staticIndex: *mut cpSpatialIndex)
     -> *mut cpSpatialIndex;
    pub fn cpSweep1DNew(bbfunc: cpSpatialIndexBBFunc,
                        staticIndex: *mut cpSpatialIndex)
     -> *mut cpSpatialIndex;
    pub fn cpSpatialIndexFree(index: *mut cpSpatialIndex) -> ();
    pub fn cpSpatialIndexCollideStatic(dynamicIndex: *mut cpSpatialIndex,
                                       staticIndex: *mut cpSpatialIndex,
                                       func: cpSpatialIndexQueryFunc,
                                       data: *mut ::libc::c_void) -> ();
    pub fn cpArbiterGetRestitution(arb: *const cpArbiter) -> cpFloat;
    pub fn cpArbiterSetRestitution(arb: *mut cpArbiter, restitution: cpFloat)
     -> ();
    pub fn cpArbiterGetFriction(arb: *const cpArbiter) -> cpFloat;
    pub fn cpArbiterSetFriction(arb: *mut cpArbiter, friction: cpFloat) -> ();
    pub fn cpArbiterGetSurfaceVelocity(arb: *const cpArbiter) -> cpVect;
    pub fn cpArbiterSetSurfaceVelocity(arb: *mut cpArbiter, vr: cpVect) -> ();
    pub fn cpArbiterGetUserData(arb: *const cpArbiter) -> cpDataPointer;
    pub fn cpArbiterSetUserData(arb: *mut cpArbiter, userData: cpDataPointer)
     -> ();
    pub fn cpArbiterTotalImpulse(arb: *const cpArbiter) -> cpVect;
    pub fn cpArbiterTotalKE(arb: *const cpArbiter) -> cpFloat;
    pub fn cpArbiterIgnore(arb: *mut cpArbiter) -> cpBool;
    pub fn cpArbiterGetShapes(arb: *const cpArbiter, a: *mut *mut cpShape,
                              b: *mut *mut cpShape) -> ();
    pub fn cpArbiterGetBodies(arb: *const cpArbiter, a: *mut *mut cpBody,
                              b: *mut *mut cpBody) -> ();
    pub fn cpArbiterGetContactPointSet(arb: *const cpArbiter)
     -> cpContactPointSet;
    pub fn cpArbiterSetContactPointSet(arb: *mut cpArbiter,
                                       set: *mut cpContactPointSet) -> ();
    pub fn cpArbiterIsFirstContact(arb: *const cpArbiter) -> cpBool;
    pub fn cpArbiterIsRemoval(arb: *const cpArbiter) -> cpBool;
    pub fn cpArbiterGetCount(arb: *const cpArbiter) -> ::libc::c_int;
    pub fn cpArbiterGetNormal(arb: *const cpArbiter) -> cpVect;
    pub fn cpArbiterGetPointA(arb: *const cpArbiter, i: ::libc::c_int)
     -> cpVect;
    pub fn cpArbiterGetPointB(arb: *const cpArbiter, i: ::libc::c_int)
     -> cpVect;
    pub fn cpArbiterGetDepth(arb: *const cpArbiter, i: ::libc::c_int)
     -> cpFloat;
    pub fn cpArbiterCallWildcardBeginA(arb: *mut cpArbiter,
                                       space: *mut cpSpace) -> cpBool;
    pub fn cpArbiterCallWildcardBeginB(arb: *mut cpArbiter,
                                       space: *mut cpSpace) -> cpBool;
    pub fn cpArbiterCallWildcardPreSolveA(arb: *mut cpArbiter,
                                          space: *mut cpSpace) -> cpBool;
    pub fn cpArbiterCallWildcardPreSolveB(arb: *mut cpArbiter,
                                          space: *mut cpSpace) -> cpBool;
    pub fn cpArbiterCallWildcardPostSolveA(arb: *mut cpArbiter,
                                           space: *mut cpSpace) -> ();
    pub fn cpArbiterCallWildcardPostSolveB(arb: *mut cpArbiter,
                                           space: *mut cpSpace) -> ();
    pub fn cpArbiterCallWildcardSeparateA(arb: *mut cpArbiter,
                                          space: *mut cpSpace) -> ();
    pub fn cpArbiterCallWildcardSeparateB(arb: *mut cpArbiter,
                                          space: *mut cpSpace) -> ();
    pub fn cpBodyAlloc() -> *mut cpBody;
    pub fn cpBodyInit(body: *mut cpBody, mass: cpFloat, moment: cpFloat)
     -> *mut cpBody;
    pub fn cpBodyNew(mass: cpFloat, moment: cpFloat) -> *mut cpBody;
    pub fn cpBodyNewKinematic() -> *mut cpBody;
    pub fn cpBodyNewStatic() -> *mut cpBody;
    pub fn cpBodyDestroy(body: *mut cpBody) -> ();
    pub fn cpBodyFree(body: *mut cpBody) -> ();
    pub fn cpBodyActivate(body: *mut cpBody) -> ();
    pub fn cpBodyActivateStatic(body: *mut cpBody, filter: *mut cpShape)
     -> ();
    pub fn cpBodySleep(body: *mut cpBody) -> ();
    pub fn cpBodySleepWithGroup(body: *mut cpBody, group: *mut cpBody) -> ();
    pub fn cpBodyIsSleeping(body: *const cpBody) -> cpBool;
    pub fn cpBodyGetType(body: *const cpBody) -> cpBodyType;
    pub fn cpBodySetType(body: *mut cpBody, _type: cpBodyType) -> ();
    pub fn cpBodyGetSpace(body: *const cpBody) -> *mut cpSpace;
    pub fn cpBodyGetMass(body: *const cpBody) -> cpFloat;
    pub fn cpBodySetMass(body: *mut cpBody, m: cpFloat) -> ();
    pub fn cpBodyGetMoment(body: *const cpBody) -> cpFloat;
    pub fn cpBodySetMoment(body: *mut cpBody, i: cpFloat) -> ();
    pub fn cpBodyGetPosition(body: *const cpBody) -> cpVect;
    pub fn cpBodySetPosition(body: *mut cpBody, pos: cpVect) -> ();
    pub fn cpBodyGetCenterOfGravity(body: *const cpBody) -> cpVect;
    pub fn cpBodySetCenterOfGravity(body: *mut cpBody, cog: cpVect) -> ();
    pub fn cpBodyGetVelocity(body: *const cpBody) -> cpVect;
    pub fn cpBodySetVelocity(body: *mut cpBody, velocity: cpVect) -> ();
    pub fn cpBodyGetForce(body: *const cpBody) -> cpVect;
    pub fn cpBodySetForce(body: *mut cpBody, force: cpVect) -> ();
    pub fn cpBodyGetAngle(body: *const cpBody) -> cpFloat;
    pub fn cpBodySetAngle(body: *mut cpBody, a: cpFloat) -> ();
    pub fn cpBodyGetAngularVelocity(body: *const cpBody) -> cpFloat;
    pub fn cpBodySetAngularVelocity(body: *mut cpBody,
                                    angularVelocity: cpFloat) -> ();
    pub fn cpBodyGetTorque(body: *const cpBody) -> cpFloat;
    pub fn cpBodySetTorque(body: *mut cpBody, torque: cpFloat) -> ();
    pub fn cpBodyGetRotation(body: *const cpBody) -> cpVect;
    pub fn cpBodyGetUserData(body: *const cpBody) -> cpDataPointer;
    pub fn cpBodySetUserData(body: *mut cpBody, userData: cpDataPointer)
     -> ();
    pub fn cpBodySetVelocityUpdateFunc(body: *mut cpBody,
                                       velocityFunc: cpBodyVelocityFunc)
     -> ();
    pub fn cpBodySetPositionUpdateFunc(body: *mut cpBody,
                                       positionFunc: cpBodyPositionFunc)
     -> ();
    pub fn cpBodyUpdateVelocity(body: *mut cpBody, gravity: cpVect,
                                damping: cpFloat, dt: cpFloat) -> ();
    pub fn cpBodyUpdatePosition(body: *mut cpBody, dt: cpFloat) -> ();
    pub fn cpBodyLocalToWorld(body: *const cpBody, point: cpVect) -> cpVect;
    pub fn cpBodyWorldToLocal(body: *const cpBody, point: cpVect) -> cpVect;
    pub fn cpBodyApplyForceAtWorldPoint(body: *mut cpBody, force: cpVect,
                                        point: cpVect) -> ();
    pub fn cpBodyApplyForceAtLocalPoint(body: *mut cpBody, force: cpVect,
                                        point: cpVect) -> ();
    pub fn cpBodyApplyImpulseAtWorldPoint(body: *mut cpBody, impulse: cpVect,
                                          point: cpVect) -> ();
    pub fn cpBodyApplyImpulseAtLocalPoint(body: *mut cpBody, impulse: cpVect,
                                          point: cpVect) -> ();
    pub fn cpBodyGetVelocityAtWorldPoint(body: *const cpBody, point: cpVect)
     -> cpVect;
    pub fn cpBodyGetVelocityAtLocalPoint(body: *const cpBody, point: cpVect)
     -> cpVect;
    pub fn cpBodyKineticEnergy(body: *const cpBody) -> cpFloat;
    pub fn cpBodyEachShape(body: *mut cpBody, func: cpBodyShapeIteratorFunc,
                           data: *mut ::libc::c_void) -> ();
    pub fn cpBodyEachConstraint(body: *mut cpBody,
                                func: cpBodyConstraintIteratorFunc,
                                data: *mut ::libc::c_void) -> ();
    pub fn cpBodyEachArbiter(body: *mut cpBody,
                             func: cpBodyArbiterIteratorFunc,
                             data: *mut ::libc::c_void) -> ();
    pub fn cpShapeDestroy(shape: *mut cpShape) -> ();
    pub fn cpShapeFree(shape: *mut cpShape) -> ();
    pub fn cpShapeCacheBB(shape: *mut cpShape) -> cpBB;
    pub fn cpShapeUpdate(shape: *mut cpShape, transform: cpTransform) -> cpBB;
    pub fn cpShapePointQuery(shape: *const cpShape, p: cpVect,
                             out: *mut cpPointQueryInfo) -> cpFloat;
    pub fn cpShapeSegmentQuery(shape: *const cpShape, a: cpVect, b: cpVect,
                               radius: cpFloat, info: *mut cpSegmentQueryInfo)
     -> cpBool;
    pub fn cpShapesCollide(a: *const cpShape, b: *const cpShape)
     -> cpContactPointSet;
    pub fn cpShapeGetSpace(shape: *const cpShape) -> *mut cpSpace;
    pub fn cpShapeGetBody(shape: *const cpShape) -> *mut cpBody;
    pub fn cpShapeSetBody(shape: *mut cpShape, body: *mut cpBody) -> ();
    pub fn cpShapeGetMass(shape: *const cpShape) -> cpFloat;
    pub fn cpShapeSetMass(shape: *mut cpShape, mass: cpFloat) -> ();
    pub fn cpShapeGetDensity(shape: *const cpShape) -> cpFloat;
    pub fn cpShapeSetDensity(shape: *mut cpShape, density: cpFloat) -> ();
    pub fn cpShapeGetMoment(shape: *const cpShape) -> cpFloat;
    pub fn cpShapeGetArea(shape: *const cpShape) -> cpFloat;
    pub fn cpShapeGetCenterOfGravity(shape: *const cpShape) -> cpVect;
    pub fn cpShapeGetBB(shape: *const cpShape) -> cpBB;
    pub fn cpShapeGetSensor(shape: *const cpShape) -> cpBool;
    pub fn cpShapeSetSensor(shape: *mut cpShape, sensor: cpBool) -> ();
    pub fn cpShapeGetElasticity(shape: *const cpShape) -> cpFloat;
    pub fn cpShapeSetElasticity(shape: *mut cpShape, elasticity: cpFloat)
     -> ();
    pub fn cpShapeGetFriction(shape: *const cpShape) -> cpFloat;
    pub fn cpShapeSetFriction(shape: *mut cpShape, friction: cpFloat) -> ();
    pub fn cpShapeGetSurfaceVelocity(shape: *const cpShape) -> cpVect;
    pub fn cpShapeSetSurfaceVelocity(shape: *mut cpShape,
                                     surfaceVelocity: cpVect) -> ();
    pub fn cpShapeGetUserData(shape: *const cpShape) -> cpDataPointer;
    pub fn cpShapeSetUserData(shape: *mut cpShape, userData: cpDataPointer)
     -> ();
    pub fn cpShapeGetCollisionType(shape: *const cpShape) -> cpCollisionType;
    pub fn cpShapeSetCollisionType(shape: *mut cpShape,
                                   collisionType: cpCollisionType) -> ();
    pub fn cpShapeGetFilter(shape: *const cpShape) -> cpShapeFilter;
    pub fn cpShapeSetFilter(shape: *mut cpShape, filter: cpShapeFilter) -> ();
    pub fn cpCircleShapeAlloc() -> *mut cpCircleShape;
    pub fn cpCircleShapeInit(circle: *mut cpCircleShape, body: *mut cpBody,
                             radius: cpFloat, offset: cpVect)
     -> *mut cpCircleShape;
    pub fn cpCircleShapeNew(body: *mut cpBody, radius: cpFloat,
                            offset: cpVect) -> *mut cpShape;
    pub fn cpCircleShapeGetOffset(shape: *const cpShape) -> cpVect;
    pub fn cpCircleShapeGetRadius(shape: *const cpShape) -> cpFloat;
    pub fn cpSegmentShapeAlloc() -> *mut cpSegmentShape;
    pub fn cpSegmentShapeInit(seg: *mut cpSegmentShape, body: *mut cpBody,
                              a: cpVect, b: cpVect, radius: cpFloat)
     -> *mut cpSegmentShape;
    pub fn cpSegmentShapeNew(body: *mut cpBody, a: cpVect, b: cpVect,
                             radius: cpFloat) -> *mut cpShape;
    pub fn cpSegmentShapeSetNeighbors(shape: *mut cpShape, prev: cpVect,
                                      next: cpVect) -> ();
    pub fn cpSegmentShapeGetA(shape: *const cpShape) -> cpVect;
    pub fn cpSegmentShapeGetB(shape: *const cpShape) -> cpVect;
    pub fn cpSegmentShapeGetNormal(shape: *const cpShape) -> cpVect;
    pub fn cpSegmentShapeGetRadius(shape: *const cpShape) -> cpFloat;
    pub fn cpPolyShapeAlloc() -> *mut cpPolyShape;
    pub fn cpPolyShapeInit(poly: *mut cpPolyShape, body: *mut cpBody,
                           count: ::libc::c_int, verts: *const cpVect,
                           transform: cpTransform, radius: cpFloat)
     -> *mut cpPolyShape;
    pub fn cpPolyShapeInitRaw(poly: *mut cpPolyShape, body: *mut cpBody,
                              count: ::libc::c_int, verts: *const cpVect,
                              radius: cpFloat) -> *mut cpPolyShape;
    pub fn cpPolyShapeNew(body: *mut cpBody, count: ::libc::c_int,
                          verts: *const cpVect, transform: cpTransform,
                          radius: cpFloat) -> *mut cpShape;
    pub fn cpPolyShapeNewRaw(body: *mut cpBody, count: ::libc::c_int,
                             verts: *const cpVect, radius: cpFloat)
     -> *mut cpShape;
    pub fn cpBoxShapeInit(poly: *mut cpPolyShape, body: *mut cpBody,
                          width: cpFloat, height: cpFloat, radius: cpFloat)
     -> *mut cpPolyShape;
    pub fn cpBoxShapeInit2(poly: *mut cpPolyShape, body: *mut cpBody,
                           _box: cpBB, radius: cpFloat) -> *mut cpPolyShape;
    pub fn cpBoxShapeNew(body: *mut cpBody, width: cpFloat, height: cpFloat,
                         radius: cpFloat) -> *mut cpShape;
    pub fn cpBoxShapeNew2(body: *mut cpBody, _box: cpBB, radius: cpFloat)
     -> *mut cpShape;
    pub fn cpPolyShapeGetCount(shape: *const cpShape) -> ::libc::c_int;
    pub fn cpPolyShapeGetVert(shape: *const cpShape, index: ::libc::c_int)
     -> cpVect;
    pub fn cpPolyShapeGetRadius(shape: *const cpShape) -> cpFloat;
    pub fn cpConstraintDestroy(constraint: *mut cpConstraint) -> ();
    pub fn cpConstraintFree(constraint: *mut cpConstraint) -> ();
    pub fn cpConstraintGetSpace(constraint: *const cpConstraint)
     -> *mut cpSpace;
    pub fn cpConstraintGetBodyA(constraint: *const cpConstraint)
     -> *mut cpBody;
    pub fn cpConstraintGetBodyB(constraint: *const cpConstraint)
     -> *mut cpBody;
    pub fn cpConstraintGetMaxForce(constraint: *const cpConstraint)
     -> cpFloat;
    pub fn cpConstraintSetMaxForce(constraint: *mut cpConstraint,
                                   maxForce: cpFloat) -> ();
    pub fn cpConstraintGetErrorBias(constraint: *const cpConstraint)
     -> cpFloat;
    pub fn cpConstraintSetErrorBias(constraint: *mut cpConstraint,
                                    errorBias: cpFloat) -> ();
    pub fn cpConstraintGetMaxBias(constraint: *const cpConstraint) -> cpFloat;
    pub fn cpConstraintSetMaxBias(constraint: *mut cpConstraint,
                                  maxBias: cpFloat) -> ();
    pub fn cpConstraintGetCollideBodies(constraint: *const cpConstraint)
     -> cpBool;
    pub fn cpConstraintSetCollideBodies(constraint: *mut cpConstraint,
                                        collideBodies: cpBool) -> ();
    pub fn cpConstraintGetPreSolveFunc(constraint: *const cpConstraint)
     -> cpConstraintPreSolveFunc;
    pub fn cpConstraintSetPreSolveFunc(constraint: *mut cpConstraint,
                                       preSolveFunc: cpConstraintPreSolveFunc)
     -> ();
    pub fn cpConstraintGetPostSolveFunc(constraint: *const cpConstraint)
     -> cpConstraintPostSolveFunc;
    pub fn cpConstraintSetPostSolveFunc(constraint: *mut cpConstraint,
                                        postSolveFunc:
                                            cpConstraintPostSolveFunc) -> ();
    pub fn cpConstraintGetUserData(constraint: *const cpConstraint)
     -> cpDataPointer;
    pub fn cpConstraintSetUserData(constraint: *mut cpConstraint,
                                   userData: cpDataPointer) -> ();
    pub fn cpConstraintGetImpulse(constraint: *const cpConstraint) -> cpFloat;
    pub fn cpConstraintIsPinJoint(constraint: *const cpConstraint) -> cpBool;
    pub fn cpPinJointAlloc() -> *mut cpPinJoint;
    pub fn cpPinJointInit(joint: *mut cpPinJoint, a: *mut cpBody,
                          b: *mut cpBody, anchorA: cpVect, anchorB: cpVect)
     -> *mut cpPinJoint;
    pub fn cpPinJointNew(a: *mut cpBody, b: *mut cpBody, anchorA: cpVect,
                         anchorB: cpVect) -> *mut cpConstraint;
    pub fn cpPinJointGetAnchorA(constraint: *const cpConstraint) -> cpVect;
    pub fn cpPinJointSetAnchorA(constraint: *mut cpConstraint,
                                anchorA: cpVect) -> ();
    pub fn cpPinJointGetAnchorB(constraint: *const cpConstraint) -> cpVect;
    pub fn cpPinJointSetAnchorB(constraint: *mut cpConstraint,
                                anchorB: cpVect) -> ();
    pub fn cpPinJointGetDist(constraint: *const cpConstraint) -> cpFloat;
    pub fn cpPinJointSetDist(constraint: *mut cpConstraint, dist: cpFloat)
     -> ();
    pub fn cpConstraintIsSlideJoint(constraint: *const cpConstraint)
     -> cpBool;
    pub fn cpSlideJointAlloc() -> *mut cpSlideJoint;
    pub fn cpSlideJointInit(joint: *mut cpSlideJoint, a: *mut cpBody,
                            b: *mut cpBody, anchorA: cpVect, anchorB: cpVect,
                            min: cpFloat, max: cpFloat) -> *mut cpSlideJoint;
    pub fn cpSlideJointNew(a: *mut cpBody, b: *mut cpBody, anchorA: cpVect,
                           anchorB: cpVect, min: cpFloat, max: cpFloat)
     -> *mut cpConstraint;
    pub fn cpSlideJointGetAnchorA(constraint: *const cpConstraint) -> cpVect;
    pub fn cpSlideJointSetAnchorA(constraint: *mut cpConstraint,
                                  anchorA: cpVect) -> ();
    pub fn cpSlideJointGetAnchorB(constraint: *const cpConstraint) -> cpVect;
    pub fn cpSlideJointSetAnchorB(constraint: *mut cpConstraint,
                                  anchorB: cpVect) -> ();
    pub fn cpSlideJointGetMin(constraint: *const cpConstraint) -> cpFloat;
    pub fn cpSlideJointSetMin(constraint: *mut cpConstraint, min: cpFloat)
     -> ();
    pub fn cpSlideJointGetMax(constraint: *const cpConstraint) -> cpFloat;
    pub fn cpSlideJointSetMax(constraint: *mut cpConstraint, max: cpFloat)
     -> ();
    pub fn cpConstraintIsPivotJoint(constraint: *const cpConstraint)
     -> cpBool;
    pub fn cpPivotJointAlloc() -> *mut cpPivotJoint;
    pub fn cpPivotJointInit(joint: *mut cpPivotJoint, a: *mut cpBody,
                            b: *mut cpBody, anchorA: cpVect, anchorB: cpVect)
     -> *mut cpPivotJoint;
    pub fn cpPivotJointNew(a: *mut cpBody, b: *mut cpBody, pivot: cpVect)
     -> *mut cpConstraint;
    pub fn cpPivotJointNew2(a: *mut cpBody, b: *mut cpBody, anchorA: cpVect,
                            anchorB: cpVect) -> *mut cpConstraint;
    pub fn cpPivotJointGetAnchorA(constraint: *const cpConstraint) -> cpVect;
    pub fn cpPivotJointSetAnchorA(constraint: *mut cpConstraint,
                                  anchorA: cpVect) -> ();
    pub fn cpPivotJointGetAnchorB(constraint: *const cpConstraint) -> cpVect;
    pub fn cpPivotJointSetAnchorB(constraint: *mut cpConstraint,
                                  anchorB: cpVect) -> ();
    pub fn cpConstraintIsGrooveJoint(constraint: *const cpConstraint)
     -> cpBool;
    pub fn cpGrooveJointAlloc() -> *mut cpGrooveJoint;
    pub fn cpGrooveJointInit(joint: *mut cpGrooveJoint, a: *mut cpBody,
                             b: *mut cpBody, groove_a: cpVect,
                             groove_b: cpVect, anchorB: cpVect)
     -> *mut cpGrooveJoint;
    pub fn cpGrooveJointNew(a: *mut cpBody, b: *mut cpBody, groove_a: cpVect,
                            groove_b: cpVect, anchorB: cpVect)
     -> *mut cpConstraint;
    pub fn cpGrooveJointGetGrooveA(constraint: *const cpConstraint) -> cpVect;
    pub fn cpGrooveJointSetGrooveA(constraint: *mut cpConstraint,
                                   grooveA: cpVect) -> ();
    pub fn cpGrooveJointGetGrooveB(constraint: *const cpConstraint) -> cpVect;
    pub fn cpGrooveJointSetGrooveB(constraint: *mut cpConstraint,
                                   grooveB: cpVect) -> ();
    pub fn cpGrooveJointGetAnchorB(constraint: *const cpConstraint) -> cpVect;
    pub fn cpGrooveJointSetAnchorB(constraint: *mut cpConstraint,
                                   anchorB: cpVect) -> ();
    pub fn cpConstraintIsDampedSpring(constraint: *const cpConstraint)
     -> cpBool;
    pub fn cpDampedSpringAlloc() -> *mut cpDampedSpring;
    pub fn cpDampedSpringInit(joint: *mut cpDampedSpring, a: *mut cpBody,
                              b: *mut cpBody, anchorA: cpVect,
                              anchorB: cpVect, restLength: cpFloat,
                              stiffness: cpFloat, damping: cpFloat)
     -> *mut cpDampedSpring;
    pub fn cpDampedSpringNew(a: *mut cpBody, b: *mut cpBody, anchorA: cpVect,
                             anchorB: cpVect, restLength: cpFloat,
                             stiffness: cpFloat, damping: cpFloat)
     -> *mut cpConstraint;
    pub fn cpDampedSpringGetAnchorA(constraint: *const cpConstraint)
     -> cpVect;
    pub fn cpDampedSpringSetAnchorA(constraint: *mut cpConstraint,
                                    anchorA: cpVect) -> ();
    pub fn cpDampedSpringGetAnchorB(constraint: *const cpConstraint)
     -> cpVect;
    pub fn cpDampedSpringSetAnchorB(constraint: *mut cpConstraint,
                                    anchorB: cpVect) -> ();
    pub fn cpDampedSpringGetRestLength(constraint: *const cpConstraint)
     -> cpFloat;
    pub fn cpDampedSpringSetRestLength(constraint: *mut cpConstraint,
                                       restLength: cpFloat) -> ();
    pub fn cpDampedSpringGetStiffness(constraint: *const cpConstraint)
     -> cpFloat;
    pub fn cpDampedSpringSetStiffness(constraint: *mut cpConstraint,
                                      stiffness: cpFloat) -> ();
    pub fn cpDampedSpringGetDamping(constraint: *const cpConstraint)
     -> cpFloat;
    pub fn cpDampedSpringSetDamping(constraint: *mut cpConstraint,
                                    damping: cpFloat) -> ();
    pub fn cpDampedSpringGetSpringForceFunc(constraint: *const cpConstraint)
     -> cpDampedSpringForceFunc;
    pub fn cpDampedSpringSetSpringForceFunc(constraint: *mut cpConstraint,
                                            springForceFunc:
                                                cpDampedSpringForceFunc)
     -> ();
    pub fn cpConstraintIsDampedRotarySpring(constraint: *const cpConstraint)
     -> cpBool;
    pub fn cpDampedRotarySpringAlloc() -> *mut cpDampedRotarySpring;
    pub fn cpDampedRotarySpringInit(joint: *mut cpDampedRotarySpring,
                                    a: *mut cpBody, b: *mut cpBody,
                                    restAngle: cpFloat, stiffness: cpFloat,
                                    damping: cpFloat)
     -> *mut cpDampedRotarySpring;
    pub fn cpDampedRotarySpringNew(a: *mut cpBody, b: *mut cpBody,
                                   restAngle: cpFloat, stiffness: cpFloat,
                                   damping: cpFloat) -> *mut cpConstraint;
    pub fn cpDampedRotarySpringGetRestAngle(constraint: *const cpConstraint)
     -> cpFloat;
    pub fn cpDampedRotarySpringSetRestAngle(constraint: *mut cpConstraint,
                                            restAngle: cpFloat) -> ();
    pub fn cpDampedRotarySpringGetStiffness(constraint: *const cpConstraint)
     -> cpFloat;
    pub fn cpDampedRotarySpringSetStiffness(constraint: *mut cpConstraint,
                                            stiffness: cpFloat) -> ();
    pub fn cpDampedRotarySpringGetDamping(constraint: *const cpConstraint)
     -> cpFloat;
    pub fn cpDampedRotarySpringSetDamping(constraint: *mut cpConstraint,
                                          damping: cpFloat) -> ();
    pub fn cpDampedRotarySpringGetSpringTorqueFunc(constraint:
                                                       *const cpConstraint)
     -> cpDampedRotarySpringTorqueFunc;
    pub fn cpDampedRotarySpringSetSpringTorqueFunc(constraint:
                                                       *mut cpConstraint,
                                                   springTorqueFunc:
                                                       cpDampedRotarySpringTorqueFunc)
     -> ();
    pub fn cpConstraintIsRotaryLimitJoint(constraint: *const cpConstraint)
     -> cpBool;
    pub fn cpRotaryLimitJointAlloc() -> *mut cpRotaryLimitJoint;
    pub fn cpRotaryLimitJointInit(joint: *mut cpRotaryLimitJoint,
                                  a: *mut cpBody, b: *mut cpBody,
                                  min: cpFloat, max: cpFloat)
     -> *mut cpRotaryLimitJoint;
    pub fn cpRotaryLimitJointNew(a: *mut cpBody, b: *mut cpBody, min: cpFloat,
                                 max: cpFloat) -> *mut cpConstraint;
    pub fn cpRotaryLimitJointGetMin(constraint: *const cpConstraint)
     -> cpFloat;
    pub fn cpRotaryLimitJointSetMin(constraint: *mut cpConstraint,
                                    min: cpFloat) -> ();
    pub fn cpRotaryLimitJointGetMax(constraint: *const cpConstraint)
     -> cpFloat;
    pub fn cpRotaryLimitJointSetMax(constraint: *mut cpConstraint,
                                    max: cpFloat) -> ();
    pub fn cpConstraintIsRatchetJoint(constraint: *const cpConstraint)
     -> cpBool;
    pub fn cpRatchetJointAlloc() -> *mut cpRatchetJoint;
    pub fn cpRatchetJointInit(joint: *mut cpRatchetJoint, a: *mut cpBody,
                              b: *mut cpBody, phase: cpFloat,
                              ratchet: cpFloat) -> *mut cpRatchetJoint;
    pub fn cpRatchetJointNew(a: *mut cpBody, b: *mut cpBody, phase: cpFloat,
                             ratchet: cpFloat) -> *mut cpConstraint;
    pub fn cpRatchetJointGetAngle(constraint: *const cpConstraint) -> cpFloat;
    pub fn cpRatchetJointSetAngle(constraint: *mut cpConstraint,
                                  angle: cpFloat) -> ();
    pub fn cpRatchetJointGetPhase(constraint: *const cpConstraint) -> cpFloat;
    pub fn cpRatchetJointSetPhase(constraint: *mut cpConstraint,
                                  phase: cpFloat) -> ();
    pub fn cpRatchetJointGetRatchet(constraint: *const cpConstraint)
     -> cpFloat;
    pub fn cpRatchetJointSetRatchet(constraint: *mut cpConstraint,
                                    ratchet: cpFloat) -> ();
    pub fn cpConstraintIsGearJoint(constraint: *const cpConstraint) -> cpBool;
    pub fn cpGearJointAlloc() -> *mut cpGearJoint;
    pub fn cpGearJointInit(joint: *mut cpGearJoint, a: *mut cpBody,
                           b: *mut cpBody, phase: cpFloat, ratio: cpFloat)
     -> *mut cpGearJoint;
    pub fn cpGearJointNew(a: *mut cpBody, b: *mut cpBody, phase: cpFloat,
                          ratio: cpFloat) -> *mut cpConstraint;
    pub fn cpGearJointGetPhase(constraint: *const cpConstraint) -> cpFloat;
    pub fn cpGearJointSetPhase(constraint: *mut cpConstraint, phase: cpFloat)
     -> ();
    pub fn cpGearJointGetRatio(constraint: *const cpConstraint) -> cpFloat;
    pub fn cpGearJointSetRatio(constraint: *mut cpConstraint, ratio: cpFloat)
     -> ();
    pub fn cpConstraintIsSimpleMotor(constraint: *const cpConstraint)
     -> cpBool;
    pub fn cpSimpleMotorAlloc() -> *mut cpSimpleMotor;
    pub fn cpSimpleMotorInit(joint: *mut cpSimpleMotor, a: *mut cpBody,
                             b: *mut cpBody, rate: cpFloat)
     -> *mut cpSimpleMotor;
    pub fn cpSimpleMotorNew(a: *mut cpBody, b: *mut cpBody, rate: cpFloat)
     -> *mut cpConstraint;
    pub fn cpSimpleMotorGetRate(constraint: *const cpConstraint) -> cpFloat;
    pub fn cpSimpleMotorSetRate(constraint: *mut cpConstraint, rate: cpFloat)
     -> ();
    pub fn cpSpaceAlloc() -> *mut cpSpace;
    pub fn cpSpaceInit(space: *mut cpSpace) -> *mut cpSpace;
    pub fn cpSpaceNew() -> *mut cpSpace;
    pub fn cpSpaceDestroy(space: *mut cpSpace) -> ();
    pub fn cpSpaceFree(space: *mut cpSpace) -> ();
    pub fn cpSpaceGetIterations(space: *const cpSpace) -> ::libc::c_int;
    pub fn cpSpaceSetIterations(space: *mut cpSpace,
                                iterations: ::libc::c_int) -> ();
    pub fn cpSpaceGetGravity(space: *const cpSpace) -> cpVect;
    pub fn cpSpaceSetGravity(space: *mut cpSpace, gravity: cpVect) -> ();
    pub fn cpSpaceGetDamping(space: *const cpSpace) -> cpFloat;
    pub fn cpSpaceSetDamping(space: *mut cpSpace, damping: cpFloat) -> ();
    pub fn cpSpaceGetIdleSpeedThreshold(space: *const cpSpace) -> cpFloat;
    pub fn cpSpaceSetIdleSpeedThreshold(space: *mut cpSpace,
                                        idleSpeedThreshold: cpFloat) -> ();
    pub fn cpSpaceGetSleepTimeThreshold(space: *const cpSpace) -> cpFloat;
    pub fn cpSpaceSetSleepTimeThreshold(space: *mut cpSpace,
                                        sleepTimeThreshold: cpFloat) -> ();
    pub fn cpSpaceGetCollisionSlop(space: *const cpSpace) -> cpFloat;
    pub fn cpSpaceSetCollisionSlop(space: *mut cpSpace,
                                   collisionSlop: cpFloat) -> ();
    pub fn cpSpaceGetCollisionBias(space: *const cpSpace) -> cpFloat;
    pub fn cpSpaceSetCollisionBias(space: *mut cpSpace,
                                   collisionBias: cpFloat) -> ();
    pub fn cpSpaceGetCollisionPersistence(space: *const cpSpace)
     -> cpTimestamp;
    pub fn cpSpaceSetCollisionPersistence(space: *mut cpSpace,
                                          collisionPersistence: cpTimestamp)
     -> ();
    pub fn cpSpaceGetUserData(space: *const cpSpace) -> cpDataPointer;
    pub fn cpSpaceSetUserData(space: *mut cpSpace, userData: cpDataPointer)
     -> ();
    pub fn cpSpaceGetStaticBody(space: *const cpSpace) -> *mut cpBody;
    pub fn cpSpaceGetCurrentTimeStep(space: *const cpSpace) -> cpFloat;
    pub fn cpSpaceIsLocked(space: *mut cpSpace) -> cpBool;
    pub fn cpSpaceAddDefaultCollisionHandler(space: *mut cpSpace)
     -> *mut cpCollisionHandler;
    pub fn cpSpaceAddCollisionHandler(space: *mut cpSpace, a: cpCollisionType,
                                      b: cpCollisionType)
     -> *mut cpCollisionHandler;
    pub fn cpSpaceAddWildcardHandler(space: *mut cpSpace,
                                     _type: cpCollisionType)
     -> *mut cpCollisionHandler;
    pub fn cpSpaceAddShape(space: *mut cpSpace, shape: *mut cpShape)
     -> *mut cpShape;
    pub fn cpSpaceAddBody(space: *mut cpSpace, body: *mut cpBody)
     -> *mut cpBody;
    pub fn cpSpaceAddConstraint(space: *mut cpSpace,
                                constraint: *mut cpConstraint)
     -> *mut cpConstraint;
    pub fn cpSpaceRemoveShape(space: *mut cpSpace, shape: *mut cpShape) -> ();
    pub fn cpSpaceRemoveBody(space: *mut cpSpace, body: *mut cpBody) -> ();
    pub fn cpSpaceRemoveConstraint(space: *mut cpSpace,
                                   constraint: *mut cpConstraint) -> ();
    pub fn cpSpaceContainsShape(space: *const cpSpace, shape: *const cpShape)
     -> cpBool;
    pub fn cpSpaceContainsBody(space: *const cpSpace, body: *const cpBody)
     -> cpBool;
    pub fn cpSpaceContainsConstraint(space: *const cpSpace,
                                     constraint: *const cpConstraint) -> cpBool;
    pub fn cpSpaceAddPostStepCallback(space: *mut cpSpace,
                                      func: cpPostStepFunc,
                                      key: *mut ::libc::c_void,
                                      data: *mut ::libc::c_void) -> cpBool;
    pub fn cpSpacePointQuery(space: *mut cpSpace, point: cpVect,
                             maxDistance: cpFloat, filter: cpShapeFilter,
                             func: cpSpacePointQueryFunc,
                             data: *mut ::libc::c_void) -> ();
    pub fn cpSpacePointQueryNearest(space: *mut cpSpace, point: cpVect,
                                    maxDistance: cpFloat,
                                    filter: cpShapeFilter,
                                    out: *mut cpPointQueryInfo)
     -> *mut cpShape;
    pub fn cpSpaceSegmentQuery(space: *mut cpSpace, start: cpVect,
                               end: cpVect, radius: cpFloat,
                               filter: cpShapeFilter,
                               func: cpSpaceSegmentQueryFunc,
                               data: *mut ::libc::c_void) -> ();
    pub fn cpSpaceSegmentQueryFirst(space: *mut cpSpace, start: cpVect,
                                    end: cpVect, radius: cpFloat,
                                    filter: cpShapeFilter,
                                    out: *mut cpSegmentQueryInfo)
     -> *mut cpShape;
    pub fn cpSpaceBBQuery(space: *mut cpSpace, bb: cpBB,
                          filter: cpShapeFilter, func: cpSpaceBBQueryFunc,
                          data: *mut ::libc::c_void) -> ();
    pub fn cpSpaceShapeQuery(space: *mut cpSpace, shape: *mut cpShape,
                             func: cpSpaceShapeQueryFunc,
                             data: *mut ::libc::c_void) -> cpBool;
    pub fn cpSpaceEachBody(space: *mut cpSpace, func: cpSpaceBodyIteratorFunc,
                           data: *mut ::libc::c_void) -> ();
    pub fn cpSpaceEachShape(space: *mut cpSpace,
                            func: cpSpaceShapeIteratorFunc,
                            data: *mut ::libc::c_void) -> ();
    pub fn cpSpaceEachConstraint(space: *mut cpSpace,
                                 func: cpSpaceConstraintIteratorFunc,
                                 data: *mut ::libc::c_void) -> ();
    pub fn cpSpaceReindexStatic(space: *mut cpSpace) -> ();
    pub fn cpSpaceReindexShape(space: *mut cpSpace, shape: *mut cpShape)
     -> ();
    pub fn cpSpaceReindexShapesForBody(space: *mut cpSpace, body: *mut cpBody)
     -> ();
    pub fn cpSpaceUseSpatialHash(space: *mut cpSpace, dim: cpFloat,
                                 count: ::libc::c_int) -> ();
    pub fn cpSpaceStep(space: *mut cpSpace, dt: cpFloat) -> ();
    pub fn cpSpaceDebugDraw(space: *mut cpSpace,
                            options: *mut cpSpaceDebugDrawOptions) -> ();
    pub fn cpMomentForCircle(m: cpFloat, r1: cpFloat, r2: cpFloat,
                             offset: cpVect) -> cpFloat;
    pub fn cpAreaForCircle(r1: cpFloat, r2: cpFloat) -> cpFloat;
    pub fn cpMomentForSegment(m: cpFloat, a: cpVect, b: cpVect,
                              radius: cpFloat) -> cpFloat;
    pub fn cpAreaForSegment(a: cpVect, b: cpVect, radius: cpFloat) -> cpFloat;
    pub fn cpMomentForPoly(m: cpFloat, count: ::libc::c_int,
                           verts: *const cpVect, offset: cpVect,
                           radius: cpFloat) -> cpFloat;
    pub fn cpAreaForPoly(count: ::libc::c_int, verts: *const cpVect,
                         radius: cpFloat) -> cpFloat;
    pub fn cpCentroidForPoly(count: ::libc::c_int, verts: *const cpVect)
     -> cpVect;
    pub fn cpMomentForBox(m: cpFloat, width: cpFloat, height: cpFloat)
     -> cpFloat;
    pub fn cpMomentForBox2(m: cpFloat, _box: cpBB) -> cpFloat;
    pub fn cpConvexHull(count: ::libc::c_int, verts: *const cpVect,
                        result: *mut cpVect, first: *mut ::libc::c_int,
                        tol: cpFloat) -> ::libc::c_int;
    pub fn cpSpaceEachBody_b(space: *mut cpSpace, block: ::libc::c_void)
     -> ();
    pub fn cpSpaceEachShape_b(space: *mut cpSpace, block: ::libc::c_void)
     -> ();
    pub fn cpSpaceEachConstraint_b(space: *mut cpSpace, block: ::libc::c_void)
     -> ();
    pub fn cpBodyEachShape_b(body: *mut cpBody, block: ::libc::c_void) -> ();
    pub fn cpBodyEachConstraint_b(body: *mut cpBody, block: ::libc::c_void)
     -> ();
    pub fn cpBodyEachArbiter_b(body: *mut cpBody, block: ::libc::c_void)
     -> ();
    pub fn cpSpacePointQuery_b(space: *mut cpSpace, point: cpVect,
                               maxDistance: cpFloat, filter: cpShapeFilter,
                               block: cpSpacePointQueryBlock) -> ();
    pub fn cpSpaceSegmentQuery_b(space: *mut cpSpace, start: cpVect,
                                 end: cpVect, radius: cpFloat,
                                 filter: cpShapeFilter,
                                 block: cpSpaceSegmentQueryBlock) -> ();
    pub fn cpSpaceBBQuery_b(space: *mut cpSpace, bb: cpBB,
                            filter: cpShapeFilter, block: cpSpaceBBQueryBlock)
     -> ();
    pub fn cpSpaceShapeQuery_b(space: *mut cpSpace, shape: *mut cpShape,
                               block: cpSpaceShapeQueryBlock) -> cpBool;
    pub fn cpArrayNew(size: ::libc::c_int) -> *mut cpArray;
    pub fn cpArrayFree(arr: *mut cpArray) -> ();
    pub fn cpArrayPush(arr: *mut cpArray, object: *mut ::libc::c_void) -> ();
    pub fn cpArrayPop(arr: *mut cpArray) -> *mut ::libc::c_void;
    pub fn cpArrayDeleteObj(arr: *mut cpArray, obj: *mut ::libc::c_void)
     -> ();
    pub fn cpArrayContains(arr: *const cpArray, ptr: *const ::libc::c_void)
     -> cpBool;
    pub fn cpArrayFreeEach(arr: *mut cpArray, freeFunc: ::libc::c_void) -> ();
    pub fn cpHashSetNew(size: ::libc::c_int, eqlFunc: cpHashSetEqlFunc)
     -> *mut cpHashSet;
    pub fn cpHashSetSetDefaultValue(set: *mut cpHashSet,
                                    default_value: *mut ::libc::c_void) -> ();
    pub fn cpHashSetFree(set: *mut cpHashSet) -> ();
    pub fn cpHashSetCount(set: *mut cpHashSet) -> ::libc::c_int;
    pub fn cpHashSetInsert(set: *mut cpHashSet, hash: cpHashValue,
                           ptr: *mut ::libc::c_void,
                           trans: cpHashSetTransFunc,
                           data: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn cpHashSetRemove(set: *mut cpHashSet, hash: cpHashValue,
                           ptr: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn cpHashSetFind(set: *mut cpHashSet, hash: cpHashValue,
                         ptr: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn cpHashSetEach(set: *mut cpHashSet, func: cpHashSetIteratorFunc,
                         data: *mut ::libc::c_void) -> ();
    pub fn cpHashSetFilter(set: *mut cpHashSet, func: cpHashSetFilterFunc,
                           data: *mut ::libc::c_void) -> ();
    pub fn cpBodyAddShape(body: *mut cpBody, shape: *mut cpShape) -> ();
    pub fn cpBodyRemoveShape(body: *mut cpBody, shape: *mut cpShape) -> ();
    pub fn cpBodyAccumulateMassFromShapes(body: *mut cpBody) -> ();
    pub fn cpBodyRemoveConstraint(body: *mut cpBody,
                                  constraint: *mut cpConstraint) -> ();
    pub fn cpSpatialIndexInit(index: *mut cpSpatialIndex,
                              klass: *mut cpSpatialIndexClass,
                              bbfunc: cpSpatialIndexBBFunc,
                              staticIndex: *mut cpSpatialIndex)
     -> *mut cpSpatialIndex;
    pub fn cpArbiterInit(arb: *mut cpArbiter, a: *mut cpShape,
                         b: *mut cpShape) -> *mut cpArbiter;
    pub fn cpArbiterUnthread(arb: *mut cpArbiter) -> ();
    pub fn cpArbiterUpdate(arb: *mut cpArbiter,
                           info: *mut Struct_cpCollisionInfo,
                           space: *mut cpSpace) -> ();
    pub fn cpArbiterPreStep(arb: *mut cpArbiter, dt: cpFloat, bias: cpFloat,
                            slop: cpFloat) -> ();
    pub fn cpArbiterApplyCachedImpulse(arb: *mut cpArbiter, dt_coef: cpFloat)
     -> ();
    pub fn cpArbiterApplyImpulse(arb: *mut cpArbiter) -> ();
    pub fn cpShapeInit(shape: *mut cpShape, klass: *const cpShapeClass,
                       body: *mut cpBody, massInfo: Struct_cpShapeMassInfo)
     -> *mut cpShape;
    pub fn cpCollide(a: *const cpShape, b: *const cpShape, id: cpCollisionID,
                     contacts: *mut Struct_cpContact)
     -> Struct_cpCollisionInfo;
    pub fn cpLoopIndexes(verts: *const cpVect, count: ::libc::c_int,
                         start: *mut ::libc::c_int, end: *mut ::libc::c_int)
     -> ();
    pub fn cpConstraintInit(constraint: *mut cpConstraint,
                            klass: *const Struct_cpConstraintClass,
                            a: *mut cpBody, b: *mut cpBody) -> ();
    pub fn cpSpaceSetStaticBody(space: *mut cpSpace, body: *mut cpBody) -> ();
    pub fn cpSpaceProcessComponents(space: *mut cpSpace, dt: cpFloat) -> ();
    pub fn cpSpacePushFreshContactBuffer(space: *mut cpSpace) -> ();
    pub fn cpContactBufferGetArray(space: *mut cpSpace)
     -> *mut Struct_cpContact;
    pub fn cpSpacePushContacts(space: *mut cpSpace, count: ::libc::c_int)
     -> ();
    pub fn cpSpaceGetPostStepCallback(space: *mut cpSpace,
                                      key: *mut ::libc::c_void)
     -> *mut cpPostStepCallback;
    pub fn cpSpaceArbiterSetFilter(arb: *mut cpArbiter, space: *mut cpSpace)
     -> cpBool;
    pub fn cpSpaceFilterArbiters(space: *mut cpSpace, body: *mut cpBody,
                                 filter: *mut cpShape) -> ();
    pub fn cpSpaceActivateBody(space: *mut cpSpace, body: *mut cpBody) -> ();
    pub fn cpSpaceLock(space: *mut cpSpace) -> ();
    pub fn cpSpaceUnlock(space: *mut cpSpace, runPostStep: cpBool) -> ();
    pub fn cpShapeUpdateFunc(shape: *mut cpShape, unused: *mut ::libc::c_void)
     -> ();
    pub fn cpSpaceCollideShapes(a: *mut cpShape, b: *mut cpShape,
                                id: cpCollisionID, space: *mut cpSpace)
     -> cpCollisionID;
}
