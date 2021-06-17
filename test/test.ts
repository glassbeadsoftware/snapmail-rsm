// See README.md for prerequisites for this to run

//import { Orchestrator } from "@holochain/tryorama";
import { Orchestrator } from '../../tryorama/src';

// -- SETUP -- //

process.on('unhandledRejection', error => {
    // Will print "unhandledRejection err is not defined"
    console.error('got unhandledRejection:', error);
});

const orchestrator = new Orchestrator()

//require('./suites/mail')(orchestrator.registerScenario)
//require('./suites/handle')(orchestrator.registerScenario)
//require('./suites/chunk')(orchestrator.registerScenario)
//require('./suites/file_send')(orchestrator.registerScenario)
require('./suites/file_send_pending')(orchestrator.registerScenario)

//require('./suites/stress')(orchestrator.registerScenario)
//require('./suites/stress_multi')(orchestrator.registerScenario)


// -- RUN -- //

const num = orchestrator.numRegistered()
console.log(`Orchestrator Registered ${num} scenarios`)
var beginning = Date.now();
orchestrator.run().then(stats => {
    let end = Date.now();
    let elapsed = end - beginning;
    console.log(`All ${num} scenarios done. Stats:`)
    console.log(stats)
    console.log("Tests duration: " + elapsed / 1000 + ' sec')
})
