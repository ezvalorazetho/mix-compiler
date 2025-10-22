#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Location {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub enum Node {
    Int {
        value: String,
        lbit: bool,
        loc: Location,
    },
    Float {
        value: String,
        lbit: bool,
        loc: Location,
    },
    Bool {
        value: String,
        loc: Location,
    },
    Null {
        value: String,
        loc: Location,
    },
    Str {
        value: String,
        loc: Location,
    },
    Var {
        value: String,
        loc: Location,
    },
    List {
        element: Vec<Box<Node>>,
        loc: Location,
    },
    Dict {
        key_value: Vec<Box<(Node, Node)>>,
        loc: Location,
    },
    DictType {
        dictname: Box<Node>,
        dictype: Box<(Node, Node)>,
        loc: Location,
    },
    ListType {
        listname: Box<Node>,
        listtype: Box<Node>,
        loc: Location,
    },
    MemLockup {
        targ: Box<Node>,
        obj: Box<Node>,
        loc: Location,
    },
    CodeLockup {
        targ: Box<Node>,
        obj: Box<Node>,
        loc: Location,
    },
    FuncDef {
        name: String,
        public: bool,
        args: Vec<Box<Node>>,
        rtype: Box<Node>,
        body: Vec<Box<Node>>,
        loc: Location,
    },
    Struct {
        name: String,
        public: bool,
        child: Vec<Box<Node>>,
        body: Vec<Box<Node>>,
        loc: Location,
    },
    Attach {
        name: String,
        attach_to: Box<Node>,
        public: bool,
        args: Vec<Box<Node>>,
        rtype: Box<Node>,
        body: Vec<Box<Node>>,
        loc: Location,
    },
    Enum {
        name: String,
        public: bool,
        child: Vec<Box<Node>>,
        loc: Location,
    },
    Return {
        value: Box<Node>,
        loc: Location,
    },
    Tuple {
        element: Vec<Box<Node>>,
        loc: Location,
    },
    LetDef {
        name: String,
        dtype: Box<Node>,
        public: bool,
        value: Box<Node>,
        loc: Location,
    },
    AssignDef {
        targ: Box<Node>,
        value: Box<Node>,
        opr: String,
        loc: Location,
    },
    BinaryOp {
        lhs: Box<Node>,
        rhs: Box<Node>,
        opr: String,
        loc: Location,
    },
    UnaryOp {
        opr: String,
        value: Box<Node>,
        loc: Location,
    },
    Conditional {
        cond: Box<Node>,
        body_if: Vec<Box<Node>>,
        body_else: Vec<Box<Node>>,
        loc: Location,
    },
    ForLoop {
        initializer: Box<Node>,
        iterator: Box<Node>,
        body: Vec<Box<Node>>,
        loc: Location,
    },
    WhileLoop {
        cond: Box<Node>,
        body: Vec<Box<Node>>,
        loc: Location,
    },
    MatchCase {
        value: Box<Node>,
        child: Vec<Box<(Node, Vec<Box<Node>>)>>,
        default: Vec<Box<Node>>,
        loc: Location,
    },
    Break,
    // Default,
    Continue,
    Range {
        min: Box<Node>,
        max: Box<Node>,
        loc: Location,
    },
    Import {
        package: Vec<Box<Node>>,
        module: Vec<Box<Node>>,
        loc: Location,
    },
    Alias {
        name: String,
        real: Box<Node>,
        loc: Location,
    },
    FuncCall {
        func: Box<Node>,
        args: Vec<Box<Node>>,
        loc: Location,
    },
    Void,
}
