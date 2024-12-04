use std::cell::Cell;
use docx_rs::{Docx, Paragraph, Run, Table, TableBorder, TableBorders, TableCell, TableCellBorder, TableRow};
use rand::Rng;
use std::fs::File;
use docx_rs::XMLElement::TableCellBorders;

#[derive(Copy, Clone, Debug)]
enum Op {
    ADD, SUB
}

impl Op {
   fn print_str(&self) -> String {
       match self {
           Op::ADD => String::from("+"),
           Op::SUB => String::from("-")
       }
   }
}

#[derive(Debug)]
struct BinaryExpression {
    a: i32,
    op: Op,
    b: i32
}

impl BinaryExpression {
    fn result(&self) -> i32 {
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
    a: i32,
    op1: Op,
    b: i32,
    op2: Op,
    c: i32
}

impl ThreeBitExpression {
    fn validate(&self) -> bool {
        let b1 = self.first_binary();
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

    fn first_binary(&self) -> BinaryExpression {
        BinaryExpression {
            a: self.a,
            op: self.op1,
            b: self.b
        }
    }
}

fn rand_operation() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=10)
}

fn rand_op() -> Op {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0..=2) == 1 {
        return Op::SUB
    }
    Op::ADD
}

fn gen_binary_expression() -> BinaryExpression {
    for _i in 0..100 {
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
    for _i in 0..100 {
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
        op1: Op::ADD,
        b: 0,
        op2: Op::ADD,
        c: 0
    }
}

impl BinaryExpression {
    fn print_str(&self) -> String {
        format!("{} {} {} =", self.a, self.op.print_str(), self.b)
    }
}

impl ThreeBitExpression {
    fn print_str(&self) -> String {
        format!("{} {} {} {} {} =", self.a, self.op1.print_str(), self.b, self.op2.print_str(), self.c)
    }
}

fn main() {
    let binary_col_count = 3;
    let three_bit_col_count = 1;
    let row_count = 20;
    let cell_width = 0.8;
    let cell_height = 0.8;

    let mut rows: Vec<TableRow> = Vec::new();
    for _r in 0..row_count {
        let mut cols: Vec<TableCell> = Vec::new();
        for _c_binary in 0..binary_col_count {
            let cell = TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(
                gen_binary_expression().print_str()
            ))).clear_all_border();
            cols.push(cell);
        }
        for _c_three_bit in 0..three_bit_col_count {
            let cell = TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(
                gen_binary_three_bit().print_str()
            ))).clear_all_border();
            cols.push(cell);
        }
        rows.push(TableRow::new(cols));
    }
    let table = Table::new(rows).clear_all_border();
    Docx::new().add_table(table).build().pack(File::create("output.docx").unwrap()).unwrap();
}
