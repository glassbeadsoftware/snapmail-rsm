const sjcl = require('sjcl')

// const CHUNK_MAX_SIZE = 500 * 1024;
export const CHUNK_MAX_SIZE = 100 * 1024;

export const delay = (ms) => new Promise((r) => setTimeout(r, ms));

/**
 *
 */
export function htos(wtf) {
    return Buffer.from(wtf).toString('base64')
}

/**
 *
 */
export function cellIdToStr(cell) {
    let res = '('
    res += htos(cell.cellId[0])
    res += ', '
    res += htos(cell.cellId[1])
    res += ')'
    return res
}


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
            if (mailItem.state.In.hasOwnProperty('Deleted')) {

                //if (mailItem.state.In === 'Deleted') {
                continue;
            }
        }
        if (mailItem.state.hasOwnProperty('Out')) {
            if (mailItem.state.Out.hasOwnProperty('Deleted')) {
            // if (mailItem.state.Out === 'Deleted') {
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


function padStr(str, len) {
    let result = str;
    let diff = len - str.length
    if (diff > 0) {
        let pad = ' '.repeat(diff)
        result += pad
    }
    return result
}

export function logDump(name, dump) {
    console.log(' ====== ' + name + ' - SOURCE-CHAIN STATE DUMP START ===== ' + dump.length)
    //console.log({dump})
    let peer_dump = dump[0].peer_dump;
    let source_chain_dump = dump[0].source_chain_dump;
    let integration_dump = dump[0].integration_dump;
    //console.log({peer_dump})
    //console.log({source_chain_dump})
    //console.log({integration_dump})
    const chain_len = source_chain_dump.elements.length
    for(let i = 0; i < chain_len; i++) {
        let element = source_chain_dump.elements[i]
        //console.log({element})
        let str = ' ' + (chain_len - i) + '. ' + element.header.type
        if (element.header.type === 'CreateLink') {
            str += ' "' + Buffer.from(element.header.tag).toString('utf-8') + '"'
        } else {
            if (element.header.entry_type) {
                if (typeof element.header.entry_type === 'object') {
                    str += ' - AppEntry ; id = ' + element.header.entry_type.App.id
                } else {
                    str += ' - ' + element.header.entry_type
                }
            }
        }
        str = padStr(str, 40)
        let hh = htos(element.header_address)
        str += ' (' + hh + ')'
        console.log(str)
    }
    console.log(' ====== ' + name + ' - SOURCE-CHAIN STATE DUMP END   =====')
}
