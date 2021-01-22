import {setup_1_conductor, setup_2_conductors, setup_3_conductors, ALEX_NICK, BILLY_NICK, CAMILLE_NICK, setup_conductor_3p} from '../config';
import { delay, htos } from '../utils';

const { split_file } = require('../utils')


// -- Export scenarios -- //

module.exports = scenario => {
    scenario("test write one chunk", test_write_one_chunk)
    //scenario("test write/get multi-chunk file with manifest", test_write_multichunk_file)
    //scenario("test write too big chunk", test_write_too_big_chunk)
}

// -- Scenarios -- //

/**
 *
 */
const test_write_one_chunk =  async (s, t) => {
    console.log('test_write_one_chunk START')
    const { alex,  alexAddress, alexCell  } = await setup_1_conductor(s, t)
    //const {alex} = await s.players({alex: conductorConfig}, true)
    const fileChunks = split_file("alex")
    console.log('fileChunks: ' + JSON.stringify(fileChunks))
    const params = {
        data_hash: fileChunks.dataHash,
        chunk_index: 0,
        chunk: fileChunks.chunks[0],
    }
    console.log('Writing chunk: ' + JSON.stringify(params))
    const file_address = await alexCell.call("snapmail", "write_chunk", params)
    console.log('file_address: ' + JSON.stringify(file_address))
    //t.match(file_address.Ok, RegExp('Qm*'))

    const chunk_address = file_address.Ok
    const result = await alexCell.call("snapmail", "get_chunk", chunk_address)
    console.log('result: ' + JSON.stringify(result))
    t.deepEqual(result.Ok, fileChunks.chunks[0])
};


/**
 *

const test_write_multichunk_file = async (s, t) => {
    const {alex} = await s.players({alex: conductorConfig}, true)
    // -- Create huge file as string
    // const data_string = create_huge_string(18)
    //const data_string = "x".repeat(250)
    const data_string = "123465789".repeat(1 * 1024 * 1024 / 10)
    // const data_string = "0123465789ABCDEF";
    // const data_string = "0123465789ABCDEFGHIJKLMNOPQRS";
    console.log('data_string_size : ' + data_string.length)

    // split file
    const fileChunks = split_file(data_string)
    // console.log('fileChunks: ' + JSON.stringify(fileChunks))

    // Write chunks
    var chunk_list = [];
    for (var i = 0; i < fileChunks.numChunks; ++i) {
        //console.log('chunk' + i + ': ' + fileChunks.chunks[i])
        const chunk_params = {
            data_hash: fileChunks.dataHash,
            chunk_index: i,
            chunk: fileChunks.chunks[i],
        }
        const chunk_address = await alex.call("app", "snapmail", "write_chunk", chunk_params)
        console.log('chunk_address' + i + ': ' + JSON.stringify(chunk_address))
        t.match(chunk_address.Ok, RegExp('Qm*'))
        chunk_list.push(chunk_address.Ok)
    }
    chunk_list = chunk_list.reverse();

    // Write manifest
    const manifest_params = {
        data_hash: fileChunks.dataHash,
        filename: "fake.str",
        filetype: "str",
        orig_filesize: data_string.length,
        chunks: chunk_list,
    }
    const manifest_address = await alex.call("app", "snapmail", "write_manifest", manifest_params)
    console.log('manifest_address' + i + ': ' + JSON.stringify(manifest_address))
    t.match(manifest_address.Ok, RegExp('Qm*'))

    // Get chunk list via manifest
    const get_manifest_params = {manifest_address: manifest_address.Ok}
    const resultGet = await alex.call("app", "snapmail", "get_manifest", get_manifest_params)
    console.log('get_manifest_result' + i + ': ' + JSON.stringify(resultGet))
    t.deepEqual(resultGet.Ok.orig_filesize, data_string.length)
    chunk_list = resultGet.Ok.chunks;

    // Get chunks
    let result_string = ''
    for (var i = chunk_list.length - 1; i >= 0; --i) {
        // await s.consistency()
        // sleep(10000)
        const params2 = {chunk_address: chunk_list[i]}
        const result = await alex.call("app", "snapmail", "get_chunk", params2)
        // console.log('get_result' + i + ': ' + JSON.stringify(result))
        result_string += result.Ok
    }
    t.deepEqual(data_string, result_string)
};
*/

/**
 *
 *
const test_write_too_big_chunk =  async (s, t) => {
    const {alex} = await s.players({alex: conductorConfig}, true)
    const data_string = "0123465789".repeat(250 * 1024 / 10)
    const fileChunks = split_file(data_string)
    console.log('fileChunks: ' + JSON.stringify(fileChunks))
    const params = {
        data_hash: fileChunks.dataHash,
        chunk_index: 0,
        chunk: data_string,
    }
    const file_address = await alex.call("app", "snapmail", "write_chunk", params)
    console.log('file_address: ' + JSON.stringify(file_address))
    let file_err = JSON.parse(file_address.Err.Internal);
    t.match(JSON.stringify(file_err.kind), RegExp('ValidationFailed*'))
};
*/
