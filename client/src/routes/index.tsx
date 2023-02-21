import { A } from "solid-start";
import Counter from "~/components/Counter";
import { createSignal } from "solid-js";



export default function Home() {
  return (
    <main class="">
      <div>
        <Counter />
      </div>
    </main>
  );
}
