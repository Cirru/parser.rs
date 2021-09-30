import { cirru_to_lisp, default as init } from "../pkg/wasm_example";

let main = async () => {
  let wasm = await init();
  console.log(
    cirru_to_lisp(`
echo a
`)
  );
};

main();
