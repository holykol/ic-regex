import { HttpAgent } from "@dfinity/agent";
import { createActor, canisterId } from "../declarations/regex/index";
import * as b from 'benny'

const regex = createActor(canisterId, {
   agent: new HttpAgent({
      host: "http://127.0.0.1:4943",
   })
})

const rx = "(?:-u(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]))"

async function precompile() {
   await regex.precompile([rx]);
}

async function captures() {
   await regex.batch_is_match(rx, [].concat(...new Array(1).fill(ips)));
}

async function purge() {
   await regex.purge_cache()
}


const cfg = {
   minTime: 1,
}

b.suite(
   'Regex',

   b.add('With precompile', async () => {
      await purge();
      await precompile();
      console.log("precompile called")

      return async () => {
         await captures();
      }
   }, cfg),


   b.add('Without precompile', async () => {
      await purge();

      return async () => {
         await captures();
      }
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
   "2beb:bd92:ac9c:72cd:9433:de0c:d97b:7127",
   "244.213.165.32",
   "bac4:eaf5:d390:d0f1:192b:2535:b535:a045",
   "119.207.217.147",
   "194.172.135.180",
   "2fc9:7186:eb54:3f13:941:2ddc:587b:c8ec",
   "e613:6c99:a028:5162:f03b:def2:8940:9875",
   "b8bc:40b7:e359:d7e3:ca33:c85f:7992:6042",
   "afde:20ad:d0df:6abb:eee4:e30d:e2b3:49ab",
   "94.217.109.123",
   "140.141.160.137",
   "146.206.57.199",
   "dfda:5d8e:44d8:1103:e79f:3d41:dac9:2883",
   "177.122.2.10",
   "f0b0:e2f2:c798:1393:adc9:653a:8951:b906",
   "10.120.36.121",
   "103.182.139.231",
   "b59a:52e1:fe2b:4b2f:f36e:e641:c03f:94b",
   "b68c:f5de:d8ba:1a61:69dc:94e6:77d4:8251",
   "80.145.120.92",
   "e6ba:c767:35cb:9c56:f4dd:4f12:c20:46b8",
   "f7b2:9e0f:2caf:3848:3b16:ba43:42e5:b0f3",
   "242.23.67.83",
   "78c7:a8cd:fc72:9016:3a68:c69c:6543:993d",
   "72.55.166.127",
];
