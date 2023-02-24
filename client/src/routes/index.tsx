import { A } from "solid-start";
import Counter from "~/components/Counter";
import { createSignal } from "solid-js";
import { RecommendedMain } from "~/components/Recomended";
import SideBar from "~/components/SideBar";



export default function Home() {
  return (
    <main class="min-h-screen bg-primary">
      <div class="flex gap-2">
        <SideBar />
        <RecommendedMain />
      </div>
    </main>
  );
}
