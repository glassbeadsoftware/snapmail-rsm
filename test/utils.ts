const sjcl = require('sjcl')

// const CHUNK_MAX_SIZE = 500 * 1024;
export const CHUNK_MAX_SIZE = 100 * 1024;

export const delay = (ms) => new Promise((r) => setTimeout(r, ms));

/**
 *
 */
export function sleep(milliseconds) {
    const date = Date.now();
    let currentDate = 0;
    do {
        currentDate = Date.now();
    } while (currentDate - date < milliseconds);
}

/**
 * Removed deleted mails from input mail list
 */
export function filterMailList(mail_list) {
    let new_list = new Array();
    for (let mailItem of mail_list) {
        if (mailItem.state.hasOwnProperty('In')) {
            if (mailItem.state.In === 'Deleted') {
                continue;
            }
        }
        if (mailItem.state.hasOwnProperty('Out')) {
            if (mailItem.state.Out === 'Deleted') {
                continue;
            }
        }
        new_list.push(mailItem);
    }
    return new_list;
}

export function sha256(message) {
    //console.log('message: ' + message)
    const myBitArray = sjcl.hash.sha256.hash(message)
    //console.log('myBitArray: ' + JSON.stringify(myBitArray))
    const hashHex = sjcl.codec.hex.fromBits(myBitArray)
    //console.log('hashHex: ' + hashHex)
    return hashHex;
}

export function chunkSubstr(str, size) {
    var numChunks = Math.ceil(str.length / size);
    var chunks = new Array(numChunks);

    for(var i = 0, o = 0; i < numChunks; ++i, o += size) {
        chunks[i] = str.substr(o, size);
    }

    return chunks;
}

export function split_file(full_data_string) {
    const hash = sha256(full_data_string);
    console.log('file hash: ' + hash)
    const chunks = chunkSubstr(full_data_string, CHUNK_MAX_SIZE);

    return {
        dataHash: hash,
        numChunks: chunks.length,
        chunks: chunks,
    }
}
