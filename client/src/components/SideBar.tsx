import { HomeIcon } from "~/icons/HomeIcon";

const SideBar = () => {
  return (<div class="min-h-screen bg-secondary">
    <div class="p-2 flex items-center justify-center">
      <a href="/" class={"flex flex-col text-white items-center justify-center"}><HomeIcon width={30} height={30} color="white" /><p>Home</p></a>
    </div>
  </div>)
}

export default SideBar;
