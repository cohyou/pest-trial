use std::fs;
use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "briqs.pest"]
struct BriqsParser;

enum Briq {
    Piq,
}

struct Printer {
    nest: usize,
}

impl Printer {
    fn print_briq(&mut self, briq: Pair<Rule>) {
        // let parts = briq.into_inner();
        // let part = parts.next().unwrap().as_rule();
        // if part == Rule::label {

        // }
        // match part.next().unwrap().as_rule() {
        //     Rule::label => print!("{}", part.as_str()),
        //     Rule::symbol => print!("{}", part.as_str()),
        //     Rule::piq => print_piq(part),
        //     Rule::list => print_list(part),
        //     _ => unimplemented!(),
        // }

        for part in briq.into_inner() {
            match part.as_rule() {
                Rule::label => print!("{}", part.as_str()),
                Rule::symbol => print!("{}", part.as_str()),
                Rule::piq => self.print_piq(part),
                Rule::list => {
                    self.print_list(part)
                },
                _ => unimplemented!(),
            }
        }        
    }

    fn print_list(&mut self, briq: Pair<Rule>) {
        print!("{}", " [");
        self.nest += 1;
        self.print_briqs(briq.into_inner().next().unwrap());
        self.nest -= 1;
        println!();
        self.print_indent();
        print!("{}", "]");
    }

    fn print_piq(&mut self, briq: Pair<Rule>) {
        let mut inner_rules = briq.into_inner();

        print!("{}", "(");
        let p = inner_rules.next().unwrap();
        self.print_briq(p);
        print!("{}", " ");
        let q = inner_rules.next().unwrap();
        self.print_briq(q);
        print!("{}", ")");
    }

    fn print_indent(&self) {
        for _ in 0..self.nest {
            print!("  ");
        }
    }

    fn print_briqs(&mut self, briqs: Pair<Rule>) {

        for briq in briqs.into_inner() {
            println!();
            match briq.as_rule() {
                Rule::briq => {
                    self.print_indent();
                    self.print_briq(briq);
                },
                _ => unimplemented!(),
            }                
        }
    }
}

pub fn parse_briqs() {
    let unparsed_file = fs::read_to_string("_.iq").expect("cannot read file");

    let file = BriqsParser::parse(Rule::briqs, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails;
    
    let mut printer = Printer { nest: 0 };
    printer.print_briqs(file);
    println!();
    // println!("{}", file.into_inner());
}