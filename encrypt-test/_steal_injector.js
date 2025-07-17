class CustomTextEncoder extends TextEncoder {
    encode(string) {
        console.log(string);
        return super.encode(string);
    }
}

Object.assign(CustomTextEncoder, TextEncoder);
TextEncoder = CustomTextEncoder;

class CustomTextDecoder extends TextDecoder {
    decode(string) {
        var decoded = super.decode(string);
        console.log(decoded);
        return decoded;
    }
}

Object.assign(CustomTextDecoder, TextDecoder);
var _old_TextDecoder = TextDecoder;
TextDecoder = CustomTextDecoder;

var old_encrypt = crypto.subtle.encrypt;
crypto.subtle.encrypt = function(...arguments) {
    console.log(arguments)
    var ret = old_encrypt.apply(this, arguments);
    return ret;
};

var old_digest = crypto.subtle.digest;
crypto.subtle.digest = async function(...arguments) {
    var ret = await old_digest.apply(this, arguments);

    console.log('---- Crypto Digest Hook ----');
    console.log('  Hash Type: ' + arguments[0]);
    console.log('  Source text: ' + new _old_TextDecoder().decode(arguments[1]));
    const hashArray = Array.from(new Uint8Array(ret));
    const result = hashArray.map((bytes) => bytes.toString(16).padStart(2, '0')).join('');
    console.log('  Result text: ' + result);
    console.log('----------------------------');

    return ret;
};

var old_import_key = crypto.subtle.importKey;
crypto.subtle.importKey = function(...arguments) {
    console.log(arguments)
    var ret = old_import_key.apply(this, arguments);
    return ret;
};

var old_btoa = window.btoa;
window.btoa = function(...arguments) {
    var ret = old_btoa.apply(this, arguments);
    console.log(ret);
    return ret;
};

var old_random = Math.random;
Math.random = function(...arguments) {
    var ret = old_random.apply(this, arguments);
    console.log(ret);
    return ret;
};

var old_fetch = window.fetch;
window.fetch = function(...arguments) {
    console.log(arguments);
    return old_fetch.apply(this, arguments);
};
