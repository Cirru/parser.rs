import { cirru_to_lisp, default as init } from "../pkg/cirru_parser";

let main = async () => {
  let wasm = await init();
  console.log(
    cirru_to_lisp(`
echo a
`)
  );
};

main();
