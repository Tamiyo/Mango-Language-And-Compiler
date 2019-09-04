#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

#include "lexer.h"
#include "parser.h"

#include "common.h"

using std::string;
using std::vector;
using std::pair;
using std::cout;
using std::endl;

int SCOPE_LEVEL = 1;
int SCOPE_DEPTH_LIMIT = 256;
map<int, map<string, Node *>> SCOPED_SYMBOL_TABLE = {};

// TODO - Tooling broken? Check it out before compiling again!!!!!
int main() {
    string input = "@function: x, y { z = x * 3\nreturn z * y }\nmyvariable = function(2, 3)$";
    Lexer lexer = {input};
    vector<LexerResult> tokens = lexer.lex();

    Parser parser = Parser(tokens);
    NodeMango *root = dynamic_cast<NodeMango *>(parser.parse());
    root->eval();

    cout << "Scoped Table at \"Level 1\" Items..." << endl;
    std::for_each(SCOPED_SYMBOL_TABLE[1].begin(), SCOPED_SYMBOL_TABLE[1].end(), [](const pair<string, Node *> &p) {
        cout << p.first << ": ";
        if (auto *pItem = dynamic_cast<NodeTerm *>(p.second)) {
            cout << pItem->token << endl;
        } else {
            cout << endl;
        }
    });

}