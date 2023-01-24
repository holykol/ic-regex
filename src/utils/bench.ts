import { HttpAgent } from "@dfinity/agent";
import { createActor, canisterId } from "../declarations/regex/index";
import * as b from 'benny'

const regex = createActor(canisterId, {
   agent: new HttpAgent({
      host: "http://127.0.0.1:4943",
   })
})

//const rx = "(?:-u(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]))"
const rx = String.raw`([-!#-'*+/-9=?A-Z^-~]+(\.[-!#-'*+/-9=?A-Z^-~]+)*|"([]!#-[^-~ \t]|(\\[\t -~]))+")@[0-9A-Za-z]([0-9A-Za-z-]{0,61}[0-9A-Za-z])?(\.[0-9A-Za-z]([0-9A-Za-z-]{0,61}[0-9A-Za-z])?)+`

async function precompile() {
   await regex.precompile([rx]);
}

async function captures() {
   await regex.is_match_batch(rx, [].concat(...new Array(2).fill(ips)));
}

async function purge() {
   await regex.purge_cache()
}

const cfg = {
   minTime: 2,
}

b.suite(
   'Regex',

   b.add('Without precompile', async () => {
      await purge();

      return async () => {
         await captures();
      }
   }, cfg),

   b.add('With precompile', async () => {
      await purge();
      await precompile();
      return captures
   }, cfg),

   b.cycle(),
   b.complete(),
   b.save({ file: 'regex', format: 'chart.html' }),
)

var ips = [
   "cafe:b6e7:399f:a887:94d9:40a2:db23:c716",
   "188.75.228.136",
   "99.250.201.140",
   "90.193.200.132",
   "124.73.252.43",
   "228.246.63.20",
   "c506:ee4d:f7bd:d5d2:5b3c:8cab:d33d:fe12",
   "5cbc:75ae:ee4a:97b2:128:7ee1:c3f9:3d61",
   "d4fb:f2b6:c1d2:6fe9:2768:451f:e654:495a",
   "238.79.59.109",
   "133.22.99.26",
   "efb1:c5b3:692d:f5fd:e450:5679:7af:1062",
   "e250:c560:25e6:9308:f250:cd40:89e3:7acb",
   "4405:d466:c242:109b:ea7f:891c:1d38:226e",
   "131.225.160.247",
   "sfoskett@msn.com",
   "pakaste@aol.com",
   "reziac@sbcglobal.net",
   "ideguy@icloud.com",
   "duchamp@yahoo.com",
   "tarreau@mac.com",
   "scottzed@live.com",
   "lcheng@sbcglobal.net",
   "wayward@sbcglobal.net",
   "tangsh@verizon.net",
   "tristan@icloud.com",
   "keutzer@icloud.com",
];
