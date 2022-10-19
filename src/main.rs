use std::fs;

#[derive(Copy, PartialEq, Clone, Debug)]
enum Operators {
    Plus,
    Put,
    Mult,
    Div,
    Minus,
}

impl core::fmt::Display for Operators {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            Self::Plus => write!(f, "+"),
            Self::Put => write!(f, "put"),
            Self::Minus => write!(f, "-"),
            Self::Div => write!(f, "/"),
            Self::Mult => write!(f, "*"),
        }
    }
}

#[derive(Copy,PartialEq, Clone, Debug)]
enum Literals {
    EmptyLiterals,
    Operator(Operators),
    Integer(u64),
}
/*
impl Literals {
    fn is_operator(&self) -> bool {
        match &*self {
            Literals::Operator(_) => true,
            _ => false,
        }
    }
    fn is_integer(&self) -> bool {
        match &*self {
            Literals::Integer(_) => true,
            _ => false,
        }
    }

}
*/
impl core::fmt::Display for Literals {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            Self::EmptyLiterals => write!(f, "EMPTYLITERALS (ERROR)"),
            Self::Integer(int) => write!(f, "{}", int),
            Self::Operator(op) => write!(f, "{}", op),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct AST {
    node: Literals,
    right_node: Option<Box<AST>>,
    left_node: Option<Box<AST>>,
}

impl AST {
    fn is_empty(self) -> bool{
        if self.left_node.is_none() && self.right_node.is_none() && self.node == Literals::EmptyLiterals {
            return true
        }
        false
    }
    
    fn root(self) -> Literals {
        return self.node;
    }

    fn lhs(self) -> AST {
        return *self.left_node.expect("ERROR: AST was empty");
    }
    
    fn rhs(self) -> AST {
        return *self.right_node.expect("ERROR: AST was empty");
    }

    fn new(literal: Literals, lhs: AST, rhs: AST) -> AST {
        AST{ 
            node: literal,
            left_node: Some(Box::new(lhs)),
            right_node: Some(Box::new(rhs)),
        }
    }

    fn create_empty() -> AST {
        AST {
            node: Literals::EmptyLiterals,
            left_node: None,
            right_node: None,
        }
    }
/*
    fn modify_root(mut self, node: Literals) {
        self.node = node;
    }

    fn modify_lhs(mut self, lhs: AST) {
        self.left_node = Some(Box::new(lhs));
    }

    fn modify_rhs(mut self, rhs: AST) {
        self.right_node = Some(Box::new(rhs));
    }

    fn print(self) {
        if !self.clone().is_empty() {
            print!("("); 
            self.clone().lhs().print();
            print!(" {} ", self.node);
            self.clone().rhs().print(); print!(")");
        }
    }
    */
}

const REG_NAMES: [&str; 7] = ["rbx", "r10", "r11","r12", "r13", "r14", "r15"];

#[derive(Clone, Copy)]
struct ScratchRegisterManagement {
    in_use: [bool; 7],
}

static mut SRM: ScratchRegisterManagement = ScratchRegisterManagement { in_use: [false;7] };

impl ScratchRegisterManagement {
    fn scratch_alloc(&mut self) -> u8 {
        for i in 0..REG_NAMES.len() {
            if !self.in_use[i] {
                self.in_use[i] = true;
                return i as u8;
            }
        }
        eprintln!("No register available");
        std::process::exit(1);
    }

    fn scratch_free(mut self, r: u8) {
        self.in_use[r as usize] = false;
    }

    fn scratch_name(self, r: u8) -> String {
        REG_NAMES[r as usize].to_string()
    }
}
#[derive(Copy, Clone)]
struct LabelGenerator {
    counter: u32,
}

impl LabelGenerator {
    fn label_create(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }

    fn label_name(name: u32) -> String {
        format!(".L{name}:")
    }
}

unsafe fn expr_codegen(ast: AST, label_gen: LabelGenerator) -> (u8, String) {
    //println!("{:?}", SRM.in_use); // TODO no more Int after 7 in a row

    if ast.clone().is_empty() {
        return (0, "".to_string());
    }
    else {
        match ast.clone().root() {
            Literals::EmptyLiterals => {
                eprintln!("ERROR: EmptyLiterals in expr_codegen()"); 
                std::process::exit(1);
            }
            Literals::Integer(int) => {
                let regu = SRM.scratch_alloc();
                let reg = SRM.scratch_name(regu); 
                return (regu, format!("        mov    {reg}, {int}\n"));
            }
            Literals::Operator(Operators::Plus) => {
                let lhs = ast.clone().lhs();
                let rhs = ast.clone().rhs();
                let (regle, mut code) = expr_codegen(lhs, label_gen);
                let (regri, code2)    = expr_codegen(rhs, label_gen);
                
                code += &code2; 

                let reg_left = SRM.scratch_name(regle);
                let reg_right = SRM.scratch_name(regri);
                code += &format!("        add    {reg_right}, {reg_left}\n");
                SRM.scratch_free(regle);
                return (regri, code);
            }
            Literals::Operator(Operators::Minus) => todo!(),
            Literals::Operator(Operators::Mult) => todo!(),
            Literals::Operator(Operators::Div) => todo!(),
            Literals::Operator(Operators::Put) => {
                let (regri, mut code) = expr_codegen(ast.clone().rhs(), label_gen);
                let reg_right = SRM.scratch_name(regri);
                code += &format!("        mov rdi, {reg_right}\n");
                code += &format!("        call put\n");
                SRM.scratch_free(regri);
                return (0, code);
            }
        }
    }

}
fn generate_code(program: Vec<AST>) -> String {
    

    let mut label_gen: LabelGenerator = LabelGenerator { counter: 1 };

    let mut code = "".to_string();

    println!("\n");

    let header = "
BITS 64
%define SYS_EXIT 60
segment .text
global _start
put:
        push    rbp
        mov     rbp, rsp
        sub     rsp, 64
        mov     QWORD [rbp-56], rdi
        mov     DWORD [rbp-4], 1
        mov     eax, DWORD [rbp-4]
        cdqe
        mov     edx, 32
        sub     rdx, rax
        mov     BYTE [rbp-48+rdx], 10
.L0:
        mov     rcx, QWORD [rbp-56]
        mov     rdx, 7378697629483820647
        mov     rax, rcx
        imul    rdx
        sar     rdx, 2
        mov     rax, rcx
        sar     rax, 63
        sub     rdx, rax
        mov     rax, rdx
        sal     rax, 2
        add     rax, rdx
        add     rax, rax
        sub     rcx, rax
        mov     rdx, rcx
        mov     eax, edx
        lea     ecx, [rax+48]
        mov     eax, DWORD [rbp-4]
        lea     edx, [rax+1]
        mov     DWORD [rbp-4], edx
        cdqe
        mov     edx, 31
        sub     rdx, rax
        mov     eax, ecx
        mov     BYTE [rbp-48+rdx], al
        mov     rcx, QWORD [rbp-56]
        mov     rdx, 7378697629483820647
        mov     rax, rcx
        imul    rdx
        mov     rax, rdx
        sar     rax, 2
        sar     rcx, 63
        mov     rdx, rcx
        sub     rax, rdx
        mov     QWORD [rbp-56], rax
        cmp     QWORD [rbp-56], 0
        jg      .L0
        mov     eax, DWORD [rbp-4]
        cdqe
        mov     edx, DWORD [rbp-4]
        movsxd  rdx, DWORD edx
        mov     ecx, 32
        sub     rcx, rdx
        lea     rdx, [rbp-48]
        add     rcx, rdx
        mov     rdx, rax
        mov     rsi, rcx
        mov     edi, 1
        mov     rax, 1
        syscall
        nop
        leave
        ret
_start:
        push    rbp
        mov     rbp, rsp
        sub     rsp, 16
";
    
    

    code = code.to_owned() + header;
    
    for ast in program {
        code += "\n";
        code += &LabelGenerator::label_name(label_gen.label_create());
        code += "\n";
       unsafe { 
            SRM.in_use = [false; 7];
            let (_, code2) = expr_codegen(ast, label_gen); 
            code += &code2;
       }
    }
    
    code +=".LEND:\n        mov     rdi, 0\n        mov    rax, 60\n        syscall";
    return code.to_string();
}


/*
fn run(ast: AST) -> u64{
    if !ast.clone().is_empty() {
        match ast.clone().root() {
            Literals::Operator(Operators::Put) => {
                let rhs = run(ast.clone().rhs());
                print!("{}", rhs);
                return 0;
            },
            Literals::Operator(Operators::Plus) => {
                let lhs = run(ast.clone().lhs());
                let rhs = run(ast.clone().rhs());
                return lhs + rhs;
            },
            Literals::Operator(Operators::Minus) => {
                let lhs = run(ast.clone().lhs());
                let rhs = run(ast.clone().rhs());
                return lhs - rhs;
            },
            Literals::Operator(Operators::Mult) => {
                let lhs = run(ast.clone().lhs());
                let rhs = run(ast.clone().rhs());
                return lhs * rhs;
            }
            Literals::Operator(Operators::Div) => {
                let lhs = run(ast.clone().lhs());
                let rhs = run(ast.clone().rhs());
                return lhs / rhs;
            }
            Literals::EmptyLiterals => {
                eprintln!("ERROR: EmptyLiterals is being traversed"); // @error            
                std::process::exit(1);
            },
            Literals::Integer(number) => {
                return number;
            },
        }
    }
    else {
        return 0;
    }
}
*/
#[derive(Copy,PartialEq, Eq, Debug, Clone)]
enum TokenType {
    Plus,
    Minus,
    Mult,
    Div,
    Semicolon,
    Put,
    OpenParen,
    CloseParen,

    Integer,
    EOF,
}

#[derive(Debug, Clone)]
struct Position {
    line: u32,
    col:  u32,
    file: String,
}

#[derive(Debug, Clone)]
struct Token {
    position: Position,
    lexeme: String,
    type_: TokenType,
    literal: Literals,
}

impl Token {
    fn new(position: Position, lexeme: String, type_: TokenType, literal: Literals) -> Token {
        Token {
            position,
            lexeme,
            type_,
            literal,
        }
    }
}


fn tokenize(program_str: String) -> Vec<Token> {
    // verry verry TODO  this function skip unknown Token
    let mut line: u32 = 0;
    let mut col: u32  = 0;
    let file_path: String = "TODO".to_string(); //TODO
    let mut tokens: Vec<Token> = vec![];
    let mut program_slice = program_str.chars().collect::<Vec<char>>().into_iter(); 
    while !program_str.is_empty() {
        let mut c = program_slice.next().unwrap_or('\0'); //TODO extract value
        // TODO use function NEW for Token
        // TODO rewrite col
        match c { 
            ')' => {
                let token = Token {
                    position: Position {line, col, file: file_path.clone() },
                    lexeme: ")".to_string(),
                    type_: TokenType::CloseParen,
                    literal: Literals::EmptyLiterals,
                };
                tokens.push(token)
            }
            '(' => {
                let token = Token {
                    position: Position {line, col, file: file_path.clone() },
                    lexeme: "(".to_string(),
                    type_: TokenType::OpenParen,
                    literal: Literals::EmptyLiterals,
                };
                tokens.push(token)
            },
            '\0' => {
                let token = Token {
                    position: Position { line, col, file: file_path.clone() }, // TODO
                    lexeme: "EOF".to_string(),
                    type_: TokenType::EOF,
                    literal: Literals::EmptyLiterals,
                };
                tokens.push(token);
                break;
            },
            ';' => {
                let token = Token {
                    position: Position { line, col, file: file_path.clone() }, // TODO
                    lexeme: ";".to_string(),
                    type_: TokenType::Semicolon,
                    literal: Literals::EmptyLiterals,
                };
                tokens.push(token);
                col += 1;
            },
            '+' =>  {
                let token = Token {
                    position: Position { line, col, file: file_path.clone() }, // TODO
                    lexeme: "+".to_string(),
                    type_: TokenType::Plus,
                    literal: Literals::Operator(Operators::Plus),
                };
                tokens.push(token);
                col += 1;
            },
            '-' =>  {
                let token = Token {
                    position: Position { line, col, file: file_path.clone() }, // TODO
                    lexeme: "+".to_string(),
                    type_: TokenType::Minus,
                    literal: Literals::Operator(Operators::Minus),
                };
                tokens.push(token);
                col += 1
            },
            _ => { 
                if c.is_whitespace() { // TODO Tabs pass 1 cols
                    if c == '\n' {
                        col   = 0;
                        line += 1; 
                    }
                    else {
                        col += 1;
                    }
                } else if c.is_numeric() {
                    let mut number_lexeme: Vec<char> = vec![];
                    let mut i = 0;
                    let mut prg_slice_cln = program_slice.clone();
                    while c.is_numeric() {
                        number_lexeme.push(c);
                        c = prg_slice_cln.next().unwrap_or('\0'); // TODO unwrap
                        col += 1;
                        i += 1;
                    }
                    for _ in 0..i-1 {
                        program_slice.next();
                    }
                    let lex = number_lexeme.iter().cloned().collect::<String>();
                    let token = Token {
                        position: Position { line, col: col+1, file: file_path.clone() }, // TODO
                        type_: TokenType::Integer,
                        lexeme: lex.clone(),
                        literal: Literals::Integer(lex.clone().parse().unwrap_or(0)),
                    };
                    tokens.push(token);
                    col += i-1;
                } else if c.is_alphabetic() {
                    let mut number_lexeme: Vec<char> = vec![];
                    let mut i = 0;
                    let mut prg_slice_cln = program_slice.clone();

                    while c.is_alphabetic() {
                        number_lexeme.push(c);
                        c = prg_slice_cln.next().unwrap_or('\0'); // TODO unwrap
                        col += 1;
                        i += 1;
                    }
                    for _ in 0..i-1 {
                        program_slice.next();
                    }
                    let lex = number_lexeme.iter().cloned().collect::<String>();
                    match lex.as_str() {
                    "put" => {
                            let token = Token {
                                position: Position { line, col: col+1, file: file_path.clone() }, // TODO
                                type_: TokenType::Put,
                                lexeme: lex.clone(),
                                literal: Literals::Operator(Operators::Put),
                            };
                            tokens.push(token);
                            col += i-1;
                        }
                    _ => {
                        eprintln!("ERROR: Unexpected word");
                        std::process::exit(1)
                        }
                    }
                }
            }
        }
    }
    return tokens;
}

struct ParsingStruct {
    tokens: Vec<Token>,
    next_token: Token,
    pointer_to_tokens: i32,
}

impl ParsingStruct {
    
    fn new(tokens: Vec<Token>) -> ParsingStruct {
        ParsingStruct {
            tokens: tokens.clone(),
            pointer_to_tokens: -1,
            next_token: tokens.get(0).unwrap().to_owned(),
        }
    }

    fn scan_token(&mut self) {
           self.pointer_to_tokens += 1;
           self.next_token = self.tokens
               .get((self.pointer_to_tokens + 1) as usize).unwrap().to_owned();
    }
}


fn parse(tokens: Vec<Token>) -> Vec<AST> {
    /*
     * E ->  T {+|-} T
     * T -> F {* | /} F 
     * F -> ID | Integer | (E) | -F | put F
     */
    
    // ParseE
    

    let mut expr_list: Vec<Vec<Token>> = vec![];
    let mut i = 0;
    let mut token = &tokens[i];
    let mut expr: Vec<Token> = vec![];
    while token.type_ != TokenType::EOF {
        if token.type_ != TokenType::Semicolon {
            expr.push(token.clone());
        } else {
            expr_list.push(expr.clone());
            expr = vec![];
        }
        i+=1;
        token = &tokens[i];
    }

    let mut  program: Vec<AST> = vec![];
    for expr in expr_list {
        let mut toks = ParsingStruct::new(expr);
        let parsed_ast = parse_e(&mut toks);
//        parsed_ast.clone().print();print!("\n");
        program.push(parsed_ast);
    }
    return program;
}


fn parse_t(token_str: &mut ParsingStruct) -> AST {
    // println!("T: {:?}", token_str.next_token);
    let mut a = parse_f(token_str);
    loop {
        if token_str.next_token.type_ == TokenType::Mult {
            token_str.scan_token();
            let b = parse_f(token_str);
            a = AST::new(
                Literals::Operator(Operators::Mult),
                a,
                b)
        } else if token_str.next_token.type_ == TokenType::Div {
            token_str.scan_token();
            let b = parse_f(token_str);
            a = AST::new(
                Literals::Operator(Operators::Div),
                a,
                b)
        } else { 
            return a;
        }
    }
}

fn parse_e(token_str: &mut ParsingStruct) -> AST {
    // println!("E: {:?}", token_str.next_token);
    let mut a = parse_t(token_str);
    loop {
        if token_str.tokens.len() <= token_str.pointer_to_tokens as usize {
            break a;
        }
        if token_str.next_token.type_ == TokenType::Plus {
            token_str.scan_token();
            let b = parse_t(token_str);
            a = AST::new(
                Literals::Operator(Operators::Plus),
                a,
                b)
        } else if token_str.next_token.type_ == TokenType::Minus {
            token_str.scan_token();
            let b = parse_t(token_str);
            a = AST::new(
                Literals::Operator(Operators::Minus),
                a,
                b)
        } else { 
            return a;
        }
    } 
}

fn parse_f(token_str: &mut ParsingStruct) -> AST {
    // println!("F: {:?}", token_str.next_token);
    if token_str.next_token.type_ == TokenType::Put {
        token_str.scan_token(); 
        return AST::new(
            Literals::Operator(Operators::Put),
            AST::create_empty(),
            parse_f(token_str)
            );
    } else if token_str.next_token.type_ == TokenType::Integer {
        let ast = AST::new(token_str.next_token.literal, AST::create_empty(), AST::create_empty());
        if token_str.tokens.len() > (token_str.pointer_to_tokens -1) as usize {
            token_str.scan_token();
        }
        return ast;
    } else if token_str.next_token.type_ == TokenType::Minus {
        token_str.scan_token();
        return AST::new(
            Literals::Operator(Operators::Minus),
            AST::create_empty(),
            parse_f(token_str)
            );
    } else if token_str.next_token.type_ == TokenType::OpenParen {
        let open_paren_col = token_str.next_token.position.col;
        let open_paren_line = token_str.next_token.position.line;
        token_str.scan_token();
        let expr = parse_e(token_str);
        if token_str.next_token.type_ == TokenType::CloseParen {
            return expr;
        } else {
            eprintln!("last expr war type {:?}", token_str.next_token.type_);
            eprintln!("ERROR:{}:{}: `(` is never closed", open_paren_line, open_paren_col); 
            std::process::exit(1);
        }
    } else {
        eprintln!("ERROR: Unknown token type in parse_f");
        std::process::exit(1);
    }

    
}


fn main() {
    
    
    let mut args = std::env::args();
    
    if args.len() < 2 || args.len() > 3{
        eprintln!("ERROR: Usage: ./stem-rs `file`");
    }

    args.next(); // consume program name
   
    let file_path: String;
        file_path = args.next().unwrap(); 
    


    let program_string = fs::read_to_string(file_path).expect("Can't read file");
    let tokens = tokenize(program_string);
    let parsed = parse(tokens);
    

    let asm_code = generate_code(parsed.clone());

    fs::write("output.asm", asm_code).expect("Can't write the output file");


  //  for ast in parsed.clone() {
  //    run(ast);
  //  }

   // println!("{}", generate_code(parsed));
}
