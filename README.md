# snapmail-rsm

Holochain-rsm DNA for [SnapMail](https://github.com/glassbeadsoftware/snapmail-release) from [Glass Bead Software](http://www.glassbead.com/).
To download and use it, go to [snapmail-release](https://github.com/glassbeadsoftware/snapmail-release) repo.

Native application for Windows, Linux and MacOS is available [here](https://github.com/glassbeadsoftware/snapmail-release/releases).

Some design documentation is available in the `/spec` folder.

CI and NIX configs are not set up for the moment.


## Building

To rebuild the DNA for holochain:
1. [Install rustup](https://rustup.rs/) and the `wasm32` target with: ``rustup target add wasm32-unknown-unknown``
1. Install [holochain and hc](https://github.com/holochain/holochain)
1. Run ``scripts\pack-happ.sh``


## Testing
Steps for running tests:
 1. Install Holochain
 2. Go to ``test`` sub directory.
 3. Run command: `npm test`
 
Test suites can also be enabled/disabled by commenting out the lines in `test\test.ts`


## Running with UI

 1. Download the [snapmail-ui repo](https://github.com/glassbeadsoftware/snapmail-ui) and store it at same folder level as `snapmail-rsm`
 2. CD to its root folder
 2. Make sure bootstrap server and proxy server are up and running.
 3. Launch `alex`, `billy`, or `camille` agents like this:`npm run alex`
 4. Or launch all three with `npm run all`

Browser windows should automatically pop-up for each agent.
