#![feature(slicing_syntax)]
extern crate checked;
use checked::Checked;

// Calculator with abort on overflow
type Unchecked = i32;
// try changing to above type and compiling with
//     rustc -O --emit=asm,link -C llvm_args=--x86-asm-syntax=intel -Z no-landing-pads calculator.rs
// to see the assembly in calculator.s

fn main() {
    match std::os::args()[] {
        [_, ref arg1, ref op, ref arg2] => {
            let arg1 : Checked<Unchecked> = Checked(from_str(arg1[]).unwrap());
            let arg2 : Checked<Unchecked> = Checked(from_str(arg2[]).unwrap());
            let result = match op[] {
                "+" => arg1 + arg2,
                "-" => arg1 - arg2,
                "/" => arg1 / arg2,
                "*" => arg1 * arg2,
                _ => panic!("Invalid op: use one of +-/*"),
            };
            println!("{}", *result);
        }
        _ => panic!("Usage: calculator arg1 op arg2"),
    }
}

