import { A } from "solid-start";
import Counter from "~/components/Counter";
import { createSignal } from "solid-js";
import { RecommendedMain } from "~/components/Recomended";



export default function Home() {
  return (
    <main class="min-h-screen bg-primary">
      <div>
        <RecommendedMain />
      </div>
    </main>
  );
}
