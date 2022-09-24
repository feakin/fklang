import {FklParser} from "@feakin/fkl-wasm-web";

let result = new FklParser(`ContextMap {
  SalesContext <-> SalesContext;
}`).parse();

console.log(result);
