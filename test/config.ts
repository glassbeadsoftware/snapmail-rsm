import { Config } from '../../tryorama/src';
//import { Config } from "@holochain/tryorama";

const ALEX_NICK = 'alice'
const BILLY_NICK = 'billy'
const CAMILLE_NICK = 'camille'

const config = Config.gen({
    //tester: testDna,
    [ALEX_NICK]: Config.dna("../snapmail.dna.gz", null),
    [BILLY_NICK]: Config.dna("../snapmail.dna.gz", null),
    [CAMILLE_NICK]: Config.dna("../snapmail.dna.gz", null),
})

module.exports = {
    config,
    ALEX_NICK,
    BILLY_NICK,
    CAMILLE_NICK
}
