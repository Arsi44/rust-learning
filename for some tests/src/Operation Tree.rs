// Operation Tree

// Return
trait Operation {
    fn apply(&mut self) -> i32;
}

impl Operation for i32 {
    fn apply(&mut self) -> i32 {
        *self
    }
}

// Minus
struct UnaryMinus<A>(A);

impl <A: Operation> Operation for UnaryMinus<A> {
    fn apply(&mut self) -> i32 {
        -self.0.apply()
    }
}

// Plus
struct Plus<A, B> {
    a: A,
    b: B,
}

impl<A: Operation, B: Operation> Operation for Plus<A, B> {
    fn apply(&mut self) -> i32 {
        self.a.apply() + self.b.apply()
    }
}

// Cache
enum Cache<T> {
    Operation(T),
    Result(i32)
}

impl<T: Operation> Operation for Cache<T> {
    fn apply(&mut self) -> i32 {
        match self {
            Cache::Operation(o) => {
                let result = o.apply();
                *self = Self::Result(result);
                result
            }
            Cache::Result(r) => *r,
        }
    }
}


fn main() {
    let three_plus_two = Plus { a: 3, b: 2};

    let cached_three_plus_two = Cache::Operation(three_plus_two);

    let minus_one = UnaryMinus(1);

    let mut three_plus_two_minus_one = Plus {
        a: cached_three_plus_two,
        b: minus_one,
    };

    let result = three_plus_two_minus_one.apply();
    print!("3 + 2 - 1 = {result} \n");
}