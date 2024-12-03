use rand::Rng;

#[derive(Copy, Clone, Debug)]
enum Op {
    ADD, SUB
}

#[derive(Debug)]
struct BinaryExpression {
    a: i8,
    op: Op,
    b: i8
}

impl BinaryExpression {
    fn result(&self) -> i8 {
        match self.op {
            Op::ADD => self.a + self.b,
            Op::SUB => self.a - self.b
        }
    }

    fn validate(&self) -> bool {
        let _rst = self.result();
        _rst >= 0 && _rst <= 10
    }
}

#[derive(Debug)]
struct ThreeBitExpression {
    a: i8,
    op1: Op,
    b: i8,
    op2: Op,
    c: i8
}

impl ThreeBitExpression {
    fn validate(&self) -> bool {
        let b1 = BinaryExpression {
            a: self.a,
            op: self.op1,
            b: self.b
        };
        if !b1.validate() {
            return false;
        }
        let b2 = BinaryExpression {
            a: b1.result(),
            op: self.op2,
            b: self.c
        };
        b2.validate()
    }
}

fn rand_operation() -> i8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=10)
}

fn rand_op() -> Op {
    let mut rng = rand::thread_rng();
    if (rng.gen_range(0..=2) == 1) {
        return Op::SUB
    }
    Op::ADD
}

fn gen_binary_expression() -> BinaryExpression {
    for i in 0..100 {
        let a = rand_operation();
        let op = rand_op();
        let b = rand_operation();
        let b = BinaryExpression {
            a, b, op
        };
        if b.validate() {
            return b
        }
    }

    BinaryExpression {
        a: 0,
        b: 0,
        op: Op::ADD
    }
}

fn gen_binary_three_bit() -> ThreeBitExpression {
    for i in 0..100 {
        let a = rand_operation();
        let op1 = rand_op();
        let b = rand_operation();
        let op2 = rand_op();
        let c = rand_operation();
        let e = ThreeBitExpression {
            a, op1, b, op2, c
        };
        if e.validate() {
            return e
        }
    }

    ThreeBitExpression {
        a: 0,
        op1,
        b: 0,
        op2,
        c
    }
}

fn main() {
    let binary_col_count = 3;
    let three_bit_col_count = 1;
    let row_count = 20;

    // gen all binary
    let mut binary_expressions: Vec<Vec<BinaryExpression>> = Vec::new();
    println!("{:?}", gen_binary_expression())
}
