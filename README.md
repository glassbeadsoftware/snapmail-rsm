# snapmail-rsm

Holochain DNA for [SnapMail](https://github.com/glassbeadsoftware/snapmail) from [Glass Bead Software](http://www.glassbead.com/).
To download and use it, go to [snapmail](https://github.com/glassbeadsoftware/snapmail) repo.

Native application for Windows, Linux and MacOS is available [here](https://github.com/glassbeadsoftware/snapmail/releases).

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
