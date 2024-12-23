use std::env;
use std::process;
use std::fs::File;
use std::io::{BufReader, Read};
use anyhow::{Context, Result};

struct Lexer <R: Read>{
    source: R,
    location: Location,
    peeked_token: Option<Token>,
}

#[derive(Debug, Copy, Clone)]
struct Location{
    line: usize,
    column: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
        }
    }
}

#[derive(Debug, Clone)]
struct Token{
    char: char,
    location: Location,
}


enum Operation{
    AddrRight, // >,
    AddrLeft, // <,
    Inc, // +,
    Sub, // -,
    Output, // .,
    Input, // ,
    JmpForward, // [,
    JmpBack, // ],
}


impl <R: Read> Lexer<R> {
    fn new(source: R) -> Self {
        Self { source,
        location: Location::default(),
        peeked_token: None,
        }
    }

    fn in_language(candidate: char) -> bool {
        match candidate {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
            _ => false,
        }
    }

    fn next(&mut self) -> Result<Option<Token>>{

        if let Some(token) = self.peeked_token.clone(){
            self.peeked_token = None;
            return Ok(Some(token));
        }


        let mut buf: [u8; 1] = [0; 1];
        while !Self::in_language(buf[0].into()){

            let read_bytes = self.source.read(&mut buf).context("Read next buyte from the source file")?;

            self.location.column += 1;

            if buf[0] == b'\n' {
                self.location.line += 1;
                self.location.column = 1;
            }

            if read_bytes != 1{
                return Ok(None);
            }

        }


        Ok(Some(Token{
            char: buf[0].into(),
            location: self.location,
        }))


    }


    fn peek(&mut self) -> Result<Option<Token>> {
        if let Some(token) = &self.peeked_token {
            return Ok(Some(token.clone()));
        }

        self.peeked_token = self.next().context("reading next token to peek at it")?;
        Ok(self.peeked_token.clone())
    }

}

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let (command, args) = args.split_first().expect("expected to have one argument atleast");

    if args.len() < 1 {
    
        eprintln!("Usage: {} <command> <.bf_file>", command);
        process::exit(1);

    }

    let input = &args[0];

    println!("Opening the brainfu*k file {input} for execution");

    let file = BufReader::new(File::open(input).context("Open the {input} file")?);

    let mut lexer = Lexer::new(file);

    let peeked_token = lexer.peek().context("peek at the first token")?;
    println!("Peeked token: {peeked_token:?}");

    let next_token = lexer.next().context("read the first token")?;
    println!("Next token: {next_token:?}");

    while let Some(token) = lexer.next().context("read next token")? {
        println!("{token:?}");
    }



    Ok(())

}
