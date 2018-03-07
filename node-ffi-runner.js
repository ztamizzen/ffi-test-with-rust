const ffi = require("ffi");
const ref = require("ref");
const RefArray = require("ref-array");
const RefStruct = require("ref-struct");
const U32Array = RefArray(ref.types.uint32);
const Tuple = RefStruct({
    x: 'uint32',
    y: 'uint32'
});
const numbers = new U32Array([1, 2, 3, 4, 5, 6]);

const lib = ffi.Library("target/release/ffi_test", {
    addition: ['uint32', ['uint32', 'uint32']],
    chars_counter: ['uint32', ['string']],
    flip_things_around: [Tuple, [Tuple]],
    sum_of_even: ['uint32', [U32Array, 'size_t']],
    theme_song_generate: ['char *', ['uint8']],
    theme_song_free: ['void', ['char *']],
    zip_code_database_new: ['pointer', []],
    zip_code_database_free: ['void', ['pointer']],
    zip_code_database_populate: ['void', ['pointer']],
    zip_code_database_population_of: ['uint32', ['pointer', 'string']],
});
const tup = new Tuple({ x: 10, y: 20 });
const result = lib.flip_things_around(tup);

function themeSongGenerate(len) {
    const songPtr = lib.theme_song_generate(len);
    try {
        return songPtr.readCString();
    } finally {
        lib.theme_song_free(songPtr);
    }
}

console.log("123 + 123 =", lib.addition(123, 123));
console.log(lib.chars_counter("I'm a string and I'm lövely!"), "chars in string", "I'm a string and I'm lövely!");
console.log(themeSongGenerate(5));
console.log(lib.sum_of_even(numbers, numbers.length));
console.log("Tuple", tup);
console.log("(%d, %d)", result.x, result.y);

const ZipCodeDatabase = function() {
    this.ptr = lib.zip_code_database_new();
};
ZipCodeDatabase.prototype.free = function() {
    lib.zip_code_database_free(this.ptr);
};
ZipCodeDatabase.prototype.populate = function() {
    lib.zip_code_database_populate(this.ptr);
};
ZipCodeDatabase.prototype.populationOf = function(zip) {
    return lib.zip_code_database_population_of(this.ptr, zip);
};

const database = new ZipCodeDatabase();
try {
    database.populate();
    const pop1 = database.populationOf('90210');
    const pop2 = database.populationOf('12833');
    console.log("Population:", pop1 - pop2);
} finally {
    database.free();
}