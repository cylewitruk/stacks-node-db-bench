@0x9ed9fd33879420d9;

struct Optional(T) {
    union {
        none @0 :Void;
        some @1 :T;
    }
}

struct Map(Key, Value) {
  entries @0 :List(Entry);
  struct Entry {
    key @0 :Key;
    value @1 :Value;
  }
}

struct Set(T) {
    elements @0 :List(Value);
    struct Value {
        value @0 :T;
    }
}

struct Tuple(T1, T2) {
    t1 @0 :T1;
    t2 @1 :T2;
}

struct Thruple(T1, T2, T3) {
    t1 @0 :T1;
    t2 @1 :T2;
    t3 @2 :T3;
}

struct Int128 {
    low @0 :UInt64;
    high @1 :UInt64;
}

struct UInt128 {
    low @0 :UInt64;
    high @1 :UInt64;
}