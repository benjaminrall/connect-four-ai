let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_3.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_export_3.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_3.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}
/**
 * Sets up a hook to log Rust panics to the browser's console when the
 * WASM module is first loaded.
 */
export function start() {
    wasm.start();
}

const AIPlayerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_aiplayer_free(ptr >>> 0, 1));
/**
 * An AI player that uses a solver to determine the best move to play in a Connect Four position.
 *
 * The player's skill level can be configured using the `Difficulty` enum, which adjusts the
 * move selection strategy.
 */
export class AIPlayer {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        AIPlayerFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_aiplayer_free(ptr, 0);
    }
    /**
     * Creates a new AI player with a default solver and specified difficulty.
     * @param {Difficulty} difficulty
     */
    constructor(difficulty) {
        _assertClass(difficulty, Difficulty);
        var ptr0 = difficulty.__destroy_into_raw();
        const ret = wasm.aiplayer_new(ptr0);
        this.__wbg_ptr = ret >>> 0;
        AIPlayerFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Attempts to load an opening book from the given path for the AI player's solver.
     *
     * Returns whether the opening book was successfully loaded.
     * @param {string} path
     * @returns {boolean}
     */
    loadOpeningBook(path) {
        const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.aiplayer_loadOpeningBook(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * Resets the AI player's solver.
     */
    reset() {
        wasm.aiplayer_reset(this.__wbg_ptr);
    }
    /**
     * Solves a position to find its exact score using the AI player's solver.
     * @param {Position} position
     * @returns {number}
     */
    solve(position) {
        _assertClass(position, Position);
        const ret = wasm.aiplayer_solve(this.__wbg_ptr, position.__wbg_ptr);
        return ret;
    }
    /**
     * Calculates the scores for all possible next moves in the given position using the
     * AI player's solver.
     * @param {Position} position
     * @returns {any[]}
     */
    getAllMoveScores(position) {
        _assertClass(position, Position);
        const ret = wasm.aiplayer_getAllMoveScores(this.__wbg_ptr, position.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * Solves and selects the AI player's move for the given position.
     * @param {Position} position
     * @returns {number | undefined}
     */
    getMove(position) {
        _assertClass(position, Position);
        const ret = wasm.aiplayer_getMove(this.__wbg_ptr, position.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * Selects a move from an array of scores using a Softmax distribution with a
     * temperature defined by the AI player's difficulty. Temperature values <= 0 will
     * result in greedy selection (always picking the best move).
     *
     * Returns the column index of the selected move or `None` if no moves are possible.
     * @param {Position} position
     * @param {any[]} scores
     * @returns {number | undefined}
     */
    selectMove(position, scores) {
        _assertClass(position, Position);
        const ptr0 = passArrayJsValueToWasm0(scores, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.aiplayer_selectMove(this.__wbg_ptr, position.__wbg_ptr, ptr0, len0);
        return ret === 0x100000001 ? undefined : ret;
    }
}

const DifficultyFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_difficulty_free(ptr >>> 0, 1));
/**
 * An enum to represent the difficulty of an AI player.
 */
export class Difficulty {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Difficulty.prototype);
        obj.__wbg_ptr = ptr;
        DifficultyFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DifficultyFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_difficulty_free(ptr, 0);
    }
    /**
     * @returns {Difficulty}
     */
    static get EASY() {
        const ret = wasm.difficulty_EASY();
        return Difficulty.__wrap(ret);
    }
    /**
     * @returns {Difficulty}
     */
    static get MEDIUM() {
        const ret = wasm.difficulty_MEDIUM();
        return Difficulty.__wrap(ret);
    }
    /**
     * @returns {Difficulty}
     */
    static get HARD() {
        const ret = wasm.difficulty_HARD();
        return Difficulty.__wrap(ret);
    }
    /**
     * @returns {Difficulty}
     */
    static get IMPOSSIBLE() {
        const ret = wasm.difficulty_IMPOSSIBLE();
        return Difficulty.__wrap(ret);
    }
}

const PositionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_position_free(ptr >>> 0, 1));
/**
 * Represents a Connect Four position compactly as a bitboard.
 *
 * The standard, 6x7 Connect Four board can be represented unambiguously using 49 bits
 * in the following bit order:
 *
 * ```comment
 *   6 13 20 27 34 41 48
 *  ---------------------
 * | 5 12 19 26 33 40 47 |
 * | 4 11 18 25 32 39 46 |
 * | 3 10 17 24 31 38 45 |
 * | 2  9 16 23 30 37 44 |
 * | 1  8 15 22 29 36 43 |
 * | 0  7 14 21 28 35 42 |
 *  ---------------------
 * ```
 *
 * The extra row of bits at the top identifies full columns and prevents
 * bits from overflowing into the next column. For computational
 * efficiency, positions are stored using two 64-bit unsigned integers:
 * one storing a mask of all occupied tiles, and the other storing a mask
 * of the current player's tiles.
 */
export class Position {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Position.prototype);
        obj.__wbg_ptr = ptr;
        PositionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PositionFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_position_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    static get WIDTH() {
        const ret = wasm.position_WIDTH();
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    static get HEIGHT() {
        const ret = wasm.position_HEIGHT();
        return ret >>> 0;
    }
    /**
     * Creates a new position instance for the initial state of the game.
     */
    constructor() {
        const ret = wasm.position_new();
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        PositionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Parses a position from a string of 1-indexed moves.
     *
     * The input string should contain a sequence of columns played, indexed from 1.
     * @param {string} moves
     * @returns {Position}
     */
    static fromMoves(moves) {
        const ptr0 = passStringToWasm0(moves, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.position_fromMoves(ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Position.__wrap(ret[0]);
    }
    /**
     * Parses a position from a string representation of the Connect Four board.
     *
     * The input string should contain exactly 42 characters from the set `['.', 'o', 'x']`,
     * representing the board row by row from the top-left to the bottom-right. All other
     * characters are ignored. 'x' is treated as the current player, and 'o' as the opponent.
     * This method assumes that a correctly formatted board string is a valid game position.
     * Invalid game positions will lead to undefined behaviour.
     * @param {string} board_string
     * @returns {Position}
     */
    static fromBoardString(board_string) {
        const ptr0 = passStringToWasm0(board_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.position_fromBoardString(ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Position.__wrap(ret[0]);
    }
    /**
     * Returns the Position's attributes formatted as a string.
     * @returns {string}
     */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.position_toString(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * A bitmask of the current player's tiles.
     * @returns {bigint}
     */
    get position() {
        const ret = wasm.position_position(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * A bitmask of all occupied tiles.
     * @returns {bigint}
     */
    get mask() {
        const ret = wasm.position_mask(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Returns the unique key for the current position.
     *
     * This key is unique to each pair of horizontally symmetrical positions, as these
     * positions will always have the same solution.
     * @returns {bigint}
     */
    getKey() {
        const ret = wasm.position_getKey(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Returns the number of moves played to reach the current position.
     * @returns {number}
     */
    getMoves() {
        const ret = wasm.position_getMoves(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Indicates whether a given column is playable.
     *
     * # Arguments
     *
     * * `col`: 0-based index of a column.
     *
     * # Returns
     *
     * True if the column is playable, false if the column is already full.
     * @param {number} col
     * @returns {boolean}
     */
    isPlayable(col) {
        const ret = wasm.position_isPlayable(this.__wbg_ptr, col);
        return ret !== 0;
    }
    /**
     * Indicates whether the current player wins by playing a given column.
     *
     * # Arguments
     *
     * * `col`: 0-based index of a playable column.
     *
     * # Returns
     *
     * True if the current player makes a 4-alignment by playing the column, false otherwise.
     * @param {number} col
     * @returns {boolean}
     */
    isWinningMove(col) {
        const ret = wasm.position_isWinningMove(this.__wbg_ptr, col);
        return ret !== 0;
    }
    /**
     * Indicates whether the current player can win with their next move.
     * @returns {boolean}
     */
    canWinNext() {
        const ret = wasm.position_canWinNext(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Plays a move in the given column.
     *
     * # Arguments
     *
     * * `col`: 0-based index of a playable column.
     * @param {number} col
     */
    play(col) {
        wasm.position_play(this.__wbg_ptr, col);
    }
    /**
     * Returns a mask for the possible moves the current player can make.
     * @returns {bigint}
     */
    possible() {
        const ret = wasm.position_possible(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Returns a mask for the possible non-losing moves the current player can make.
     * @returns {bigint}
     */
    possibleNonLosingMoves() {
        const ret = wasm.position_possibleNonLosingMoves(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Indicates whether the current position has been won by either player.
     * @returns {boolean}
     */
    isWonPosition() {
        const ret = wasm.position_isWonPosition(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Returns a mask for the entirety of the given column.
     *
     * # Arguments
     *
     * * `col`: 0-based index of a column.
     *
     * # Returns
     *
     * A bitmask with a one in all cells of the column.
     * @param {number} col
     * @returns {bigint}
     */
    static columnMask(col) {
        const ret = wasm.position_columnMask(col);
        return BigInt.asUintN(64, ret);
    }
}

const SolverFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_solver_free(ptr >>> 0, 1));
/**
 * A strong solver for finding the exact score of Connect Four positions.
 *
 * This class implements a high-performance negamax search algorithm with several
 * optimisations, including:
 * - Alpha-beta pruning
 * - Score-based move ordering to prioritise stronger moves
 * - A transposition table to cache results of previously seen positions
 * - A binary search on the score for faster convergence
 */
export class Solver {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SolverFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_solver_free(ptr, 0);
    }
    /**
     * Creates a new `Solver` instance, using the pre-packaged opening book.
     */
    constructor() {
        const ret = wasm.solver_new();
        this.__wbg_ptr = ret >>> 0;
        SolverFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * A counter for the number of nodes explored in the last `solve` call.
     * @returns {number}
     */
    get explored_positions() {
        const ret = wasm.solver_explored_positions(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Attempts to load an opening book from the given path.
     *
     * Returns whether the opening book was successfully loaded.
     * @param {string} path
     * @returns {boolean}
     */
    loadOpeningBook(path) {
        const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.solver_loadOpeningBook(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
     * Resets the solver's state.
     */
    reset() {
        wasm.solver_reset(this.__wbg_ptr);
    }
    /**
     * Solves a position to find its exact score.
     *
     * This function uses a binary search over the possible score range, repeatedly calling the
     * negamax search with a null window to test if the score is above a certain value. This
     * allows faster convergence to the true score.
     *
     * Assumes that the given position is valid and not won by either player.
     *
     * # Arguments
     *
     * * `position`: The board position to solve.
     *
     * # Returns
     * The exact score of the position, which reflects the outcome of the game assuming that both
     * players play perfectly. A position has:
     * - A positive score if the current player will win. 1 if they win with their last move, 2 if
     *   they win with their second to last move, ...
     * - A null score if the game will end in a draw
     * - A negative score if the current player will lose. -1 if the opponent wins with their last
     *   move, -2 if the opponent wins with their second to last move, ...
     * @param {Position} position
     * @returns {number}
     */
    solve(position) {
        _assertClass(position, Position);
        const ret = wasm.solver_solve(this.__wbg_ptr, position.__wbg_ptr);
        return ret;
    }
    /**
     * Calculates the scores for all possible next moves in the given position.
     *
     * Returns a fixed-size array where each index corresponds to a column, containing
     * a score if a move in that column is possible and `None` if the column is full and
     * the move is impossible.
     *
     * This array can be used to directly calculate the optimal move to play in a position.
     * @param {Position} position
     * @returns {any[]}
     */
    getAllMoveScores(position) {
        _assertClass(position, Position);
        const ret = wasm.solver_getAllMoveScores(this.__wbg_ptr, position.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_getRandomValues_3c9c0d586e575a16 = function() { return handleError(function (arg0, arg1) {
        globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
    }, arguments) };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_3;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };
    imports.wbg.__wbindgen_is_null = function(arg0) {
        const ret = arg0 === null;
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = arg0 === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = arg1;
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('connect_four_ai_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
