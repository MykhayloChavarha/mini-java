// steps for lexical analysis. 
//

enum DfaState {
    Start, 
    Ident, 
    Integer,
    And, 
    AndAnd, 
    Error
}