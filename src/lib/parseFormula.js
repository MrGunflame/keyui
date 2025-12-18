// Remove whitespace
/** @param {string} input */
function clean(input) {
    return input.replace(/\s+/g, "");
}

// Parse atomic variable like p, q, r
/**
 * @param {string} input
 * @param {number} i
 * @returns {[string, number]}
 */
function parseAtom(input, i) {
    let start = i;
    while (i < input.length && /[a-zA-Z0-9]/.test(input[i])) {
        i++;
    }
    return [input.slice(start, i), i];
}

// Parse negation !term
/**
 * @param {string} input
 * @param {number} i
 * @returns {[any, number]}
 */
function parseNeg(input, i) {
    if (input[i] === "!") {
        let [sub, j] = parseTerm(input, i + 1);
        return [{ op: "!", args: [sub] }, j];
    }
    return parseAtom(input, i);
}

// Parse parentheses or negation or atom
/**
 * @param {string} input
 * @param {number} i
 * @returns {[any, number]}
 */
function parseTerm(input, i) {
    if (input[i] === "(") {
        let [inside, j] = parseImplication(input, i + 1);
        if (input[j] !== ")") throw "Missing ')'";
        return [inside, j + 1];
    }
    return parseNeg(input, i);
}

// Parse A -> B -> C (right associative)
/**
 * @param {string} input
 * @param {number} [i=0]
 * @returns {[any, number]}
 */
function parseImplication(input, i = 0) {
    let [left, j] = parseTerm(input, i);

    if (input.slice(j, j + 2) === "->") {
        let [right, k] = parseImplication(input, j + 2);
        left = { op: "->", args: [left, right] };
        j = k;
    }

    return [left, j];
}

// Entry point
/** @param {string} input */
export function parseFormula(input) {
    input = clean(input);
    let [ast, i] = parseImplication(input);
    if (i !== input.length) throw "Unexpected characters at end";
    return ast;
}

