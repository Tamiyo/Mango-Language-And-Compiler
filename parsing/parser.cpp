//
// Created by zoolo on 3/2/2020.
//

#include "parser.h"

Mango1 *parser::parse() {
    stack<int> state_stack;
    state_stack.push(0);

    stack<Node *> value_stack;

    ltokens.reverse();

    while (true) {

        int state = state_stack.peek();
        lexer_token ltoken = ltokens.peek();
        int decision;

        cout << "STACK: " << state_stack.print_state() << endl;
        cout << "ACTION(" << state << ", " << grammar::token_map[ltoken.tok] << ")" << endl;

        if (taction.count({state, ltoken.tok}) > 0) decision = taction[{state, ltoken.tok}];
        else {
            cout << "Parse Table Key Error" << endl;
            return nullptr;
        }

        if (decision > 0) {
            cout << "\tSHIFT " << decision << endl;
            if (ltoken.tok == token::literal) {
                auto *node = new Literal{ltoken.val};
                value_stack.push(dynamic_cast<Node *>(node));
            }

            ltokens.pop();
            state_stack.push(decision);
        } else if (decision < 0) {
            decision *= -1;
            cout << "\tREDUCE " << decision << endl;
            if (indexed_grammar.count(decision) > 0) {
                tuple<int, token, vector<token>> result = indexed_grammar[decision];

                int length = get<0>(result);
                token A = get<1>(result);

                while (length-- > 0) {
                    if (state_stack.empty()) {
                        cout << "Reduce Error" << endl;
                        return nullptr;
                    } else state_stack.pop();
                }

                cout << "GOTO(" << state_stack.peek() << ", " << grammar::token_map[A] << ")" << endl;

                if (tgoto.count({state_stack.peek(), A}) > 0) {
                    state_stack.push(tgoto[{state_stack.peek(), A}]);
                    reduce(decision, &value_stack);
                } else {
                    cout << "GOTO Error" << endl;
                    return nullptr;
                }
            } else {
                cout << "Reduce Index Error" << endl;
                return nullptr;
            }

        } else if (decision == 0) {
            cout << "ACCEPT!" << endl;
            reduce(1, &value_stack);
            return dynamic_cast<Mango1 *>(value_stack.peek());
        } else {
            cout << "Parsing Failed" << endl;
            return nullptr;
        }

        cout << endl;
    }
}
